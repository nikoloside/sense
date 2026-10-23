# 0030 · 引擎可选 Claude / DeepSeek，角色跟着引擎；语音直接做对话

日期：2026-10-23 · 状态：**草案，待定稿** · 补充 0025（桌宠）、docs/art/characters.md

## 已定（作者说的）
- 桌上站谁，看引擎**此刻**用的是哪个模型。现在只有 Claude，所以只会出现 Claude；要让 DeepSeek 出场，就得先给引擎加上 DeepSeek。
- 语音直接做对话，不先做「只播报」。

## 1. 引擎加 DeepSeek
- 设置页多一个「引擎用哪个模型」：Claude（默认）/ DeepSeek。
- **DeepSeek 不另写一套 agent**：仍然跑同一个 `claude -p`（同样的提示词、同样只读的工具白名单），只是把它指到 DeepSeek 的 Anthropic 兼容接口：
  - 环境变量设 `ANTHROPIC_BASE_URL=https://api.deepseek.com/anthropic`。
  - `ANTHROPIC_AUTH_TOKEN` 设成作者的 DeepSeek key。
  - 模型设成 `ANTHROPIC_MODEL`。
  - 这样读会话日志、读图、Chrome 只读页这些工具都在本机执行，换模型不换行为。
- **key 只放在本机**：`~/Library/Application Support/Sense/config.json` 的 `deepseek_api_key`，权限 600，不进仓库。由作者自己填，Sense 不去别的项目里找 key。
- **联网**：DeepSeek 没法用 WebSearch，因为那是 Anthropic 服务端执行的工具。但 DeepSeek 自己有 Search / Fetch 接口（作者说的「DSH」；第三方资料写的是 2026-10 开放的 `POST https://api.deepseek.com/v1/tools/search`，和聊天用同一个 key），所以联网的事也交给 DeepSeek 跑，不换回 Claude：
  - Sense 自己带一个只读的 MCP 小服务：`sense-pocket --mcp-search`，stdio 协议，只有 `search` 和 `fetch` 两个工具，转发到 DeepSeek 的接口。
  - 选 DeepSeek 时，`claude -p` 用 `--mcp-config` 挂上它，白名单把 `WebSearch` / `WebFetch` 换成 `mcp__sense__search` / `mcp__sense__fetch`。提示词里的「用 WebSearch 查」改成「用 search 工具查」。
  - fetch 只放行 https，不带 cookie。search 每次最多 10 条结果。
  - 实装前先对照 api-docs.deepseek.com 核对接口的路径、参数和计费。
  - 所以 DeepSeek 引擎下，桌上从头到尾站的都是 DeepSeek。
- 每次调用在 engine.log 里记下用的是哪个模型，以及首字和总耗时。

## 2. 谁站在桌上
- 引擎正在跑一件事 → 当前引擎模型对应的角色（claude / deepseek）。
- 正在语音通话 → 通话所用模型的角色。
- 都闲着 → 口袋精。
- 口袋挂在 Claude、DeepSeek 腰间；进卡时鼓一下，碎卡时抖一下。
- 动作沿用现在的 moods：idle、busy、eat、cheer、crack、alert、sleep，再加 wave、talk。图按 docs/art/characters.md 由 Codex 出。

## 3. 语音对话
- **Sense 是 voice-live 的客户端**，不复制它的代码：
  - 连 `ws://…/ws?brain=relay&character=<id>&language=Chinese&voice=<qwen 音色>`。
  - 麦克风采 PCM16 16k 送上去，收分段 PCM 播放，回报 `played` / `interrupted`。
  - 打断、附和、填充音全由 voice-live 处理。
- **台词由 Sense 自己出（relay）**：voice-live 把识别出的话以 `character_prompt` 交给 Sense，Sense 用当前引擎的模型回答，并以 `character_delta` 流回去。
  - Claude：开一个常驻的 `claude -p --input-format stream-json --output-format stream-json` 进程，用快模型，不给工具，避免每句冷启动。首字要实测，目标 2 秒内。
  - DeepSeek：直接调流式接口。
  - 角色的系统提示：`characters/<id>/soul.md` + 一份口袋摘要（卡数、最近进的卡、快碎的卡、待回答的问题、银卡的最新动向）。摘要只有标题和计数，不含证据原文。
- **v1 能做的**：
  - 问口袋里的事，比如「最近有什么新的」「来自深渊怎么样了」。
  - 用嘴回答「要问你的」那些是 / 不是题：角色先复述一遍，你说「对」才记。
  - 闲聊。
  - **不能**用嘴碾碎卡、改属性、发东西。
- **入口**：快捷菜单和桌宠上的「说话」，再按一次挂断。只在通话中开麦克风，菜单栏图标同时变色。
- **语言与声音**：中文、日语、英语，跟着 app 语言走。voice-live 跑在**本机**，用 fish 分支（Fish 识别 + Fish 流式声音）。每个角色中、日、英各一个 Fish 音色，从 worldlines-play 的声线清单里挑，写在 `characters/<id>/voice.md`。
- **服务地址**：默认 `ws://127.0.0.1:8777`，设置页可以改。
- **隐私**：
  - Sense 不存录音，也不存逐字稿；通话结束只留一条「打过一次电话，多长，回答了几道题」。
  - 但 voice-live 那一侧自己会写事件日志（含识别文字），这条要在 voice-live 里给 Sense 的通话关掉，或者接受。

## 角色正本
- 每个角色一份 `characters/<id>/soul.md`：她是谁、怎么说话、**只知道口袋摘要里的东西**、不做什么；再加一份 `voice.md`。格式沿用 worldlines-characters 的 soul.md。

## 待定
1. 三个角色的中日英音色已配好（`characters/<id>/voice.md`），等作者试听确认。
2. voice-live 的事件日志：为 Sense 通话关掉，还是保留？
