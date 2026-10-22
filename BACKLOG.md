# Backlog

按 docs/SENSE.md 第 13 节的里程碑组织。**M1 起的代码在定义定稿后统一开工；M0 是手写卡片，不算代码。** 范围变化先改 SENSE.md。

## 讨论阶段（进行中）

- [x] 技术栈 → TypeScript（0011）
- [x] Sense 的定义：四要素、三层（0012）
- [x] 形态：CLI ＋ 卡片页，MCP 推迟到 v1.1（0013）
- [x] 引擎由 agent 执行、来源以会话日志为先（0014）
- [x] ~~卡只有两种（0015）~~ 撤回
- [x] 卡片系统并入 SENSE.md v1.2：七种卡与属性（0016）、石铜银金与可调门槛（0017）、口袋 app ＋ 截图 ＋ 剪贴板 ＋ 邮件 ＋ 浏览器全部扫（0018）、卡槽 ＋ 融合 ＋ 委托 ＋ 抽卡 survey（0019）
- [ ] 第一个正式 field：等 M0.5 的口袋里哪类卡最多、留下最多再定（日志建议 AI 角色与世界设计）
- [ ] 第 12 节剩余问题：向量存储、嵌入提供方、人的伦理开关
- [ ] 同步 Claude 文档到 SENSE.md v1.2（落后两版）
- [ ] schema：Judgment 加 scene / choice / contrast / uses；Domain 加 research；新增 Card、Clip、Agent / Slot、Fusion、Quest、Survey、TierThreshold（SENSE.md 6.10–6.14）

## M0 纸面原型（2026-10-20 第二版已出，见 m0/）

- [x] 读作者 Chrome 记录副本，抽搜索词、落地页、点击链
- [x] 第一版：从 Chrome 记录手写 12 张卡（作废，理由是推的）
- [x] 第二版：subagent 读会话日志，8 候选判断 ＋ 5 待补判断 ＋ 3 推荐，引文逐条核对；模板改成两种卡
- [ ] 收集作者反应；候选判断过半确认、待补卡有答案才进 M1
- [ ] 第一版 caveat 里没入卡的几条（不维护两份代码、每种语言专门声线、定价从体验出发）等反应后决定要不要补
- [ ] 若 Claude Code / Codex 会话里有评价类证据，补第二版判断卡（对照来自会话）

## M0.5 七种卡（下一步）

- [x] subagent 读会话日志出第一叠七种卡：41 张、16 条属性（m0/pocket.json）
- [x] 浏览器履历：品味整理启动时自动读（0023）；新版 app 要在「完全磁盘访问权限」里打开才读得到（0026）
- [ ] 邮件：需要先连 Gmail 连接器（当前未授权），连上后补人物卡
- [x] 口袋原型：口袋 ＋ 检索、3 个 agent 卡槽与出一手（在线问 Claude 或预先算好）、融合、委托、抽卡 survey、属性清单（https://claude.ai/artifact/TDhBwdXNKFTCK4iRD5MGko）
- [ ] 读回作者在口袋里的操作（2026-10-23 读过一次：只有一天的数据，3 手抽卡、6 次打开、「要问你的」没答过；A1–A3 各有一条反例，见下一条）（桌面版 ~/Library/Application Support/Sense/pocket-state.json；网页版 db 的 pocket/state），据此调门槛、改属性
- [x] 品级重定义（决策 0022）：银 = 关注它本身，金 = 口味、往外推荐生铜卡，金只能由银融合；pocket.json 迁移为 石 10 / 铜 20 / 银 11 / 金 0
- [x] 桌面版实跑：追踪台对「来自深渊」联网查到劇場版第 1 部 10/23 上映，生成更新卡（claude -p ＋ WebSearch 链路通）
- [x] 去 mock：融合按口袋实时状态出提示；委托真实倒数、每周由 agent 换一批；石 7 天 / 铜 60 天自动归档；「看了」更新进度、「再看」打开链接；外链交给系统浏览器
- [ ] 第一张金卡：「谜底型世界」「竜騎士07」两组预设融合的投入已全是银卡，等作者在口袋里融合
- [x] 判断银卡的"关注"实跑过：C01–C03 从会话日志各找到新证据，A1 / A2 / A3 还各找到一条反例
- [x] 桌面版（app/，Tauri v2）：⌃⌘A 截图、菜单栏、剪贴板流水、入口页（浏览器 / 文件 / 会话日志）、追踪台、用例卡、展示卡；agent 一律调本机 claude -p（决策 0021）
- [x] 桌面版实跑：⌃⌘A、菜单栏、追踪台、品味整理都跑过；Chrome 目录要完全磁盘访问（已确认，0026）
- [ ] 剪贴板"去向"只记到"之后切到的 app"；真要记粘贴得上辅助功能权限监听 ⌘V
- [ ] 追踪台只在 app 开着时转；要关机后也跑，得加 launchd 定时任务
- [x] 网页版 demo 改放 ludics.space/demo（0027）
- [ ] 完成标志：作者在口袋里留下过半的卡；至少一次融合与一次出一手跑通

- [ ] 反例攒起来以后怎么办：属性被反例打脸时，提议加场景条件或拆成两条，作为问题问作者（待定义）
- [ ] 第 2 轮实装（2026-10-22 ～ 23 已做）：Sense 页按领域（0023）、新卡与碎卡（0024）、快捷菜单与桌宠（0025）、听歌页面（0026）、开源与下载页（0027）、截图同时进剪贴板
- [ ] Spotify：作者在 Chrome 里登录 open.spotify.com 后再看一次
- [ ] 截图进剪贴板：作者实际按一次 ⌃⌘A、⌘V 验证

## M1 口袋 ＋ context（A、B 环）

- [ ] 口袋 app 骨架（macOS）、自带截图即建卡、剪贴板（7.1.1 的跳过规则先于一切）

- [ ] 适配器：Claude Code、Codex、浏览器履历与书签、指令文件、邮件、GitHub、论文、仓库
- [ ] 按 field 筛选与 `--exclude-work`
- [ ] 脱敏：读取边界一次、入库前一次，fail-closed
- [ ] 证据提取、属性合成、卡生成
- [ ] 完成标志：截图即建卡跑通；`scan --dry-run` 证据零泄露

## M2 KB ＋ 排序（C、D 环）

- [ ] SQLite ＋ FTS5 ＋ 向量表
- [ ] 来源：Semantic Scholar、GitHub
- [ ] 扩展循环：种子 → 查询 → 相关性门 → 去重 → 入库 → 下一跳；预算与深度
- [ ] 两阶段排序；四个内部量；why 指向 judgment_id
- [ ] `sense ask` 给出 3 条推荐；人的聚合与 entry_point
- [ ] Field / Item / SenseContext / Ranking 的 schema 定稿进 schema/

## M3 页面 ＋ 纠正（E、F 环）

- [ ] UMAP 预计算，单文件 HTML 四区块
- [ ] `sense correct` 传播规则表；HistoryEntry.summary
- [ ] `--public` 导出
- [ ] 完成标志：SENSE.md 第 8 节全程跑通，成功标准 1–5

## M4 MCP

- [ ] `sense mcp` 八个工具；`claude mcp add` / `codex mcp add` 接入说明
- [ ] arXiv、HN、web search 来源

## M5 游戏 field

- [ ] Steam / itch.io / HN 来源适配器
- [ ] "今晚想玩的游戏"走通，引擎不改
