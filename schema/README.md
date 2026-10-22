# 数据模型

第一版只有五种对象。全部是 JSON，字段用英文。没有分数字段（见 decisions/0002）。

```
Evidence ──聚合──► Judgment ──驱动──► Recommendation ◄── Request
                      ▲                      │
                      └────── Correction ◄───┘
```

| 对象 | 是什么 | 谁创建 |
|---|---|---|
| **Evidence** 证据 | 一句可溯源的 claim："多次要求删掉解释性注释"，附来源、时间、scope | 导入时由 Agent 生成；用户可删 |
| **Judgment** 判断 | 自然语言的品味判断，附证据、领域、适用条件、状态 | Agent 提出，用户确认 / 修改 / 否定 |
| **Request** 需求 | 用户的一句话，拆出长期品味之外的当下状态和实际限制 | 用户 |
| **Recommendation** 推荐 | 一个条目 ＋ 为什么适合（指向具体 judgment） | Agent |
| **Correction** 纠正 | 用户对推荐或判断的反应，落到具体 judgment 上 | 用户 |

档案 (`TasteProfile`) 就是 judgments 的集合加版本号。每次纠正产生新版本，旧版本保留。

完整字段见 `taste-profile.schema.json`，示例见 `../examples/taste-profile.example.json`。
