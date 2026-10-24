//! 语音通话的「大脑」（0030）：voice-live 负责听和说，台词由这里出。
//!
//! 一通电话开一个常驻的 claude 进程（stream-json 进出，不给任何工具），这样每句话不用冷启动；
//! 引擎选了 DeepSeek 时同一个进程指到 DeepSeek。系统提示 = 角色的 soul.md + 这通电话开始时的口袋摘要，由页面拼好传进来。
//! 不落盘：不留会话（--no-session-persistence），台词只经事件交给页面。

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};

struct Brain {
    child: Child,
    stdin: ChildStdin,
    call: u64,
}

static BRAIN: Mutex<Option<Brain>> = Mutex::new(None);

#[derive(Clone, serde::Serialize)]
struct VoiceEv {
    call: u64,
    kind: &'static str, // delta | idle | error | exit
    text: String,
}

fn stop_locked(slot: &mut Option<Brain>) {
    if let Some(mut b) = slot.take() {
        let _ = b.child.kill();
        let _ = b.child.wait();
    }
}

#[tauri::command]
pub fn voice_start(app: AppHandle, call: u64, model: String, system: String) -> Result<(), String> {
    let bin = super::claude_bin().ok_or("没找到 claude 命令")?;
    let work = super::data_dir().join("agent");
    let _ = std::fs::create_dir_all(&work);
    let mut cmd = Command::new(bin);
    cmd.current_dir(&work)
        .args(["-p", "--input-format", "stream-json", "--output-format", "stream-json", "--include-partial-messages", "--verbose"])
        .args(["--no-session-persistence", "--strict-mcp-config", "--tools", ""])
        .arg("--system-prompt")
        .arg(&system);
    if model == "deepseek" {
        let key = super::deepseek_key().ok_or("引擎选了 DeepSeek，但 config.json 里没有 deepseek_api_key")?;
        super::use_deepseek(&mut cmd, &key);
    } else {
        // 电话里首字最要紧：用快模型。
        cmd.args(["--model", "haiku"]);
    }
    cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null());
    let mut child = cmd.spawn().map_err(|e| format!("claude 没启动：{e}"))?;
    let stdin = child.stdin.take().ok_or("stdin")?;
    let stdout = child.stdout.take().ok_or("stdout")?;
    {
        let mut slot = BRAIN.lock().map_err(|e| e.to_string())?;
        stop_locked(&mut slot);
        *slot = Some(Brain { child, stdin, call });
    }
    super::trace(format!("voice start call={call} model={model}"));
    std::thread::spawn(move || {
        let emit = |kind: &'static str, text: String| {
            let _ = app.emit_to("main", "sense://voice", VoiceEv { call, kind, text });
        };
        for line in BufReader::new(stdout).lines() {
            let Ok(line) = line else { break };
            let Ok(v) = serde_json::from_str::<serde_json::Value>(&line) else { continue };
            match v.get("type").and_then(|t| t.as_str()) {
                Some("stream_event") => {
                    let e = &v["event"];
                    if e["type"] == "content_block_delta" && e["delta"]["type"] == "text_delta" {
                        if let Some(t) = e["delta"]["text"].as_str() {
                            emit("delta", t.to_string());
                        }
                    }
                }
                Some("result") => {
                    if v.get("is_error").and_then(|x| x.as_bool()).unwrap_or(false) {
                        emit("error", v.get("result").and_then(|r| r.as_str()).unwrap_or("").chars().take(200).collect());
                    }
                    emit("idle", String::new());
                }
                _ => {}
            }
        }
        emit("exit", String::new());
    });
    Ok(())
}

#[tauri::command]
pub fn voice_say(call: u64, text: String) -> Result<(), String> {
    let mut slot = BRAIN.lock().map_err(|e| e.to_string())?;
    let b = slot.as_mut().filter(|b| b.call == call).ok_or("这通电话的大脑不在了")?;
    let msg = serde_json::json!({"type": "user", "message": {"role": "user", "content": text}});
    writeln!(b.stdin, "{msg}").and_then(|_| b.stdin.flush()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn voice_stop() {
    if let Ok(mut slot) = BRAIN.lock() {
        stop_locked(&mut slot);
    }
}

// ---------- voice-live 服务（0030）：由 Sense 自己起、自己关，不用另外打开 ----------
//
// 机器相关的东西（voice-live 的目录、uv 的路径、环境变量）放在本机的 voice.json，不进仓库：
// {"dir": "...", "uv": "...", "port": 8777, "env": {"VOICE_LIVE_TTS": "fish", ...}}
// 端口上已经有人在听就直接用；否则起一个，Sense 退出时连同它的子进程一起关掉。
// 服务的输出不落盘：只在内存里留最后几行，起不来时拿来报错。

use std::collections::VecDeque;
use std::net::{SocketAddr, TcpStream};
use std::time::{Duration, Instant};

static SERVER: Mutex<Option<Child>> = Mutex::new(None);
static STARTING: Mutex<()> = Mutex::new(());
static TAIL: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());

#[derive(serde::Deserialize)]
struct ServerConf {
    dir: String,
    uv: Option<String>,
    port: Option<u16>,
    #[serde(default)]
    env: std::collections::HashMap<String, String>,
}

fn conf() -> Result<ServerConf, String> {
    let p = super::data_dir().join("voice.json");
    let s = std::fs::read_to_string(&p).map_err(|_| format!("没配置语音服务：{} 不存在", p.display()))?;
    serde_json::from_str(&s).map_err(|e| format!("voice.json 读不懂：{e}"))
}

fn listening(port: u16) -> bool {
    TcpStream::connect_timeout(&SocketAddr::from(([127, 0, 0, 1], port)), Duration::from_millis(300)).is_ok()
}

fn ensure() -> Result<String, String> {
    let _g = STARTING.lock().unwrap();
    let c = conf()?;
    let port = c.port.unwrap_or(8777);
    if listening(port) {
        return Ok("ready".into());
    }
    {
        let mut slot = SERVER.lock().unwrap();
        let alive = slot.as_mut().map(|ch| matches!(ch.try_wait(), Ok(None))).unwrap_or(false);
        if !alive {
            use std::os::unix::process::CommandExt;
            let uv = c.uv.clone().unwrap_or_else(|| "uv".into());
            let mut cmd = Command::new(&uv);
            cmd.args(["run", "python", "-m", "voice_live"])
                .current_dir(&c.dir)
                .envs(&c.env)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::piped())
                .process_group(0);
            let mut ch = cmd.spawn().map_err(|e| format!("起不来 voice-live（{uv}）：{e}"))?;
            if let Some(err) = ch.stderr.take() {
                std::thread::spawn(move || {
                    for line in BufReader::new(err).lines().map_while(Result::ok) {
                        let mut t = TAIL.lock().unwrap();
                        t.push_back(line);
                        while t.len() > 12 {
                            t.pop_front();
                        }
                    }
                });
            }
            super::trace(format!("voice-live spawn pid={}", ch.id()));
            *slot = Some(ch);
        }
    }
    let t0 = Instant::now();
    while t0.elapsed() < Duration::from_secs(120) {
        if listening(port) {
            super::trace(format!("voice-live ready after {}s", t0.elapsed().as_secs()));
            return Ok("ready".into());
        }
        let exited = SERVER.lock().unwrap().as_mut().map(|ch| !matches!(ch.try_wait(), Ok(None))).unwrap_or(true);
        if exited {
            let tail = TAIL.lock().unwrap().iter().rev().take(3).cloned().collect::<Vec<_>>().join(" / ");
            super::trace(format!("voice-live exited early"));
            return Err(format!("voice-live 退出了：{}", tail.chars().take(300).collect::<String>()));
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    Err("voice-live 两分钟还没起来".into())
}

/// 页面打电话前调：没起就起，等到能连上为止。
#[tauri::command]
pub async fn voice_server() -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(ensure).await.map_err(|e| e.to_string())?
}

/// 启动 Sense 时在后台先起好（有 voice.json 才起），打电话时就不用等。
pub fn server_boot() {
    if conf().is_ok() {
        std::thread::spawn(|| {
            if let Err(e) = ensure() {
                super::trace(format!("voice-live boot: {e}"));
            }
        });
    }
}

/// Sense 退出时关掉自己起的那个（别人起的不动）。整个进程组一起关，uv 下面的 python 也走。
pub fn server_stop() {
    if let Some(mut ch) = SERVER.lock().unwrap().take() {
        let _ = Command::new("/bin/kill").args(["-TERM", &format!("-{}", ch.id())]).status();
        let t0 = Instant::now();
        while t0.elapsed() < Duration::from_secs(3) {
            if !matches!(ch.try_wait(), Ok(None)) {
                return;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        let _ = Command::new("/bin/kill").args(["-KILL", &format!("-{}", ch.id())]).status();
        let _ = ch.wait();
    }
}
