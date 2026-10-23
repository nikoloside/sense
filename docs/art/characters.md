# 角色美术需求（交给 Codex 出图）

三个角色：**Claude**、**DeepSeek**、**口袋精**（Sense 的口袋本身拟人化）。都是原创的二头身。

## 规则
- **不模仿任何现有角色、画师或网站的画风**，也不画任何公司的 logo、文字或水印。名字只用在文件名上，不出现在画面里。
- 每个角色先出一张**设定图（ref）**。之后每一帧都把 ref 当参考图，用图像编辑模式改姿势和表情，保证脸、发型、服装、配色每帧一致。
- **统一规格**：
  - 画布 1024×1024 PNG，透明背景，边缘没有白边或光晕。
  - 角色居中（x=512），脚底在 y=980，头顶大约在 y=120。同一个角色的所有帧比例、位置都一样。
  - 不画地面阴影，不画「!」「z」之类的符号，这些由 app 自己叠。
- **交付目录**：
  - `assets/characters/<id>/<frame>.png`，id 是 `claude` / `deepseek` / `pouch`。
  - 另外每张缩一份 256×256 放进 `assets/characters/<id>/256/`。
- 出完后把同一角色的全部帧叠在一起检查：头和脚的位置不跳。

## 通用风格提示词（每张都加在最前面）

```
Original chibi anime character, super-deformed 2-heads-tall proportions (head about half of total height),
big expressive eyes, clean soft lineart, cel shading with gentle gradients, warm and friendly,
full body, facing the viewer at a slight 3/4 angle, standing centered on an invisible baseline,
transparent background, no ground shadow, no text, no logo, no watermark, desktop mascot sprite,
consistent design across frames.
```

## 三个角色

### claude — 大小姐
```
An elegant, proud little ojou-sama (rich young lady). Very long orange hair (#E8762C) with soft amber highlights,
the ends falling in big glossy ringlet curls, a large dark-brown satin ribbon bow on the right side of her head.
Warm amber eyes with long lashes, a confident, slightly smug smile, chin a little raised.
Luxurious dark chocolate-brown ruffled dress with orange trim, lace cuffs and a cream-white lace collar
with a small gold brooch, layered petticoat, dark stockings, polished brown mary-jane shoes.
She holds a small folding lace fan near her mouth; a thick leather-bound ledger book with gold corners
is tucked under her other arm.
A small orange drawstring pouch charm with a round gold clasp (#E3A72F) hangs from her waist on a short cord.
```

### deepseek — 大肥鱼
```
A big, round, chubby whale girl — soft and plump like a dumpling, much rounder than the others.
She wears a royal-blue (#3F63D4) whale costume hoodie: the hood is a whale head with little eyes on top
and a white belly stripe down the front, sleeves end in small flippers, a big round whale tail behind her.
Wavy blue hair peeks out from the hood, sleepy half-lidded blue eyes, round blushing cheeks, a lazy content smile.
She hugs a small fish-shaped snack. Unbothered, slow, cozy.
A small orange drawstring pouch charm with a round gold clasp (#E3A72F) hangs from her waist on a short cord.
```

### pouch — 小可爱
```
The pouch itself personified: a tiny, extra-cute baby-like girl, noticeably smaller than the other two
(about 80% of their height), very big head, huge sparkly orange eyes, round rosy cheeks, tiny hands.
Orange-dominant (#E8762C) with cream and gold (#E3A72F) accents.
She wears a hood shaped like a drawstring pouch: the two top corners stick up like little ears,
a scalloped flap over the forehead fastened by a round gold clasp.
Short peach-blonde hair peeks out under the hood. Orange tunic dress over a cream blouse,
drawstring cords with gold tassels on the chest, small brown boots.
She holds a small fan of four cards colored stone grey, bronze, silver and gold.
```
口袋精要比现在桌宠里那个像素口袋更像一个角色：更小、更圆、更可爱。兜帽的颜色和金扣沿用现在的口袋图标。

## 每个角色要出的帧（每个角色 13 张）

| 文件名 | 什么时候用 | 图像编辑提示词（在 ref 上改） |
|---|---|---|
| `ref.png` | 设定图，只做参考 | `character reference sheet, front view, same character` |
| `idle.png` | 待机 | `standing relaxed, soft smile, eyes open, mouth closed` |
| `idle_blink.png` | 眨眼 | `identical to idle, only the eyes closed in a gentle blink` |
| `idle_talk_a.png` | 说话（小口型） | `identical to idle, only the mouth slightly open as if speaking` |
| `idle_talk_b.png` | 说话（大口型） | `identical to idle, only the mouth open wider as if speaking` |
| `busy.png` | 引擎在跑 | claude：`reading her open ledger book through a gold lorgnette, focused and dignified`；deepseek：`swimming forward lazily with her tail, holding a small lantern, sniffing around for food`；pouch：`sorting her four cards with tiny hands, concentrating hard` |
| `busy_blink.png` | 引擎在跑时眨眼 | `identical to busy, only the eyes closed in a blink` |
| `eat.png` | 进了一张卡 | `a small glowing card dropping into the pouch charm at her waist, she looks down delighted`（pouch：`a card dropping into her hood pouch`） |
| `cheer.png` | 开始关注、领任务、融合 | claude：`laughing proudly with the fan open, one hand on her hip`；deepseek：`bouncing happily, flippers up, belly jiggling`；pouch：`both tiny arms raised, eyes closed in a big smile` |
| `crack.png` | 碎了一张卡 | `startled, small flinch, eyes wide, hands up near her face` |
| `alert.png` | 有卡快碎或任务能领 | `one hand raised as if calling for attention, eyebrows up` |
| `sleep.png` | 引擎暂停 | claude：`dozing elegantly in a sitting pose, fan resting on her lap`；deepseek：`curled up like a round blue mochi, snoring softly`；pouch：`curled up asleep inside her own hood` |
| `wave.png` | 打招呼（启动时） | `waving one hand at the viewer, cheerful smile` |

`idle_blink`、`idle_talk_a`、`idle_talk_b`、`busy_blink` 和它们的原帧之间，**除了眼睛或嘴，一个像素都不能动**，因为 app 会快速切换这几帧做眨眼和口型。如果编辑模式改动了别处，就只取原帧的眼部或嘴部区域，把它合回原帧。

## 另外要的小件

| 文件名 | 用途 | 提示词 |
|---|---|---|
| `charm/idle.png` | 单独的口袋挂饰（512×512，透明） | `a small orange drawstring pouch charm with a round gold clasp on a short cord, cute, front view` |
| `charm/full.png` | 进卡后鼓起来 | `same pouch charm, bulging slightly with a card peeking out of the top` |
| `pouch/bust.png` | 菜单头部和设置页头像（512×512） | 口袋精半身像，正脸、微笑 |
| `pouch/icon.png` | app 图标候选（1024×1024） | 口袋精的头和兜帽，圆角方形里放橙色底，图标风格 |

合计：3 个角色 × 13 张 = 39 张，加上挂饰 2 张、头像 1 张、图标 1 张。

## 男版（每个角色一个，作者手动选）

每个角色再出一个男版，人设一样，只换性别。桌上站谁、用女版还是男版，都由作者在快捷菜单里自己选，app 不会自己换。

- **规则、规格、通用风格提示词、13 帧的清单，都和上面的女版一样。**
- 男版另出一张自己的 `ref.png`，后面的帧都用男版的 ref 当参考，**不要拿女版的 ref 去改性别**，否则脸会在两版之间漂。
- 帧表里写 she / her 的地方，男版改成 he / his。
- 身高和女版一样，也是二头身，脚底 y=980。口袋精男版也比另外两个小一圈。
- 少年感，不要成年男性，不要肌肉；和女版站在一起要像同一套角色。
- 交付目录：`assets/characters/claude-m/`、`deepseek-m/`、`pouch-m/`，同样每张缩一份 256 放进 `256/`。

### claude-m — 大少爷
```
An elegant, proud little young master (rich boy), a boyish counterpart of the orange-haired ojou-sama.
Orange hair (#E8762C) with soft amber highlights, neat and slightly wavy, the back gathered into a short low ponytail
tied with a dark-brown satin ribbon; one curl falls over his forehead.
Warm amber eyes, a confident, slightly smug smile, chin a little raised.
Dark chocolate-brown tailcoat with orange piping, a cream-white ruffled cravat pinned with a small gold brooch,
a cream waistcoat, dark-brown shorts with knee socks, polished brown shoes.
He holds a small gold lorgnette in one hand; a thick leather-bound ledger book with gold corners
is tucked under his other arm.
A small orange drawstring pouch charm with a round gold clasp (#E3A72F) hangs from his belt on a short cord.
```
帧的差异：
- `busy`：`reading his open ledger book through his gold lorgnette, focused and dignified`
- `cheer`：`laughing proudly with one hand on his hip, the other raising the lorgnette like a toast`
- `sleep`：`dozing elegantly in a sitting pose, the ledger open on his lap`

### deepseek-m — 大肥鱼（男）
```
A big, round, chubby whale boy — soft and plump like a dumpling, much rounder than the others.
He wears a royal-blue (#3F63D4) whale costume hoodie: the hood is a whale head with little eyes on top
and a white belly stripe down the front, sleeves end in small flippers, a big round whale tail behind him.
Short messy blue hair peeks out from the hood, sleepy half-lidded blue eyes, round blushing cheeks, a lazy content smile.
Blue shorts, chunky white sneakers. He hugs a small fish-shaped snack. Unbothered, slow, cozy.
A small orange drawstring pouch charm with a round gold clasp (#E3A72F) hangs from his waist on a short cord.
```
帧的差异：和女版一样（busy 拿小灯笼游着找吃的，cheer 鳍举起来、肚子一颤，sleep 团成一颗蓝色麻薯）。

### pouch-m — 小可爱（男）
```
The pouch itself personified as a tiny, extra-cute baby-like boy, noticeably smaller than the other two
(about 80% of their height), very big head, huge sparkly orange eyes, round rosy cheeks, tiny hands.
Orange-dominant (#E8762C) with cream and gold (#E3A72F) accents.
He wears a hood shaped like a drawstring pouch: the two top corners stick up like little ears,
a scalloped flap over the forehead fastened by a round gold clasp.
Short fluffy peach-blonde hair with a little cowlick peeks out under the hood.
Orange tunic over a cream shirt, orange shorts, drawstring cords with gold tassels on the chest, small brown boots.
He holds a small fan of four cards colored stone grey, bronze, silver and gold.
```
帧的差异：和女版一样（busy 理那四张卡，cheer 两只小手举起来，sleep 缩进自己的兜帽里睡）。

男版合计：3 × 13 = 39 张，不用另出挂饰和图标。

## 出完之后（Claude 这边接）
- 桌宠按状态换帧：idle、busy、sleep、alert、eat、crack、cheer、wave。
- 桌上站谁由作者在快捷菜单「桌上是谁」里手动选，app 不会跟着引擎或通话自己换人。
- 说话时按 TTS 音量在 `idle_talk_a` 和 `idle_talk_b` 之间切换。
