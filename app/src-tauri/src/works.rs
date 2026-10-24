//! 作品（0035）：派活的产出。正文 / 图存在做它的角色的文件夹 characters/<id>/works/<作品 id>.<md|png>，
//! 另存一份到 ~/Documents/Sense/作品/ 方便拿去别处用。出图走 OpenAI gpt-image-1，钥匙在本机 config.json 的
//! `image_api_key`（作者自己填；不在仓库、不进日志、不出现在进程参数里）。只产出，不碰外面。

use base64::Engine;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn image_key() -> Option<String> {
    super::config().get("image_api_key").and_then(|k| k.as_str()).map(|k| k.trim().to_string()).filter(|k| !k.is_empty())
}

fn safe_id(s: &str, max: usize) -> Result<String, String> {
    if s.is_empty() || s.len() > max || !s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        return Err(format!("名字不对：{s}"));
    }
    Ok(s.to_string())
}

fn works_dir(who: &str) -> Result<PathBuf, String> {
    let who = safe_id(who, 32)?;
    let d = super::data_dir().join("characters").join(who).join("works");
    fs::create_dir_all(&d).map_err(|e| e.to_string())?;
    Ok(d)
}

#[tauri::command]
pub fn image_ready() -> bool {
    image_key().is_some()
}

/// 出一张图，返回 PNG 的 base64。钥匙经 curl 的 stdin 配置传进去，不出现在命令行。
#[tauri::command]
pub async fn image_gen(prompt: String) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let key = image_key().ok_or("还没接出图服务：在 ~/Library/Application Support/Sense/config.json 里加 \"image_api_key\"（OpenAI 的 key）。")?;
        let body = serde_json::json!({ "model": "gpt-image-1", "prompt": prompt, "size": "1024x1024", "quality": "medium", "n": 1 });
        let tmp = super::data_dir().join(format!("img-req-{}.json", std::process::id()));
        fs::write(&tmp, body.to_string()).map_err(|e| e.to_string())?;
        let t0 = std::time::Instant::now();
        let child = Command::new("curl")
            .args(["-sS", "--max-time", "180", "-K", "-", "-H", "Content-Type: application/json", "--data-binary"])
            .arg(format!("@{}", tmp.display()))
            .arg("https://api.openai.com/v1/images/generations")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();
        let out = child.and_then(|mut c| {
            c.stdin.take().unwrap().write_all(format!("header = \"Authorization: Bearer {key}\"\n").as_bytes())?;
            c.wait_with_output()
        });
        let _ = fs::remove_file(&tmp);
        let out = out.map_err(|e| e.to_string())?;
        let v: serde_json::Value = serde_json::from_slice(&out.stdout).map_err(|_| format!("出图服务没回话：{}", String::from_utf8_lossy(&out.stderr).trim()))?;
        super::trace(format!("image_gen ms={} ok={}", t0.elapsed().as_millis(), v["data"][0]["b64_json"].is_string()));
        if let Some(b) = v["data"][0]["b64_json"].as_str() {
            return Ok(b.to_string());
        }
        Err(format!("出图失败：{}", v["error"]["message"].as_str().unwrap_or("不知道为什么")))
    })
    .await
    .map_err(|e| e.to_string())?
}

/// 存一件作品：text 存 .md，png（base64）存 .png。返回 Documents 里那份的路径。
#[tauri::command]
pub fn work_save(who: String, id: String, name: String, text: Option<String>, png: Option<String>) -> Result<String, String> {
    let id = safe_id(&id, 40)?;
    let d = works_dir(&who)?;
    let (ext, bytes) = match (text, png) {
        (Some(t), _) => ("md", t.into_bytes()),
        (None, Some(b)) => ("png", base64::engine::general_purpose::STANDARD.decode(b.split(',').last().unwrap_or("")).map_err(|e| e.to_string())?),
        _ => return Err("没有内容".into()),
    };
    fs::write(d.join(format!("{id}.{ext}")), &bytes).map_err(|e| e.to_string())?;
    let name: String = name.chars().filter(|c| !"/\\:*?\"<>|".contains(*c) && !c.is_control()).take(40).collect();
    let docs = PathBuf::from(std::env::var("HOME").unwrap_or_default()).join("Documents/Sense/作品");
    fs::create_dir_all(&docs).map_err(|e| e.to_string())?;
    let p = docs.join(format!("{id} {}.{ext}", name.trim()));
    fs::write(&p, &bytes).map_err(|e| e.to_string())?;
    Ok(p.display().to_string())
}

/// 读一件作品：.md 返回原文，.png 返回 data URL。
#[tauri::command]
pub fn work_read(who: String, id: String, png: bool) -> Result<String, String> {
    let id = safe_id(&id, 40)?;
    let p = works_dir(&who)?.join(format!("{id}.{}", if png { "png" } else { "md" }));
    if png {
        let b = fs::read(&p).map_err(|e| e.to_string())?;
        Ok(format!("data:image/png;base64,{}", base64::engine::general_purpose::STANDARD.encode(b)))
    } else {
        fs::read_to_string(&p).map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub fn work_reveal(path: String) {
    let docs = PathBuf::from(std::env::var("HOME").unwrap_or_default()).join("Documents/Sense/作品");
    let p = PathBuf::from(&path);
    if p.starts_with(&docs) && p.exists() {
        let _ = Command::new("open").arg("-R").arg(&p).status();
    }
}
