# 0007 · 形态：本地 MCP 服务器 ＋ CLI ＋ 生成的可视化页

日期：2026-10-20 · 状态：已定 · 替代 v0.5 §5 的 Web 服务与每用户远程 agent

## 决策
Sense 第一版是一个本地进程：`sense` CLI 做批处理，`sense mcp` 以 stdio 供 Claude Code / Codex 调用，`sense render` 生成自包含 HTML 页。数据全在 `~/.sense/`。不做账户、上传和托管。

## 原因
- 目标用户活在 Claude Code / Codex 里，context 已经在本机，MCP 让他们的 agent 直接用上自己的 sense。
- 参照 ai-sessions-mcp、memex 这类工具，本地读取和检索已被验证可行。
- 没验证闭环前不背服务、账户和隐私说明的成本。

## 影响
BACKLOG 以 CLI 跑通闭环为先；Web 托管是以后把同一个库搬上去。详见 docs/agent-shape.md。
