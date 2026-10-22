# 0020 · 口袋 app 用 Tauri v2 做；引擎先留在 Claude Code

日期：2026-10-22 · 状态：已定 · 细化 0018 的"macOS 口袋 app"

## 决策
- 口袋 app 用 Tauri v2：界面就是现有的口袋网页（TypeScript / 原生 JS），系统能力用 Rust 薄壳和官方插件：全局热键 ⌃⌘A 截图、菜单栏图标、读本机浏览器履历与书签、把口袋状态写成本地文件。
- 截图热键是 ⌃⌘A（作者 2026-10-22 更正）。与微信 Mac 版的默认截图键冲突，app 里可改。
- 引擎（读日志、生成卡、出一手）第一步仍由 Claude Code 里的 subagent 执行（0014），写出 `m0/pocket.json`；app 读它，并把作者的操作写到 `~/Library/Application Support/Sense/pocket-state.json`，Claude Code 读回。之后再把引擎做成随 app 运行的 Node 进程。
- 浏览器履历由 app 在本机读取：只读作者本人的 profile（按 Local State 的名字排除他人），副本读完即删，登录、支付、银行、医疗、签证、后台类页面在读取边界丢弃，只留标题、域名、次数、周数，不留 URL（书签除外）。

## 原因
- 作者 2026-10-22："可以"（同意做成 Tauri app）；"说错了是 ctrl command A"。
- 网页做不到全局热键、截图、读本机文件；Claude Code 里的 agent 读 Chrome 目录会被权限分类器拦下，由作者自己装的 app 读才是正当路径。
- Tauri 能直接复用网页界面，打包几 MB；Electron 太重，Swift 要重写界面。

## 影响
SENSE.md 5.1、10.1、13；新增 `app/`。
