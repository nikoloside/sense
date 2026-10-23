# 0030 · 引擎可选 Claude / DeepSeek；桌上的角色手动选；语音直接做对话

日期：2026-10-23 · 状态：已定 · 补充 0025（桌宠）、docs/art/characters.md

## 已定（作者说的）
- 桌上站谁，看引擎**此刻**用的是哪个模型。现在只有 Claude，所以只会出现 Claude；要让 DeepSeek 出场，就得先给引擎加上 DeepSeek。
- 语音直接做对话，不先做「只播报」。
- （同日改）桌上站谁**只由作者手动选**，不跟着引擎或通话自动换：来回自动切换看着怪。每个角色再出一个男版。

## 1. 引擎加 DeepSeek
- 设置页多一个「引擎用哪个模型」：Claude（默认）/ DeepSeek。
- **DeepSeek 不另写一套 agent**：仍然跑同一个 `claude -p`（同样的提示词、同样只读的工具白名单），只是把它指到 DeepSeek 的 Anthropic 兼容接口：
  - 环境变量设 `ANTHROPIC_BASE_URL=https://api.deepseek.com/anthropic`。
  - `ANTHROPIC_AUTH_TOKEN` 设成作者的 DeepSeek key。
  - 模型设成 `ANTHROPIC_MODEL`。
  - 这样读会话日志、读图、Chrome 只读页这些工具都在本机执行，换模型不换行为。
- **key 只放在本机**：`~/Library/Application Support/Sense/config.json` 的 `deepseek_api_key`，权限 600，不进仓库。由作者自己填，Sense 不去别的项目里找 key。
- **联网**：DeepSeek 的 Anthropic 兼容接口原生支持 Claude Code 的 WebSearch，搜索在 DeepSeek 服务端用它自家的搜索引擎执行（api-docs.deepseek.com，Claude Code 接入页）。WebFetch、Read、Chrome 这些工具本来就在本机执行。所以选 DeepSeek 时，联网的事也由 DeepSeek 跑，不换回 Claude，也不用另写搜索服务。
  - 作者说的「DSH」是 DeepSeek Harness（命令行 `dsh`），DeepSeek 自己的 agent 框架。Sense 不换成它：同一个 `claude -p` 换后端就够了，提示词、工具白名单、只读边界都不用动。
  - 环境变量照 DeepSeek 的接入页：`ANTHROPIC_BASE_URL`、`ANTHROPIC_AUTH_TOKEN`，各档模型都设成 `deepseek-flash`，并关掉非必要流量。
  - 搜索按次计费（2026-11-01 起），费用记在作者自己的 DeepSeek 账户上。
- 每次调用在 engine.log 里记下用的是哪个模型，以及首字和总耗时。

## 2. 谁站在桌上
- 作者在快捷菜单「桌上是谁」里选：口袋精或 Claude（之后加 DeepSeek 和三个男版）。选了就一直是这位，引擎在跑时这位做 busy 动作，不换人。
- 通话的对象仍是引擎当前的模型；桌上那位不因为通话换人。
- 气泡说的是某张卡（吃到新卡、hover 时的 NEW）时可以点，点了打开那张卡。
- 口袋挂在 Claude、DeepSeek 腰间；进卡时鼓一下，碎卡时抖一下。
- 动作沿用现在的 moods：idle、busy、eat、cheer、crack、alert、sleep，再加 wave、talk。图按 docs/art/characters.md 由 Codex 出。

## 3. 语音对话
- **Sense 是 voice-live 的客户端**，不复制它的代码：
  - 连 `ws://…/ws?brain=relay&character=<id>&language=Chinese&voice=<qwen 音色>`。
  - 麦克风采 PCM16 16k 送上去，收分段 PCM 播放，回报 `played` / `interrupted`。
  - 打断、附和、填充音全由 voice-live 处理。
- **台词由 Sense 自己出（relay）**：voice-live 把识别出的话以 `character_prompt` 交给 Sense，Sense 用当前引擎的模型回答，并以 `character_delta` 流回去。
  - 一通电话开一个常驻的 `claude -p --input-format stream-json --output-format stream-json` 进程，不给任何工具（`--tools ""`），接通时就起，避免每句冷启动。Claude 用快模型（haiku），DeepSeek 用 deepseek-flash。
  - 实测（Claude haiku）：第一句首字约 3.9 秒（进程预热），之后约 1.2 秒。
  - DeepSeek：直接调流式接口。
  - 角色的系统提示：`characters/<id>/soul.md` + 一份口袋摘要（卡数、最近进的卡、快碎的卡、待回答的问题、银卡的最新动向）。摘要只有标题和计数，不含证据原文。
- **v1 能做的**：
  - 问口袋里的事，比如「最近有什么新的」「来自深渊怎么样了」。
  - 用嘴回答「要问你的」那些是 / 不是题：角色先复述一遍，你说「对」才记。
  - 闲聊。
  - **不能**用嘴碾碎卡、改属性、发东西。
- **入口**：快捷菜单顶上的「和 Claude / DeepSeek 说话」，以及设置 → 引擎 → 语音通话里的「打电话」，再按一次挂断。只在通话中开麦克风。
- **语言与声音**：中文、日语、英语，跟着 app 语言走。voice-live 跑在**本机**，用 fish 分支（Fish 识别 + Fish 流式声音）。每个角色中、日、英各一个 Fish 音色，从 worldlines-play 的声线清单里挑，写在 `characters/<id>/voice.md`。
- **服务地址**：默认 `ws://127.0.0.1:8777`，设置页可以改。
- **隐私**：
  - Sense 不存录音，也不存逐字稿；通话结束只留一条「打过一次电话，多长，回答了几道题」。
  - voice-live 那一侧：Sense 的通话带 `log=meta`，voice-live 只记时长、状态和计数，不记任何一句话（voice-live 的 fish 分支新增这个参数）。
  - 大脑进程不留会话（`--no-session-persistence`），挂断就结束。
- **用嘴回答是非题**：角色先复述，作者说「对」之后，角色在那句话末尾写一个标记〔记 Q2=是〕；Sense 截下标记、记下答案，标记本身不送去念。没有标记就不记。

## 角色正本
- 每个角色一份 `characters/<id>/soul.md`：她是谁、怎么说话、**只知道口袋摘要里的东西**、不做什么；再加一份 `voice.md`。格式沿用 worldlines-characters 的 soul.md。

## 实装位置
- 引擎：`app/src-tauri/src/lib.rs` 的 `run_claude(model)`、`use_deepseek`；设置 → 引擎 → 用哪个模型。
- 语音大脑：`app/src-tauri/src/voice.rs`；页面里的通话客户端在模板的 voice call 段；麦克风说明在 `app/src-tauri/Info.plist`。
- 桌宠：`m0/pet.html` 按作者选的角色和状态换帧，通话时按播放音量切口型帧。
