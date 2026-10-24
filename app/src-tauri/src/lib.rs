//! Sense 口袋的 Rust 薄壳。
//!
//! 只做网页做不到的事：
//! - 截图全局热键（默认 ⌃⇧⌘A，可在设置里改；0029）→ 系统框选截图 → 把图片交给口袋页面，页面自己建卡、播放进口袋的特效；同一张图也放进系统剪贴板，⌘V 能贴。
//! - 菜单栏图标：左键弹快捷菜单，右键是原生菜单。桌宠：一个透明小窗，拖图片给它就是喂进口袋。
//! - 口袋状态读写到 ~/Library/Application Support/Sense/pocket-state.json，Claude Code 能读回。
//! - 读本机 Chrome 履历与书签，按频次规则出候选。只读作者本人的 profile，副本读完即删，
//!   敏感类页面在读取边界丢弃，只把标题、域名、次数、周数交给页面（书签保留 URL）。

#[cfg(target_os = "macos")]
mod clipboard;
mod voice;
mod charmem;

use base64::Engine;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

/// 不是你本人的 Chrome profile：显示名含这些字的整份跳过，从不打开。
/// 名单只放在本机 ~/Library/Application Support/Sense/config.json 的 `exclude_profiles` 里，不进仓库。
/// 本机的 config.json（权限 600，不进仓库）：exclude_profiles、deepseek_api_key 等。
fn config() -> serde_json::Value {
    fs::read_to_string(data_dir().join("config.json")).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or(serde_json::Value::Null)
}

/// 作者自己填的 DeepSeek key（0030）。Sense 不去别处找 key。
fn deepseek_key() -> Option<String> {
    config().get("deepseek_api_key").and_then(|k| k.as_str()).map(|k| k.trim().to_string()).filter(|k| !k.is_empty())
}

/// 设置页要知道 DeepSeek 能不能选：只回有没有填，不回 key 本身。
#[tauri::command]
fn engine_models() -> serde_json::Value {
    serde_json::json!({ "deepseek": deepseek_key().is_some() })
}

/// 引擎用 DeepSeek 时，仍然跑同一个 claude，只是指到 DeepSeek 的 Anthropic 兼容接口（0030）。
/// WebSearch 由 DeepSeek 服务端执行；Read、WebFetch、Chrome 这些工具照旧在本机跑。
fn use_deepseek(cmd: &mut Command, key: &str) {
    for k in ["ANTHROPIC_API_KEY", "ANTHROPIC_SMALL_FAST_MODEL"] {
        cmd.env_remove(k);
    }
    cmd.env("ANTHROPIC_BASE_URL", "https://api.deepseek.com/anthropic")
        .env("ANTHROPIC_AUTH_TOKEN", key)
        .env("ANTHROPIC_MODEL", "deepseek-flash")
        .env("ANTHROPIC_DEFAULT_OPUS_MODEL", "deepseek-flash")
        .env("ANTHROPIC_DEFAULT_SONNET_MODEL", "deepseek-flash")
        .env("ANTHROPIC_DEFAULT_HAIKU_MODEL", "deepseek-flash")
        .env("CLAUDE_CODE_SUBAGENT_MODEL", "deepseek-flash")
        .env("CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC", "1");
}

fn other_people() -> Vec<String> {
    fs::read_to_string(data_dir().join("config.json"))
        .ok()
        .and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok())
        .and_then(|v| v.get("exclude_profiles").cloned())
        .and_then(|v| serde_json::from_value::<Vec<String>>(v).ok())
        .unwrap_or_default()
        .into_iter()
        .filter(|n| !n.trim().is_empty())
        .collect()
}

/// 读取边界：命中就整行丢弃，不进入任何返回值。
const BLOCK_HOST: &[&str] = &[
    "accounts.", "login.", "auth.", "signin", "sso.", "id.", "passport", "pay.", "payment", "checkout",
    "stripe.", "paypal.", "bank", "mail.", "gmail", "outlook.", "calendar.", "admin.", "console.",
    "dashboard.", "billing", "invoice", "myaccount", "secure.", "vault", "1password", "bitwarden",
    "hospital", "clinic", "visa", "immigration", "mynumber", "tax", "nenkin",
];
const BLOCK_TITLE: &[&str] = &[
    "登录", "登入", "ログイン", "sign in", "log in", "password", "密码", "パスワード", "支付", "付款",
    "決済", "お支払い", "银行", "銀行", "invoice", "請求", "医疗", "医療", "病院", "诊所", "签证",
    "ビザ", "在留", "住所", "address", "verification", "验证码", "認証",
];

fn data_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    let p = PathBuf::from(home).join("Library/Application Support/Sense");
    let _ = fs::create_dir_all(&p);
    p
}

fn chrome_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join("Library/Application Support/Google/Chrome")
}

// ---------- 口袋状态 ----------

/// 引擎每一步落一行到 engine.log，方便查"跑了没、卡在哪"。只写步骤名和数字，不写内容。
#[tauri::command]
fn trace(msg: String) {
    use std::io::Write;
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(data_dir().join("engine.log")) {
        let _ = writeln!(f, "{now} {}", msg.chars().take(300).collect::<String>());
    }
}

#[tauri::command]
fn load_state() -> Option<String> {
    fs::read_to_string(data_dir().join("pocket-state.json")).ok()
}

#[tauri::command]
fn save_state(state: String) -> Result<(), String> {
    serde_json::from_str::<serde_json::Value>(&state).map_err(|e| format!("状态不是合法 JSON：{e}"))?;
    let dir = data_dir();
    let tmp = dir.join("pocket-state.json.tmp");
    fs::write(&tmp, state).map_err(|e| e.to_string())?;
    fs::rename(&tmp, dir.join("pocket-state.json")).map_err(|e| e.to_string())
}

// ---------- 截图 ----------

#[derive(Clone, Serialize)]
struct Clip {
    b64: String,
    mime: &'static str,
    source_app: String,
    /// 是否也复制到了系统剪贴板
    copied: bool,
}

fn frontmost_app() -> String {
    // lsappinfo 不需要额外权限。
    let front = Command::new("lsappinfo").arg("front").output().ok();
    let asn = front.map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default();
    if asn.is_empty() {
        return String::new();
    }
    Command::new("lsappinfo")
        .args(["info", "-only", "name", &asn])
        .output()
        .ok()
        .map(|o| {
            let s = String::from_utf8_lossy(&o.stdout).to_string();
            s.split('=').nth(1).unwrap_or("").trim().trim_matches('"').to_string()
        })
        .unwrap_or_default()
}

fn snap(app: AppHandle) {
    std::thread::spawn(move || {
        let source_app = frontmost_app();
        let path = std::env::temp_dir().join(format!("sense-snap-{}.png", std::process::id()));
        let _ = fs::remove_file(&path);
        // -i 交互框选，-x 不出快门声。用户按 Esc 取消时不会生成文件。
        trace(format!("snap start from={source_app}"));
        let out = Command::new("screencapture").args(["-i", "-x"]).arg(&path).output();
        let ok = out.as_ref().map(|o| o.status.success()).unwrap_or(false);
        if !ok || !path.exists() {
            let why = match &out {
                Ok(o) => format!("code={:?} {}", o.status.code(), String::from_utf8_lossy(&o.stderr).trim().chars().take(160).collect::<String>()),
                Err(e) => e.to_string(),
            };
            trace(format!("snap no-image {why}"));
            // Esc 取消也走这里（code=1、没有 stderr），不打扰；有报错才告诉页面。
            if out.as_ref().map(|o| !o.stderr.is_empty()).unwrap_or(true) {
                let _ = app.emit("sense://snap-fail", why);
            }
            return;
        }
        let bytes = match fs::read(&path) {
            Ok(b) => b,
            Err(_) => return,
        };
        let _ = fs::remove_file(&path); // 原图不留在磁盘上
        // AppKit 的剪贴板放在主线程写；后台线程写过一次，⌘V 贴不出来。
        #[cfg(target_os = "macos")]
        let copied = {
            let (tx, rx) = std::sync::mpsc::channel();
            let png = bytes.clone();
            let _ = app.run_on_main_thread(move || {
                let _ = tx.send(clipboard::put_png(&png));
            });
            rx.recv_timeout(std::time::Duration::from_secs(3)).unwrap_or(false)
        };
        #[cfg(not(target_os = "macos"))]
        let copied = false;
        trace(format!("snap image bytes={} copied={copied}", bytes.len()));
        #[cfg(target_os = "macos")]
        {
            let app2 = app.clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(1500));
                let (tx, rx) = std::sync::mpsc::channel();
                let _ = app2.run_on_main_thread(move || {
                    let _ = tx.send(clipboard::png_still_ours());
                });
                trace(format!("snap clipboard-after-1.5s ours={:?}", rx.recv_timeout(std::time::Duration::from_secs(3)).ok()));
            });
        }
        // 截图就是截图：不把口袋窗口拉到前面，进卡由桌宠的气泡告诉你。
        let _ = app.emit(
            "sense://clip",
            Clip { b64: base64::engine::general_purpose::STANDARD.encode(bytes), mime: "image/png", source_app, copied },
        );
    });
}

#[tauri::command]
fn snap_now(app: AppHandle) {
    snap(app);
}

// ---------- 浏览器履历 ----------

#[derive(Serialize)]
struct HistoryRow {
    key: String,
    title: String,
    host: String,
    visits: u32,
    weeks: u32,
    first: String,
    last: String,
    profiles: Vec<String>,
}

#[derive(Serialize)]
struct Bookmark {
    title: String,
    url: String,
    host: String,
    profile: String,
    added: String,
}

#[derive(Serialize)]
struct ScanResult {
    profiles_read: Vec<String>,
    profiles_skipped: Vec<String>,
    /// 读失败的 profile 和原因（比如系统不让读），页面上照实显示，不再悄悄变成 0 条。
    profiles_failed: Vec<String>,
    visits_seen: u64,
    visits_dropped: u64,
    rows: Vec<HistoryRow>,
    bookmarks: Vec<Bookmark>,
}

fn own_profiles() -> (Vec<(String, String)>, Vec<String>) {
    let mut own = vec![];
    let mut skipped = vec![];
    let ls = fs::read_to_string(chrome_dir().join("Local State")).unwrap_or_default();
    let v: serde_json::Value = serde_json::from_str(&ls).unwrap_or(serde_json::Value::Null);
    let others = other_people();
    if let Some(cache) = v.pointer("/profile/info_cache").and_then(|c| c.as_object()) {
        for (dir, info) in cache {
            let name = info.get("name").and_then(|n| n.as_str()).unwrap_or(dir).to_string();
            if others.iter().any(|o| name.contains(o.as_str())) {
                skipped.push(name);
            } else {
                own.push((dir.clone(), name));
            }
        }
    }
    (own, skipped)
}

fn host_of(url: &str) -> String {
    let rest = url.split("://").nth(1).unwrap_or("");
    rest.split(['/', '?', '#']).next().unwrap_or("").trim_start_matches("www.").to_lowercase()
}

fn blocked(host: &str, title: &str, url: &str) -> bool {
    let t = title.to_lowercase();
    let u = url.to_lowercase();
    BLOCK_HOST.iter().any(|b| host.contains(b))
        || BLOCK_TITLE.iter().any(|b| t.contains(&b.to_lowercase()))
        || u.contains("password")
        || u.contains("/login")
        || u.contains("/signin")
        || u.contains("/checkout")
        || host.is_empty()
        || u.starts_with("chrome")
        || u.starts_with("file:")
        || u.starts_with("http://localhost")
        || u.starts_with("http://127.")
}

/// 把同一部作品、同一个频道的不同页面归成一条：去掉站点后缀和集数。
fn normalize_title(title: &str) -> String {
    let mut t = title.to_string();
    for sep in [" - YouTube", "_哔哩哔哩_bilibili", "_bilibili", " | ", " - ", " – ", " — ", "_"] {
        if let Some(i) = t.find(sep) {
            if i > 3 {
                t.truncate(i);
            }
        }
    }
    let mut out = String::new();
    let chars: Vec<char> = t.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        // 去掉"第12话""EP3""#5""12集"这类集数标记里的数字
        if c.is_ascii_digit() {
            i += 1;
            continue;
        }
        out.push(c);
        i += 1;
    }
    let cleaned: String = out
        .replace("第话", "").replace("第集", "").replace("第回", "").replace("第話", "")
        .replace("EP", "").replace("Ep", "").replace("ep", "").replace('#', "")
        .split_whitespace().collect::<Vec<_>>().join(" ");
    cleaned.chars().take(30).collect::<String>().trim().to_string()
}

/// Chrome 时间戳：1601-01-01 起的微秒。返回 Unix 秒。
fn chrome_to_unix(t: i64) -> i64 {
    t / 1_000_000 - 11_644_473_600
}

fn ymd(unix: i64) -> String {
    // 不引入日期库：民用历换算。
    let days = unix.div_euclid(86_400);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    format!("{:04}-{:02}-{:02}", if m <= 2 { y + 1 } else { y }, m, d)
}

struct Agg {
    title: String,
    host: String,
    visits: u32,
    weeks: HashSet<i64>,
    first: i64,
    last: i64,
    profiles: HashSet<String>,
}

fn read_history(dir: &Path, name: &str, since_chrome: i64, agg: &mut HashMap<String, Agg>, seen: &mut u64, dropped: &mut u64) -> Result<(), String> {
    let src = chrome_dir().join(dir).join("History");
    if !src.exists() {
        return Ok(());
    }
    let tmpdir = std::env::temp_dir().join(format!("sense-hist-{}-{}", std::process::id(), dir.display().to_string().replace(' ', "_")));
    let _ = fs::create_dir_all(&tmpdir);
    let dst = tmpdir.join("History");
    fs::copy(&src, &dst).map_err(|e| e.to_string())?;
    let wal = chrome_dir().join(dir).join("History-wal");
    if wal.exists() {
        let _ = fs::copy(&wal, tmpdir.join("History-wal"));
    }
    let result = (|| -> Result<(), String> {
        let conn = rusqlite::Connection::open_with_flags(&dst, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY).map_err(|e| e.to_string())?;
        let mut st = conn
            .prepare("SELECT urls.url, urls.title, visits.visit_time FROM visits JOIN urls ON urls.id = visits.url WHERE visits.visit_time > ?1")
            .map_err(|e| e.to_string())?;
        let rows = st
            .query_map([since_chrome], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1).unwrap_or_default(), r.get::<_, i64>(2)?)))
            .map_err(|e| e.to_string())?;
        for row in rows.flatten() {
            let (url, title, vt) = row;
            *seen += 1;
            let host = host_of(&url);
            if title.trim().is_empty() || blocked(&host, &title, &url) {
                *dropped += 1;
                continue;
            }
            let norm = normalize_title(&title);
            if norm.chars().count() < 2 {
                *dropped += 1;
                continue;
            }
            let key = format!("{host}|{norm}");
            let unix = chrome_to_unix(vt);
            let e = agg.entry(key).or_insert_with(|| Agg { title: norm.clone(), host: host.clone(), visits: 0, weeks: HashSet::new(), first: unix, last: unix, profiles: HashSet::new() });
            e.visits += 1;
            e.weeks.insert(unix.div_euclid(7 * 86_400));
            e.first = e.first.min(unix);
            e.last = e.last.max(unix);
            e.profiles.insert(name.to_string());
        }
        Ok(())
    })();
    let _ = fs::remove_dir_all(&tmpdir); // 副本读完即删
    result
}

fn walk_bookmarks(node: &serde_json::Value, profile: &str, out: &mut Vec<Bookmark>) {
    if node.get("type").and_then(|t| t.as_str()) == Some("url") {
        let url = node.get("url").and_then(|u| u.as_str()).unwrap_or("").to_string();
        let title = node.get("name").and_then(|u| u.as_str()).unwrap_or("").to_string();
        let host = host_of(&url);
        if !title.is_empty() && !blocked(&host, &title, &url) && url.starts_with("https://") {
            let added = node.get("date_added").and_then(|d| d.as_str()).and_then(|d| d.parse::<i64>().ok()).map(|d| ymd(chrome_to_unix(d))).unwrap_or_default();
            out.push(Bookmark { title, url, host, profile: profile.to_string(), added });
        }
    }
    if let Some(ch) = node.get("children").and_then(|c| c.as_array()) {
        for c in ch {
            walk_bookmarks(c, profile, out);
        }
    }
}

#[tauri::command]
async fn scan_history(days: Option<i64>) -> Result<ScanResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let (own, skipped) = own_profiles();
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0);
        let since_unix = now - days.unwrap_or(365) * 86_400;
        let since_chrome = (since_unix + 11_644_473_600) * 1_000_000;
        let mut agg: HashMap<String, Agg> = HashMap::new();
        let (mut seen, mut dropped) = (0u64, 0u64);
        let mut read = vec![];
        let mut failed = vec![];
        let mut bookmarks = vec![];
        for (dir, name) in &own {
            match read_history(Path::new(dir), name, since_chrome, &mut agg, &mut seen, &mut dropped) {
                Ok(()) => read.push(name.clone()),
                Err(e) => failed.push(format!("{name}：{e}")),
            }
            if let Ok(txt) = fs::read_to_string(chrome_dir().join(dir).join("Bookmarks")) {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&txt) {
                    if let Some(roots) = v.get("roots").and_then(|r| r.as_object()) {
                        for r in roots.values() {
                            walk_bookmarks(r, name, &mut bookmarks);
                        }
                    }
                }
            }
        }
        // 频次规则：跨 ≥3 周且 ≥5 次。
        let mut rows: Vec<HistoryRow> = agg
            .into_iter()
            .filter(|(_, a)| a.weeks.len() >= 3 && a.visits >= 5)
            .map(|(key, a)| HistoryRow {
                key,
                title: a.title,
                host: a.host,
                visits: a.visits,
                weeks: a.weeks.len() as u32,
                first: ymd(a.first),
                last: ymd(a.last),
                profiles: a.profiles.into_iter().collect(),
            })
            .collect();
        rows.sort_by(|a, b| b.weeks.cmp(&a.weeks).then(b.visits.cmp(&a.visits)));
        rows.truncate(200);
        bookmarks.sort_by(|a, b| b.added.cmp(&a.added));
        bookmarks.truncate(80);
        Ok(ScanResult { profiles_read: read, profiles_skipped: skipped, profiles_failed: failed, visits_seen: seen, visits_dropped: dropped, rows, bookmarks })
    })
    .await
    .map_err(|e| e.to_string())?
}


// ---------- 让 Claude 干活 ----------
//
// 引擎留在 Claude Code（决策 0020）：口袋 app 不带模型，需要判断、联网、认图时
// 调本机的 `claude -p`。不留会话（--no-session-persistence），不加载 MCP，
// 只放行这次要用的工具；在 Sense 自己的数据目录里跑，不会读到任何项目的 CLAUDE.md。

fn claude_bin() -> Option<PathBuf> {
    let home = std::env::var("HOME").unwrap_or_default();
    for p in [format!("{home}/.local/bin/claude"), format!("{home}/.claude/local/claude"), "/opt/homebrew/bin/claude".into(), "/usr/local/bin/claude".into()] {
        if Path::new(&p).exists() {
            return Some(PathBuf::from(p));
        }
    }
    // 从 Finder 启动时没有 shell 的 PATH，问一次登录 shell。
    let out = Command::new("/bin/zsh").args(["-lc", "command -v claude"]).output().ok()?;
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() { None } else { Some(PathBuf::from(s)) }
}

const CHROME_READ: [&str; 6] = [
    "mcp__claude-in-chrome__tabs_context_mcp",
    "mcp__claude-in-chrome__tabs_create_mcp",
    "mcp__claude-in-chrome__navigate",
    "mcp__claude-in-chrome__get_page_text",
    "mcp__claude-in-chrome__find",
    "mcp__claude-in-chrome__tabs_close_mcp",
];

#[tauri::command]
async fn run_claude(prompt: String, web: Option<bool>, chrome: Option<bool>, images: Option<Vec<String>>, read_dirs: Option<Vec<String>>, max_turns: Option<u32>, model: Option<String>) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let bin = claude_bin().ok_or("没找到 claude 命令。先装 Claude Code 并登录一次。")?;
        let deepseek = model.as_deref() == Some("deepseek");
        let ds_key = if deepseek { Some(deepseek_key().ok_or("引擎选了 DeepSeek，但还没填 key：在 ~/Library/Application Support/Sense/config.json 里加 \"deepseek_api_key\"。")?) } else { None };
        let t0 = std::time::Instant::now();
        let work = data_dir().join("agent");
        let _ = fs::create_dir_all(&work);
        let mut tools: Vec<&str> = vec![];
        let mut prompt = prompt;
        let mut tmp_imgs = vec![];
        for (i, b64) in images.unwrap_or_default().iter().enumerate() {
            let raw = b64.split(',').last().unwrap_or("");
            let bytes = base64::engine::general_purpose::STANDARD.decode(raw).map_err(|e| e.to_string())?;
            let p = work.join(format!("img-{}-{i}.jpg", std::process::id()));
            fs::write(&p, bytes).map_err(|e| e.to_string())?;
            prompt.push_str(&format!("\n\n先用 Read 工具看这张图片：{}", p.display()));
            tmp_imgs.push(p);
        }
        if !tmp_imgs.is_empty() || read_dirs.as_ref().map(|d| !d.is_empty()).unwrap_or(false) {
            tools.extend(["Read", "Glob", "Grep"]);
        }
        if web.unwrap_or(false) {
            tools.extend(["WebSearch", "WebFetch"]);
        }
        // 用作者本人登录着的 Chrome（Claude in Chrome）。只给只读的几样：开标签、导航、读文字、关标签。
        // 不给点击、填表、执行脚本、上传。去哪些网站由调用方的 prompt 限定（白名单见决策 0026）。
        let chrome = chrome.unwrap_or(false);
        if chrome {
            tools.extend(CHROME_READ);
        }
        let mut cmd = Command::new(bin);
        cmd.current_dir(&work)
            .args(["-p", "--output-format", "json", "--no-session-persistence", "--strict-mcp-config"])
            .args(["--max-turns", &max_turns.unwrap_or(16).to_string()]);
        if chrome {
            cmd.arg("--chrome");
        }
        if let Some(k) = &ds_key {
            use_deepseek(&mut cmd, k);
        }
        if !tools.is_empty() {
            cmd.arg("--allowedTools").arg(tools.join(","));
        }
        for d in read_dirs.unwrap_or_default() {
            cmd.arg("--add-dir").arg(d.replace('~', &std::env::var("HOME").unwrap_or_default()));
        }
        cmd.stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped());
        let mut child = cmd.spawn().map_err(|e| format!("claude 没启动：{e}"))?;
        {
            use std::io::Write;
            let mut stdin = child.stdin.take().ok_or("stdin")?;
            stdin.write_all(prompt.as_bytes()).map_err(|e| e.to_string())?;
        }
        let out = child.wait_with_output().map_err(|e| e.to_string());
        for p in tmp_imgs {
            let _ = fs::remove_file(p); // 截图不留在磁盘上
        }
        let out = out?;
        let txt = String::from_utf8_lossy(&out.stdout).to_string();
        let v: serde_json::Value = serde_json::from_str(&txt).map_err(|_| {
            let err = String::from_utf8_lossy(&out.stderr);
            format!("claude 没有返回结果：{}", err.chars().take(300).collect::<String>())
        })?;
        let result = v.get("result").and_then(|r| r.as_str()).unwrap_or("").to_string();
        trace(format!("agent model={} web={} ms={} err={}", if deepseek { "deepseek" } else { "claude" }, web.unwrap_or(false), t0.elapsed().as_millis(),
            v.get("is_error").and_then(|e| e.as_bool()).unwrap_or(false)));
        if v.get("is_error").and_then(|e| e.as_bool()).unwrap_or(false) {
            return Err(result);
        }
        Ok(result)
    })
    .await
    .map_err(|e| e.to_string())?
}

// ---------- 电脑上的线索 ----------

#[derive(Serialize)]
struct Project { name: String, sessions: u32, last: String }
#[derive(Serialize)]
struct LocalFile { name: String, ext: String, place: String, modified: String }
#[derive(Serialize)]
struct ContextScan {
    projects: Vec<Project>,
    codex_sessions: u32,
    files: Vec<LocalFile>,
    games: Vec<String>,
    screenshots: u32,
}

const BLOCK_FILE: &[&str] = &[
    "passport", "visa", "在留", "マイナンバー", "mynumber", "bank", "銀行", "银行", "invoice", "請求", "receipt", "領収",
    "tax", "税", "給与", "salary", "payslip", "contract", "契約", "保険", "insurance", "medical", "診断", "病院", "password",
    "id_card", "身份证", "住民票", "statement", "明細",
];

fn mtime(p: &Path) -> i64 {
    fs::metadata(p).and_then(|m| m.modified()).ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_secs() as i64).unwrap_or(0)
}

#[tauri::command]
async fn scan_context() -> Result<ContextScan, String> {
    tauri::async_runtime::spawn_blocking(|| {
        let home = PathBuf::from(std::env::var("HOME").unwrap_or_default());
        // Claude Code 项目：只数会话个数与最近时间，不读内容。目录名是路径把 / 换成 -。
        let mut projects = vec![];
        if let Ok(rd) = fs::read_dir(home.join(".claude/projects")) {
            for e in rd.flatten() {
                let dir = e.path();
                let name = e.file_name().to_string_lossy().to_string();
                if name.contains("Sense-agent") || name.contains("Application-Support-Sense") {
                    continue; // 口袋自己调 claude 的工作目录
                }
                let mut n = 0;
                let mut last = 0;
                if let Ok(files) = fs::read_dir(&dir) {
                    for f in files.flatten() {
                        if f.path().extension().map(|x| x == "jsonl").unwrap_or(false) {
                            n += 1;
                            last = last.max(mtime(&f.path()));
                        }
                    }
                }
                let short = name.rsplit('-').find(|s| !s.is_empty()).unwrap_or(&name).to_string();
                if n > 0 {
                    projects.push((last, Project { name: short, sessions: n, last: ymd(last) }));
                }
            }
        }
        projects.sort_by(|a, b| b.0.cmp(&a.0));
        let projects = projects.into_iter().map(|p| p.1).take(40).collect();
        let mut codex = 0;
        fn walk(p: &Path, n: &mut u32, depth: u32) {
            if depth > 5 { return; }
            if let Ok(rd) = fs::read_dir(p) {
                for e in rd.flatten() {
                    let q = e.path();
                    if q.is_dir() { walk(&q, n, depth + 1); } else if q.extension().map(|x| x == "jsonl").unwrap_or(false) { *n += 1; }
                }
            }
        }
        walk(&home.join(".codex/sessions"), &mut codex, 0);
        // 桌面 / 下载 / 文稿顶层：只要文件名，不打开文件。
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0);
        let mut files = vec![];
        let mut shots = 0;
        for (place, sub) in [("桌面", "Desktop"), ("下载", "Downloads"), ("文稿", "Documents")] {
            if let Ok(rd) = fs::read_dir(home.join(sub)) {
                for e in rd.flatten() {
                    let p = e.path();
                    let name = e.file_name().to_string_lossy().to_string();
                    if name.starts_with('.') || p.is_dir() { continue; }
                    let low = name.to_lowercase();
                    if low.starts_with("screenshot") || name.starts_with("スクリーンショット") || name.starts_with("截屏") { shots += 1; continue; }
                    if BLOCK_FILE.iter().any(|b| low.contains(b)) { continue; }
                    let ext = p.extension().map(|x| x.to_string_lossy().to_lowercase()).unwrap_or_default();
                    if !["pdf", "epub", "md", "txt", "docx", "pptx", "key", "pages", "png", "jpg", "jpeg", "mp4", "zip"].contains(&ext.as_str()) { continue; }
                    let m = mtime(&p);
                    if now - m > 120 * 86_400 { continue; }
                    files.push((m, LocalFile { name, ext, place: place.into(), modified: ymd(m) }));
                }
            }
        }
        files.sort_by(|a, b| b.0.cmp(&a.0));
        let files = files.into_iter().map(|f| f.1).take(80).collect();
        // 装了的游戏：Steam 库目录名。
        let mut games = vec![];
        if let Ok(rd) = fs::read_dir(home.join("Library/Application Support/Steam/steamapps/common")) {
            for e in rd.flatten() {
                let n = e.file_name().to_string_lossy().to_string();
                if !n.starts_with('.') && !n.to_lowercase().contains("steamworks") { games.push(n); }
            }
        }
        Ok(ContextScan { projects, codex_sessions: codex, files, games, screenshots: shots })
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
fn clip_img_watch(on: bool) {
    #[cfg(target_os = "macos")]
    clipboard::IMG_WATCH.store(on, std::sync::atomic::Ordering::Relaxed);
    let _ = on;
}

/// 截图键（0029）。默认 ⌃⇧⌘A：⌃⌘A 是微信的截图键，两个 app 都注册时系统不报错，按键却会被先注册的那个吃掉。
const DEFAULT_SNAP_KEY: &str = "ctrl+shift+super+KeyA";
static SNAP_KEY: std::sync::Mutex<Option<Shortcut>> = std::sync::Mutex::new(None);

/// 换截图键：先注册新的，成功了再放掉旧的；新的注册不上就保留旧的，把原因交回页面。
#[tauri::command]
fn set_snap_key(app: AppHandle, accel: String) -> Result<String, String> {
    let accel = if accel.is_empty() { DEFAULT_SNAP_KEY.to_string() } else { accel };
    let key: Shortcut = accel.parse().map_err(|e| format!("认不出这个组合：{e}"))?;
    let gs = app.global_shortcut();
    let mut cur = SNAP_KEY.lock().map_err(|e| e.to_string())?;
    if cur.as_ref() == Some(&key) {
        return Ok(accel);
    }
    gs.register(key).map_err(|e| format!("注册不上：{e}"))?;
    if let Some(old) = cur.take() {
        let _ = gs.unregister(old);
    }
    *cur = Some(key);
    Ok(accel)
}

#[tauri::command]
fn clip_watch(on: bool) {
    #[cfg(target_os = "macos")]
    clipboard::WATCH.store(on, std::sync::atomic::Ordering::Relaxed);
    let _ = on;
}

/// 外链交给系统浏览器；只放行 https。
#[tauri::command]
fn open_url(url: String) -> Result<(), String> {
    if !url.starts_with("https://") || url.contains(char::is_whitespace) {
        return Err("只打开 https 链接".into());
    }
    Command::new("open").arg(&url).status().map(|_| ()).map_err(|e| e.to_string())
}

/// 展示卡存成图片，放进「下载」。
#[tauri::command]
fn save_png(name: String, b64: String) -> Result<String, String> {
    let raw = b64.split(',').last().unwrap_or("");
    let bytes = base64::engine::general_purpose::STANDARD.decode(raw).map_err(|e| e.to_string())?;
    let safe: String = name.chars().filter(|c| c.is_alphanumeric() || "-_.".contains(*c)).collect();
    let p = PathBuf::from(std::env::var("HOME").unwrap_or_default()).join("Downloads").join(if safe.is_empty() { "sense.png".into() } else { safe });
    fs::write(&p, bytes).map_err(|e| e.to_string())?;
    let _ = Command::new("open").arg("-R").arg(&p).status();
    Ok(p.display().to_string())
}


// ---------- YouTube 观看记录 ----------
//
// 音乐的来源（作者 2026-10-22："音乐可以看我 youtube 的阅读记录"）。只取 youtube.com / music.youtube.com 的
// watch 页标题，按标题归并，返回次数、周数、首末日期；不返回 URL。和 scan_history 一样只读本人 profile，副本读完即删。

#[derive(Serialize)]
struct WatchRow {
    title: String,
    music_app: bool,
    plays: u32,
    weeks: u32,
    first: String,
    last: String,
}

fn clean_yt_title(t: &str) -> String {
    let mut s = t.trim().to_string();
    // 开头的 "(3) " 是未读通知数
    if s.starts_with('(') {
        if let Some(i) = s.find(") ") {
            if s[1..i].chars().all(|c| c.is_ascii_digit()) {
                s = s[i + 2..].to_string();
            }
        }
    }
    for suf in [" - YouTube Music", " - YouTube"] {
        if let Some(x) = s.strip_suffix(suf) {
            s = x.to_string();
        }
    }
    s.trim().to_string()
}

#[derive(Serialize)]
struct WatchScan {
    rows: Vec<WatchRow>,
    ytm_visits: u32,   // music.youtube.com 的播放次数
    ytm_untitled: u32, // 其中标题只是 "YouTube Music"、Chrome 没记下歌名的
}

#[tauri::command]
async fn scan_youtube(days: Option<i64>) -> Result<WatchScan, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let (own, _) = own_profiles();
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0);
        let since_chrome = (now - days.unwrap_or(730) * 86_400 + 11_644_473_600) * 1_000_000;
        let mut agg: HashMap<String, (bool, u32, HashSet<i64>, i64, i64)> = HashMap::new();
        let (mut ytm_visits, mut ytm_untitled) = (0u32, 0u32);
        for (dir, _) in &own {
            let src = chrome_dir().join(dir).join("History");
            if !src.exists() {
                continue;
            }
            let tmpdir = std::env::temp_dir().join(format!("sense-yt-{}-{}", std::process::id(), dir.replace(' ', "_")));
            let _ = fs::create_dir_all(&tmpdir);
            let dst = tmpdir.join("History");
            if fs::copy(&src, &dst).is_err() {
                let _ = fs::remove_dir_all(&tmpdir);
                continue;
            }
            let wal = chrome_dir().join(dir).join("History-wal");
            if wal.exists() {
                let _ = fs::copy(&wal, tmpdir.join("History-wal"));
            }
            let _ = (|| -> Result<(), rusqlite::Error> {
                let conn = rusqlite::Connection::open_with_flags(&dst, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)?;
                let mut st = conn.prepare(
                    "SELECT urls.url, urls.title, visits.visit_time FROM visits JOIN urls ON urls.id = visits.url \
                     WHERE visits.visit_time > ?1 AND (urls.url LIKE 'https://www.youtube.com/watch%' OR urls.url LIKE 'https://m.youtube.com/watch%' OR urls.url LIKE 'https://music.youtube.com/watch%')",
                )?;
                let rows = st.query_map([since_chrome], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1).unwrap_or_default(), r.get::<_, i64>(2)?)))?;
                for (url, title, vt) in rows.flatten() {
                    let t = clean_yt_title(&title);
                    if url.starts_with("https://music.") {
                        ytm_visits += 1;
                        if t.is_empty() || t == "YouTube Music" {
                            ytm_untitled += 1;
                        }
                    }
                    if t.chars().count() < 2 || t == "YouTube" || t == "YouTube Music" {
                        continue;
                    }
                    let unix = chrome_to_unix(vt);
                    let e = agg.entry(t).or_insert((false, 0, HashSet::new(), unix, unix));
                    e.0 |= url.starts_with("https://music.");
                    e.1 += 1;
                    e.2.insert(unix.div_euclid(7 * 86_400));
                    e.3 = e.3.min(unix);
                    e.4 = e.4.max(unix);
                }
                Ok(())
            })();
            let _ = fs::remove_dir_all(&tmpdir); // 副本读完即删
        }
        let mut out: Vec<WatchRow> = agg
            .into_iter()
            .filter(|(_, v)| v.1 >= 2 || v.0)
            .map(|(title, v)| WatchRow { title, music_app: v.0, plays: v.1, weeks: v.2.len() as u32, first: ymd(v.3), last: ymd(v.4) })
            .collect();
        out.sort_by(|a, b| b.weeks.cmp(&a.weeks).then(b.plays.cmp(&a.plays)));
        out.truncate(400);
        Ok(WatchScan { rows: out, ytm_visits, ytm_untitled })
    })
    .await
    .map_err(|e| e.to_string())?
}

// ---------- 快捷菜单与桌宠 ----------
//
// 菜单栏图标、头部小口袋、桌宠，点哪个都弹同一个下拉菜单（menu 窗口，menu.html）。
// 菜单和桌宠不碰口袋状态：它们只听主窗口推来的 sense://summary，按钮发 sense://cmd 给主窗口，
// 状态只有主窗口一个写入方。

const MENU_W: f64 = 300.0;
const PET_W: f64 = 184.0;
const PET_H: f64 = 236.0;

/// 菜单失焦收起的时刻。点图标本身也会先让菜单失焦，紧接着的那次点击不该再把它打开。
static MENU_HIDDEN_AT: std::sync::Mutex<Option<std::time::Instant>> = std::sync::Mutex::new(None);

fn show_main_window(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.unminimize();
        let _ = w.set_focus();
    }
}

fn toggle_main(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        if w.is_visible().unwrap_or(false) && w.is_focused().unwrap_or(false) {
            let _ = w.hide();
        } else {
            show_main_window(app);
        }
    }
}

fn hide_menu(app: &AppHandle) {
    if let Some(m) = app.get_webview_window("menu") {
        if m.is_visible().unwrap_or(false) {
            let _ = m.hide();
            *MENU_HIDDEN_AT.lock().unwrap() = Some(std::time::Instant::now());
        }
    }
}

/// 在一块屏幕矩形（物理像素）下方居中弹出菜单；下面放不下就放到上方。再点一次就收起。
fn toggle_menu_at(app: &AppHandle, x: f64, y: f64, w: f64, h: f64) {
    let Some(m) = app.get_webview_window("menu") else { return };
    if m.is_visible().unwrap_or(false) {
        hide_menu(app);
        return;
    }
    if let Some(t) = *MENU_HIDDEN_AT.lock().unwrap() {
        if t.elapsed() < std::time::Duration::from_millis(300) {
            return;
        }
    }
    let sf = m.scale_factor().unwrap_or(2.0);
    let mw = MENU_W * sf;
    let mh = m.outer_size().map(|s| s.height as f64).unwrap_or(480.0 * sf);
    let gap = 6.0 * sf;
    let (mut nx, mut ny) = (x + w / 2.0 - mw / 2.0, y + h + gap);
    if let Ok(Some(mon)) = app.monitor_from_point(x + w / 2.0, y + h / 2.0) {
        let wa = mon.work_area();
        let (l, t) = (wa.position.x as f64, wa.position.y as f64);
        let (r, b) = (l + wa.size.width as f64, t + wa.size.height as f64);
        if ny + mh > b {
            ny = (y - gap - mh).max(t);
        }
        nx = nx.clamp(l + gap, (r - mw - gap).max(l));
    }
    let _ = m.set_position(tauri::PhysicalPosition::new(nx.round() as i32, ny.round() as i32));
    let _ = m.show();
    let _ = m.set_focus();
    let _ = m.emit("sense://menu-open", ());
}

/// 主窗口或桌宠里某个元素被点了：rect 是该窗口内容区里的逻辑坐标。
#[tauri::command]
fn menu_toggle(app: AppHandle, window: tauri::WebviewWindow, x: f64, y: f64, w: f64, h: f64, pressed_ms_ago: Option<f64>) {
    // 桌宠单击要等一下才知道不是双击；按下那一刻菜单已经因失焦收起了，这一下算「收起」，不要再弹出来。
    if let (Some(ago), Some(t)) = (pressed_ms_ago, *MENU_HIDDEN_AT.lock().unwrap()) {
        if t.elapsed().as_secs_f64() * 1000.0 <= ago + 80.0 {
            return;
        }
    }
    let sf = window.scale_factor().unwrap_or(2.0);
    let Ok(p) = window.inner_position() else { return };
    toggle_menu_at(&app, p.x as f64 + x * sf, p.y as f64 + y * sf, w * sf, h * sf);
}

#[tauri::command]
fn menu_hide(app: AppHandle) {
    hide_menu(&app);
}

/// 菜单按内容高度自适应。
#[tauri::command]
fn menu_fit(app: AppHandle, h: f64) {
    if let Some(m) = app.get_webview_window("menu") {
        let _ = m.set_size(tauri::LogicalSize::new(MENU_W, h.clamp(120.0, 760.0)));
    }
}

#[tauri::command]
fn show_main(app: AppHandle) {
    hide_menu(&app);
    show_main_window(&app);
}

#[tauri::command]
fn quit_app(app: AppHandle) {
    app.exit(0);
}

/// 显示或收起桌宠。没有记过位置就放在主屏幕右下角。
#[tauri::command]
fn pet_show(app: AppHandle, on: bool, x: Option<i32>, y: Option<i32>) {
    let Some(p) = app.get_webview_window("pet") else { return };
    if !on {
        let _ = p.hide();
        return;
    }
    let sf = p.scale_factor().unwrap_or(2.0);
    let on_screen = |x: i32, y: i32| app.monitor_from_point(x as f64 + 40.0, y as f64 + 40.0).ok().flatten().is_some();
    let pos = match (x, y) {
        (Some(x), Some(y)) if on_screen(x, y) => Some((x, y)),
        _ => app.primary_monitor().ok().flatten().map(|m| {
            let wa = m.work_area();
            (
                wa.position.x + wa.size.width as i32 - ((PET_W + 48.0) * sf) as i32,
                wa.position.y + wa.size.height as i32 - ((PET_H + 24.0) * sf) as i32,
            )
        }),
    };
    if let Some((x, y)) = pos {
        let _ = p.set_position(tauri::PhysicalPosition::new(x, y));
    }
    let _ = p.show();
}

#[tauri::command]
fn pet_drag(app: AppHandle) {
    hide_menu(&app);
    if let Some(p) = app.get_webview_window("pet") {
        let _ = p.start_dragging();
    }
}

/// 把图片拖到桌宠身上 = 喂给口袋，和截图走同一条路。只收图片，最多 5 张，每张 20MB 以内。
fn feed(app: &AppHandle, paths: &[PathBuf]) -> usize {
    let mut n = 0;
    for p in paths.iter().take(5) {
        let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
        let mime = match ext.as_str() {
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "webp" => "image/webp",
            _ => continue,
        };
        if fs::metadata(p).map(|m| m.len() > 20 << 20).unwrap_or(true) {
            continue;
        }
        let Ok(bytes) = fs::read(p) else { continue };
        let _ = app.emit_to(
            "main",
            "sense://clip",
            Clip { b64: base64::engine::general_purpose::STANDARD.encode(bytes), mime, source_app: "桌宠".into(), copied: false },
        );
        n += 1;
    }
    n
}

/// `open -a "Sense 口袋" --args --scan-check`：在 app 自己的权限下试读一次履历，结果写到 scan-check.txt 后退出。
/// 只写 profile 名、条数和错误，不写任何标题或网址。
fn scan_check() {
    let (own, skipped) = own_profiles();
    let mut out = format!("own={} skipped={}\n", own.len(), skipped.len());
    out += &format!("local_state: {}\n", fs::read_to_string(chrome_dir().join("Local State")).map(|t| format!("ok {} bytes", t.len())).unwrap_or_else(|e| e.to_string()));
    out += &format!("chrome_dir: {}\n", fs::read_dir(chrome_dir()).map(|d| format!("ok {} entries", d.count())).unwrap_or_else(|e| e.to_string()));
    for (dir, name) in &own {
        let (mut agg, mut seen, mut dropped) = (HashMap::new(), 0u64, 0u64);
        let r = read_history(Path::new(dir), name, 0, &mut agg, &mut seen, &mut dropped);
        out += &format!("{name}: {} seen={seen}\n", r.err().unwrap_or_else(|| "ok".into()));
    }
    let _ = fs::write(data_dir().join("scan-check.txt"), out);
}

pub fn run() {
    if std::env::args().any(|a| a == "--scan-check") {
        scan_check();
        return;
    }
    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, sc, ev| {
                    let mine = SNAP_KEY.lock().map(|k| k.as_ref() == Some(sc)).unwrap_or(false);
                    if mine && ev.state() == ShortcutState::Pressed {
                        trace("snap key pressed".into());
                        snap(app.clone());
                    }
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            load_state, save_state, snap_now, scan_history, run_claude, scan_context, clip_watch, clip_img_watch, set_snap_key, engine_models, voice::voice_start, voice::voice_say, voice::voice_stop, voice::voice_server, charmem::char_mem_load, charmem::char_mem_write, charmem::char_mem_clear, save_png, open_url, scan_youtube,
            menu_toggle, menu_hide, menu_fit, show_main, quit_app, pet_show, pet_drag, trace
        ])
        .setup(move |app| {
            voice::server_boot();
            // 页面读到自己存的键后会再调一次 set_snap_key；这里先把默认键挂上。
            if let Err(e) = set_snap_key(app.handle().clone(), String::new()) {
                eprintln!("截图键没注册上：{e}");
            }
            // 右键还是原生菜单，左键弹快捷菜单。
            let show = MenuItem::with_id(app, "show", "打开口袋", true, None::<&str>)?;
            let shot = MenuItem::with_id(app, "snap", "截图进口袋", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &shot, &quit])?;
            let mut tray = TrayIconBuilder::with_id("sense")
                .tooltip("Sense 口袋")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, ev| match ev.id.as_ref() {
                    "show" => toggle_main(app),
                    "snap" => snap(app.clone()),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, ev| {
                    if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, rect, .. } = ev {
                        let app = tray.app_handle();
                        let sf = app.get_webview_window("menu").and_then(|m| m.scale_factor().ok()).unwrap_or(2.0);
                        let p = rect.position.to_physical::<f64>(sf);
                        let s = rect.size.to_physical::<f64>(sf);
                        toggle_menu_at(app, p.x, p.y, s.width, s.height);
                    }
                });
            if let Some(icon) = app.default_window_icon() {
                tray = tray.icon(icon.clone());
            }
            tray.build(app)?;
            #[cfg(target_os = "macos")]
            clipboard::start(app.handle().clone(), app.config().identifier.clone());
            Ok(())
        })
        .on_window_event(|w, ev| match (w.label(), ev) {
            // 关窗只是收进菜单栏，口袋还在。
            ("main", tauri::WindowEvent::CloseRequested { api, .. }) => {
                let _ = w.hide();
                api.prevent_close();
            }
            ("menu", tauri::WindowEvent::Focused(false)) => hide_menu(w.app_handle()),
            ("pet", tauri::WindowEvent::Moved(p)) => {
                let _ = w.emit_to("main", "sense://cmd", serde_json::json!({"cmd": "petpos", "arg": {"x": p.x, "y": p.y}}));
            }
            ("pet", tauri::WindowEvent::DragDrop(d)) => {
                let (kind, n) = match d {
                    tauri::DragDropEvent::Enter { .. } => ("enter", 0),
                    tauri::DragDropEvent::Leave => ("leave", 0),
                    tauri::DragDropEvent::Drop { paths, .. } => ("drop", feed(w.app_handle(), paths)),
                    _ => return,
                };
                let _ = w.emit_to("pet", "sense://pet-drag", serde_json::json!({"kind": kind, "n": n}));
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("Sense 口袋启动失败")
        .run(|_, ev| {
            if let tauri::RunEvent::Exit = ev {
                voice::server_stop();
            }
        });
}
