# Sense

**Your personal taste agent.** · [English](README.en.md) · [日本語](README.ja.md)
懂你的品味，帮你发现值得喜欢的东西。
あなたの好みを理解し、次の「好き」を見つける。

**下载 · Download：[ludics.space](https://ludics.space)** · Demo：[ludics.space/demo](https://ludics.space/demo/) · MIT License

Sense 是一个理解你个人品味的 Agent。它从你与 AI 的对话、收藏和喜欢的作品中学习，通过你的确认和纠正，逐渐理解你的选择标准，再帮你寻找文章、游戏、作品与创作者。你可以随时查看、修改和带走它对你的理解。

- 品牌名：**Sense**
- 定位：**Personal Taste Agent**（能力用 Taste 解释）
- 核心体验：**发现 ＋ 纠正 ＋ 记住**
- 核心闭环：**context → sense context → 可视化 → 有价值的推荐**
- 形态：**本地 MCP 服务器 ＋ CLI ＋ 生成的可视化页**

## 文档

| 文件 | 内容 |
|---|---|
| [docs/SENSE.md](docs/SENSE.md) | **唯一的定义文档**：一句话、论点、核心闭环、形态、数据模型、引擎规则、体验走查、隐私、范围、成功标准、待决问题、里程碑、术语表 |
| [docs/framework.html](docs/framework.html) | 系统框架图解：总循环、卡的档位、agent 引擎、设计语言、数据边界。从 SENSE.md 整理，冲突时以 SENSE.md 为准 |
| [docs/decisions/](docs/decisions/) | 决策日志，每条一个文件 |
| [docs/references.md](docs/references.md) | 参照物：ai-sessions-mcp、memex、Boardy、Paxel |
| [schema/](schema/) · [examples/](examples/) | 档案的 JSON Schema 与示例（Field / Item / SenseContext 的 schema 在 M2 前定稿） |
| [m0/](m0/) | M0 纸面原型：subagent 从作者的 Claude Code / Codex 会话日志写出的 16 张卡与手机卡片 demo |
| [BACKLOG.md](BACKLOG.md) | 任务清单，按 SENSE.md 第 13 节里程碑组织 |
| [docs/design-v0.5-archive.md](docs/design-v0.5-archive.md) | 2026-10-19 的完整设计稿 v0.5，已归档 |

## 下载和运行

- **安装包**：[GitHub Releases](https://github.com/nikoloside/sense/releases/latest) 里的 `.dmg`，只支持 macOS（Apple 芯片）。没有经过 Apple 公证，第一次打开会被拦：到「系统设置 › 隐私与安全性」点「仍要打开」。
- **需要**：已安装并登录的 [Claude Code](https://claude.com/claude-code)（`claude` 命令）。所有 agent 调用都是本机的 `claude -p`，用你自己的账号。读听歌页面要 Claude in Chrome 扩展。
- **自己编译**：见 [app/README.md](app/README.md)。签名证书用环境变量 `APPLE_SIGNING_IDENTITY` 传入，不写进仓库；不传就是临时签名，每次编译后 macOS 都会重新要权限。
- **只读谁的数据**：只读你自己的 Chrome profile。别人的 profile 名单写在本机 `~/Library/Application Support/Sense/config.json` 的 `exclude_profiles` 里，不进仓库。

## 关于仓库里的数据

`m0/pocket.json`（以及 demo）是作者本人的真实口袋：从作者的 Claude Code 会话日志、浏览器和 YouTube Music 记录里整理出来的卡，作者同意原样公开。第三方的名字已经去掉。你装上之后，口袋从你自己的数据重新长出来，存在 `~/Library/Application Support/Sense/`，不上传。

## 状态

2026-10-20 起项。2026-10-23 开源，桌面版（Tauri）可用。定义文档 v1.2（2026-10-21）：口袋 app 为主面，七种卡、属性、石铜银金四个品级，agent 卡槽、融合、委托、抽卡 survey（决策 0016–0019）。M0 第二叠卡由 subagent 从作者的会话日志写出，见 m0/。下一步 M0.5：按七种卡出第一叠，再按第 13 节统一实装。
