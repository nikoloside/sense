# 0008 · 核心闭环是 context → sense context → 可视化 → 推荐；平台连接器不是核心

日期：2026-10-20 · 状态：已定

## 决策
闭环：收集某个 field 的 context（本地优先，远程扩展）→ 整理成可快速排序检索的 KB → 用从本地提炼的 sense 排序 → 可视化你的位置与独特 → 给出人 / 作品 / 项目推荐 → 纠正写回。
Steam、Netflix、Reddit、Spotify 这类连接器是特定 field 的远程来源，排在引擎跑通之后。

## 原因
连接器拿到的只是消费记录；没有 field KB 就无法排序，也看不出独特性。先有引擎，连接器才有用。

## 影响
v1-scope.md 的"替你寻找"改为基于 field KB；四步体验不变。详见 docs/core-loop.md。
