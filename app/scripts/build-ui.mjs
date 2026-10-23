// 把 m0/pocket.template.html 与 m0/pocket.json 拼成 app/ui/index.html。
// Tauri 窗口没有 artifact 的发布骨架，这里补上 doctype、charset、viewport 和同样的小 reset。
// 快捷菜单（m0/menu.html）和桌宠（m0/pet.html）是另外两个窗口，配色从主模板的 THEMES 段原样拷过去。
import { readFileSync, writeFileSync, mkdirSync, readdirSync, copyFileSync, existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
const here = dirname(fileURLToPath(import.meta.url));
const root = resolve(here, "../..");
const tpl = readFileSync(resolve(root, "m0/pocket.template.html"), "utf8");
const data = readFileSync(resolve(root, "m0/pocket.json"), "utf8");
JSON.parse(data);
if (tpl.split("/*__POCKET__*/").length !== 2) throw new Error("template must contain exactly one /*__POCKET__*/");
const themes = tpl.match(/\/\*THEMES\{\*\/([\s\S]*?)\/\*\}THEMES\*\//);
if (!themes) throw new Error("template must contain one /*THEMES{*/ … /*}THEMES*/ block");
const shell = (body) => `<!doctype html><html lang="zh"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1,viewport-fit=cover"><style>:root{color-scheme:light;padding-top:env(safe-area-inset-top,0px);padding-bottom:env(safe-area-inset-bottom,0px)}body{margin:0;font:14px system-ui,sans-serif}img{max-width:100%}[hidden]{display:none!important}</style></head><body>${body}</body></html>`;
mkdirSync(resolve(here, "../ui"), { recursive: true });
const out = (name, html) => { writeFileSync(resolve(here, "../ui", name), html); console.log("ui/" + name, html.length, "chars"); };
// 角色正本（characters/<id>/soul.md + voice.md 里的中日英 reference_id），给语音通话用（0030）。
const LANG = { "中文": "zh", "日本語": "ja", "English": "en" };
const souls = {};
for (const id of ["claude", "deepseek", "pouch"]) {
  const dir = resolve(root, "characters", id);
  if (!existsSync(dir)) continue;
  const soul = readFileSync(resolve(dir, "soul.md"), "utf8").replace(/^---[\s\S]*?---\n/, "").trim();
  const voices = {};
  for (const m of readFileSync(resolve(dir, "voice.md"), "utf8").matchAll(/^\| (中文|日本語|English) \| `([0-9a-f]{32})`/gm)) voices[LANG[m[1]]] = m[2];
  souls[id] = { soul, voices };
}
if (tpl.split("/*__SOULS__*/").length !== 2) throw new Error("template must contain exactly one /*__SOULS__*/");
out("index.html", shell(tpl.replace("/*__POCKET__*/", data.replaceAll("</script", "<\\/script")).replace("/*__SOULS__*/{}", JSON.stringify(souls).replaceAll("</script", "<\\/script"))));
for (const name of ["menu.html", "pet.html"]) {
  const src = readFileSync(resolve(root, "m0", name), "utf8");
  if (!src.includes("/*__THEMES__*/")) throw new Error(name + " must contain /*__THEMES__*/");
  out(name, `<!doctype html><html lang="zh"><head><meta charset="utf-8">${src.replace("/*__THEMES__*/", themes[1])}</html>`);
}
// 角色图（docs/art/characters.md）：桌宠用 256 的那份。
for (const id of ["claude", "deepseek", "pouch", "claude-m", "deepseek-m", "pouch-m", "charm"]) {
  const from = resolve(root, "assets/characters", id, "256");
  if (!existsSync(from)) continue;
  const to = resolve(here, "../ui/characters", id);
  mkdirSync(to, { recursive: true });
  for (const f of readdirSync(from).filter((f) => f.endsWith(".png"))) copyFileSync(resolve(from, f), resolve(to, f));
}
