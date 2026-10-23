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
