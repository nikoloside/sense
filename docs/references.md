# 参照物

*2026-10-20 核实，只读了页面，没有运行任何工具。维护状态未查。*

## 读本地 agent 会话的 MCP / 工具

- [ai-sessions-mcp (yoavf)](https://github.com/yoavf/ai-sessions-mcp)：Go；读 Claude Code `~/.claude/projects/*/*.jsonl`、Codex `~/.codex/sessions/` 与 `archived_sessions/`、Gemini CLI、opencode、Copilot CLI、Mistral Vibe；tools：`list_available_sources` / `list_sessions` / `search_sessions`（BM25）/ `get_session`；`claude mcp add --scope user --transport stdio ai-sessions -- ~/.aisessions/bin/aisessions`，Codex 用 `codex mcp add`。最接近 Sense 的 context 层。
- [memex (vimo-ai)](https://github.com/vimo-ai/memex)：Claude Code / Codex / OpenCode / Gemini 入一个库，FTS5 ＋ 向量混合检索，数据在 `~/.vimo`；lite 版是零依赖 CLI，不起服务。检索层可借鉴。
- [session-recall](https://mcpservers.org/servers/yoavf/ai-sessions-mcp)（同目录页提到）：FTS5 trigram ＋ 可选向量。
- [ChatCrystal (ZengLiangYi)](https://github.com/TensorBlock/awesome-mcp-servers/blob/main/docs/knowledge-management--memory.md)：本地 PKM，导入多工具会话，语义搜索与 MCP memory tools。
- [Unified History MCP (jmars)](https://glama.ai/mcp/servers/jmars/unified-history-mcp)：会话、会议记录、命令历史统一全文检索。

用户提到的"测试 AI native 的那个本地 MCP 工具"没有查到同名项目，以上是最接近的候选，待确认具体是哪一个。

## "找我的资料 → 推荐该认识谁"

- [Boardy](https://www.boardy.ai/)：AI 超级连接者，双向 opt-in 介绍，走 WhatsApp / 短信 / LinkedIn。
- [Same](https://www.indiehackers.com/product/same)：上传 CV / PDF，提取 "Professional DNA"，按协同点而非头衔提议连接。
- Superconnect：用户描述为"通过找我的论文，推荐要认识谁"，未查到同名产品页，以上两个是体验上最接近的。

## 竞品

- [Paxel (YC)](https://paxel.ycombinator.com/)：见 design-v0.5-archive.md 文末。
