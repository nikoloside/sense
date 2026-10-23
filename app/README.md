# Sense 口袋 · 桌面版（Tauri v2）

界面就是 `m0/pocket.template.html`；构建时和 `m0/pocket.json` 拼成 `app/ui/index.html`。Rust 只做网页做不到的事（决策 0020、0021）。

## 跑起来

```sh
cd app
cargo tauri dev            # 开发：改模板后重启即可
cargo tauri build --bundles app   # 打包：src-tauri/target/release/bundle/macos/Sense 口袋.app
cargo tauri build --bundles dmg   # 发布用的 .dmg
# 用固定证书签名，macOS 的权限才不会每次编译后失效：
APPLE_SIGNING_IDENTITY="Apple Development: 你的名字 (XXXXXXXXXX)" cargo tauri build --bundles app
```

需要：Rust、`cargo install tauri-cli`、Node（只用来拼界面）、已登录的 Claude Code（`claude` 命令）。

## 第一次要给的权限

| 功能 | macOS 会问 | 不给会怎样 |
|---|---|---|
| 截图键（默认 ⌃⇧⌘A） | 隐私与安全性 → 屏幕录制 | 截出来只有桌面背景 |
| 截图键 | 不用问；别设成 ⌃⌘A：那是微信的截图键，两边都注册时系统不报错，先注册的吃掉按键 | 在「入口 → 截图」里「换截图键」；或者照旧用微信、系统截图，打开「收剪贴板里的截图」 |
| 收剪贴板里的截图 | 不用问；默认关 | 剪贴板里出现新图（不是 Finder 里复制的文件）就问一句，点「放进口袋」才收 |
| 语音通话 | 隐私与安全性 → 麦克风（第一次打电话时问） | 只在通话中开；要本机开着 voice-live（设置 → 引擎 → 语音通话里有命令） |
| 引擎用 DeepSeek | 不用问；在 config.json 填 `deepseek_api_key` | key 只留在本机；联网搜索由 DeepSeek 服务端执行 |
| 剪贴板 | 不用问 | 默认关，入口页里打开 |
| 浏览器履历 | 隐私与安全性 → 完全磁盘访问权限，打开「Sense 口袋」 | 读到 0 条，品味整理只用听歌页面 |
| 听歌页面 | Chrome 里装 Claude in Chrome 扩展；第一次访问每个网站时扩展会问 | 听歌页面框里显示失败原因 |

## 它做什么

| 地方 | 真的做了什么 |
|---|---|
| 截图键 / 菜单栏「截图进口袋」 | `screencapture -i -x` 截到临时文件 → 读进来就删 → 页面让 Claude 认图（`claude -p` ＋ Read）→ 成卡飞进口袋 |
| 入口 · 剪贴板 | 0.5 秒轮询 NSPasteboard；跳过密码管理器标记、聊天与邮件 app、像密钥 / 邮箱 / 电话的内容；记来源 app 与"之后切到"的 app |
| 入口 · 浏览器 | 复制自己的 Chrome profile 的 History 到临时目录，只读查询后删；敏感页在读取边界丢弃；跨 ≥3 周且 ≥5 次才算候选 |
| 入口 · 文件 | 下载、桌面、文稿顶层最近 120 天的文件名；Steam 游戏目录名；Claude Code / Codex 会话个数。不打开文件 |
| 入口 · 会话日志 | `claude -p --add-dir ~/.claude/projects --add-dir ~/.codex/sessions`，只读工具，按 SENSE.md 7.1 规则出卡 |
| 追踪台 | 到期的座位自动跑：联网查更新 / 挖信息 / 排进抽卡 |
| 用例 · 做、牌桌 · 出一手 | `claude -p` ＋ WebSearch，结果标"联网查过"；失败如实报错，不退回假数据 |
| 口袋状态 | `~/Library/Application Support/Sense/pocket-state.json`，Claude Code 读这个文件调门槛、改属性 |

`claude -p` 都带 `--no-session-persistence --strict-mcp-config`，在 `~/Library/Application Support/Sense/agent` 里跑，不碰任何项目。用的是你自己 Claude 账号的额度：额度用完时，页面会显示 Claude 返回的原话。

## 不读什么

- 别人的 Chrome profile：在 `src-tauri/src/lib.rs` 的 `OTHER_PEOPLE` 里按名字排除。
- 登录、支付、银行、医疗、签证、邮件、后台页：见 `BLOCK_HOST`、`BLOCK_TITLE`。
- 文件名带护照、在留、银行、请求书、税、合同等字样的文件：见 `BLOCK_FILE`。
