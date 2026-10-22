# 0011 · 技术栈用 TypeScript

日期：2026-10-20 · 状态：已定

## 决策
Sense 用 TypeScript 实现（Node 运行时）。嵌入与降维走 API 或小库，不引入 Python。

## 原因
- MCP SDK、单文件 HTML 渲染、CLI 工具链在 TS 侧最顺手。
- Python 的优势只在 UMAP 与数据处理，可用 API / 小库替代。
- 用户 2026-10-20 确认（"ts 可以"）。

## 影响
SENSE.md 第 12 节问题 1 关闭；M1 起点按 TS 校准。
