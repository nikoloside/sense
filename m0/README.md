# M0 · 纸面原型

## M0.5 · 口袋（2026-10-21，当前）

| 文件 | 是什么 |
|---|---|
| `pocket.json` | 41 张七种卡（判断 10、金句 7、兴趣 6、人物 5、关注 5、收藏 5、推荐 3），16 条属性，3 个 agent 及各自预先算好的一手（结果用 WebSearch 核实过），4 组融合，3 张委托。只用会话日志：浏览器履历被权限拦截，没有读 |
| `pocket.template.html` | 口袋 app 原型：口袋 ＋ 检索、牌桌（agent 卡槽与出一手）、委托、抽卡 survey、属性清单。数据注入在 `/*__POCKET__*/` |
| `pocket.html` | 构建产物，已发布为私密 artifact：https://claude.ai/artifact/TDhBwdXNKFTCK4iRD5MGko 。声明了 `sample`（出一手时真的问 Claude，不联网，结果标"未核实"）和 `db`（操作写到 `pocket/state`，Claude 能读回） |

构建：同下方 deck 的方法，模板换成 `pocket.template.html`，占位符换成 `/*__POCKET__*/`，数据换成 `pocket.json`。

## M0 第二版（2026-10-20）

| 文件 | 是什么 |
|---|---|
| `cards.json` | 16 张卡的数据：8 张候选判断卡（四要素齐）、5 张待补判断卡（缺理由或对照，只提问）、3 张推荐卡。每条证据只保留日期、来源（Claude Code / Codex / 指令文件 ＋ 项目目录名）和用户原话摘录（≤300 字，「」标出）；不含文件路径、URL、合作者真名。`meta.summary` 是"你和默认做法分岔在哪"的三句话，显示在汇总卡上。 |
| `deck.template.html` | 手机优先的卡片页模板。两种卡：判断卡（`status: candidate` 时动作是确认 / 不对 / 改一句；`pending` 时是答一句 / 不是我在意的）、推荐卡（like / skip / almost）。最后一张汇总卡列出分岔摘要和全部反应，并导出成 `sense` CLI 命令。反应只存在浏览器 localStorage（键 `sense-m0-reactions-v2`）。 |
| `deck.html` | 构建产物（模板 ＋ 数据），已发布为私密 artifact：https://claude.ai/artifact/U2HbRzTxh3SP57cnWistZ5 |

## 怎么来的

按决策 0014：由一个 subagent（Claude Code 作宿主）按 SENSE.md 7.1 的规则执行 A 环：

1. 逐行流式读 `~/.claude/projects/*/*.jsonl` 与 `~/.codex/sessions/**/*.jsonl`，只取 `type == user` 且 `role == user` 的文本块；跳过 tool_result、宿主注入（`<command-name>`、`<system-reminder>` 等）、isMeta / isSidechain、其他 agent 发来的 teammate 消息、定时任务模板。
2. 读取边界脱敏：密钥样式、邮箱、电话、≥32 位 hex/base64 替换；含 password / token= / secret= 的整条丢弃。
3. 按偏好标记（不要 / 别 / 不是…而是 / 应该 / 为什么 / 我觉得 / 不行 / 不对 …）分项目读样本，写卡。四要素必须来自用户自己写的话；缺一项即 `pending`。
4. 写出前用脚本把每一句「引文」核对回该日期的用户轮次（220 句，0 缺失）。
5. 抽取物（`logs/`）用完即删；脚本留在会话 scratchpad，不进仓库。

第一版（从 Chrome History 推的 12 张卡）已作废：标题里没有理由，判断是编的。浏览器记录按 0014 退为补充来源。

## 构建

```sh
python3 -c "
tpl=open('m0/deck.template.html',encoding='utf-8').read()
cards=open('m0/cards.json',encoding='utf-8').read()
open('m0/deck.html','w',encoding='utf-8').write(tpl.replace('/*__CARDS__*/',cards.replace('</script','<\\\\/script')))
"
```

## 成功标志

候选判断卡过半被确认；待补卡的问题有答案；推荐卡的 like / skip 能反过来指出哪张判断写错。满足后定第一个正式 field，进 M1。
