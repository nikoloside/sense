//! 角色的成长层（0036）：本机 characters/<id>/ 文件夹。正本 soul.md 在仓库里，这里只放她经历出来的东西。
//!
//! knowledge.md 她知道的事（关于他 / 关于世界 / 想问他的），每条带日期和渠道
//! opinions.md  她的主见，每条指向一次经历
//! diary/YYYY-MM-DD.md  她的经历
//! stories/<名字>.md     和他一起编的故事的设定集
//!
//! 只认这几种路径；不落通话全文（页面挂断时交给 agent 整理，只把整理出的几句写进来）。

use std::fs;
use std::path::PathBuf;

fn dir(id: &str) -> Result<PathBuf, String> {
    if id.is_empty() || id.len() > 32 || !id.chars().all(|c| c.is_ascii_lowercase() || c == '-') {
        return Err(format!("角色 id 不对：{id}"));
    }
    let d = super::data_dir().join("characters").join(id);
    fs::create_dir_all(d.join("diary")).map_err(|e| e.to_string())?;
    fs::create_dir_all(d.join("stories")).map_err(|e| e.to_string())?;
    Ok(d)
}

/// 允许写的相对路径：knowledge.md、opinions.md、diary/2026-10-25.md、stories/<名字>.md（名字不含路径符号）。
fn allowed(rel: &str) -> bool {
    if rel == "knowledge.md" || rel == "opinions.md" {
        return true;
    }
    if let Some(d) = rel.strip_prefix("diary/").and_then(|x| x.strip_suffix(".md")) {
        let b = d.as_bytes();
        return b.len() == 10 && b[4] == b'-' && b[7] == b'-' && d.chars().filter(|c| c.is_ascii_digit()).count() == 8;
    }
    if let Some(n) = rel.strip_prefix("stories/").and_then(|x| x.strip_suffix(".md")) {
        let n = n.trim();
        return !n.is_empty() && n.chars().count() <= 40 && !n.contains(['/', '\\', '.', ':']) && !n.starts_with('-');
    }
    false
}

fn read(p: PathBuf) -> String {
    fs::read_to_string(p).unwrap_or_default()
}

fn list_md(d: PathBuf) -> Vec<(String, String)> {
    let mut v: Vec<(String, String)> = fs::read_dir(&d)
        .map(|it| {
            it.filter_map(Result::ok)
                .filter_map(|e| {
                    let n = e.file_name().to_string_lossy().to_string();
                    n.strip_suffix(".md").map(|s| (s.to_string(), read(e.path())))
                })
                .collect()
        })
        .unwrap_or_default();
    v.sort_by(|a, b| b.0.cmp(&a.0));
    v
}

#[tauri::command]
pub fn char_mem_load(id: String) -> Result<serde_json::Value, String> {
    let d = dir(&id)?;
    let diary: Vec<_> = list_md(d.join("diary")).into_iter().take(14).map(|(k, v)| serde_json::json!({"date": k, "text": v})).collect();
    let stories: Vec<_> = list_md(d.join("stories")).into_iter().map(|(k, v)| serde_json::json!({"name": k, "text": v})).collect();
    Ok(serde_json::json!({
        "id": id,
        "knowledge": read(d.join("knowledge.md")),
        "opinions": read(d.join("opinions.md")),
        "diary": diary,
        "stories": stories,
    }))
}

/// 整份写（append=true 时接在末尾）。
#[tauri::command]
pub fn char_mem_write(id: String, path: String, content: String, append: Option<bool>) -> Result<(), String> {
    if !allowed(&path) {
        return Err(format!("不能写这个路径：{path}"));
    }
    if content.len() > 200_000 {
        return Err("太长了".into());
    }
    let p = dir(&id)?.join(&path);
    if append.unwrap_or(false) {
        use std::io::Write;
        let mut f = fs::OpenOptions::new().create(true).append(true).open(&p).map_err(|e| e.to_string())?;
        f.write_all(content.as_bytes()).map_err(|e| e.to_string())
    } else {
        fs::write(&p, content).map_err(|e| e.to_string())
    }
}

/// 清空一个角色的成长层（正本不动）。
#[tauri::command]
pub fn char_mem_clear(id: String) -> Result<(), String> {
    let d = dir(&id)?;
    fs::remove_dir_all(&d).map_err(|e| e.to_string())?;
    super::trace(format!("char mem cleared id={id}"));
    Ok(())
}
