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

### claude — 温柔的思考者
```
A gentle, thoughtful girl. Very long flowing orange hair (#E8762C) reaching her knees, soft amber highlights,
side locks framing her face, a large dark-brown ribbon bow on the right side of her head.
Warm amber eyes, light blush. Dark chocolate-brown long-sleeved dress with orange trim and orange cuffs,
cream-white round collar with a small orange bow. Dark stockings, brown mary-jane shoes.
She hugs a thick brown leather book to her chest.
A small orange drawstring pouch charm with a round gold clasp (#E3A72F) hangs from her waist on a short cord.
```

### deepseek — 冷静的探索者
```
A calm, curious explorer girl. Long wavy royal-blue hair (#3F63D4) with periwinkle highlights, blue eyes.
White frilled maid headdress. Navy dress with a white frilled apron and periwinkle trim, white stockings, navy shoes.
A small blue whale tail peeks out from behind her skirt.
A small orange drawstring pouch charm with a round gold clasp (#E3A72F) hangs from her waist on a short cord.
```

### pouch — 口袋精（口袋本身）
```
The pouch itself personified: a cheerful small girl, orange-dominant (#E8762C) with cream and gold (#E3A72F) accents.
She wears a hood shaped like a drawstring pouch: the two top corners of the hood stick up like little ears,
a scalloped flap covers the top of her forehead, fastened by a round gold clasp.
Short peach-blonde hair peeks out under the hood, orange eyes, rosy cheeks.
Orange tunic dress over a cream blouse, drawstring cords with gold tassels hanging on her chest, brown boots.
She holds a small fan of four cards colored stone grey, bronze, silver and gold.
```
口袋精要比现在桌宠里那个像素口袋更像一个角色。兜帽的颜色和金扣沿用现在的口袋图标。

## 每个角色要出的帧（每个角色 13 张）

| 文件名 | 什么时候用 | 图像编辑提示词（在 ref 上改） |
|---|---|---|
| `ref.png` | 设定图，只做参考 | `character reference sheet, front view, same character` |
| `idle.png` | 待机 | `standing relaxed, soft smile, eyes open, mouth closed` |
| `idle_blink.png` | 眨眼 | `identical to idle, only the eyes closed in a gentle blink` |
| `idle_talk_a.png` | 说话（小口型） | `identical to idle, only the mouth slightly open as if speaking` |
| `idle_talk_b.png` | 说话（大口型） | `identical to idle, only the mouth open wider as if speaking` |
| `busy.png` | 引擎在跑 | claude：`reading her open book with a small magnifying glass, focused`；deepseek：`holding a small lantern forward, peering ahead curiously`；pouch：`sorting her four cards, focused` |
| `busy_blink.png` | 引擎在跑时眨眼 | `identical to busy, only the eyes closed in a blink` |
| `eat.png` | 进了一张卡 | `a small glowing card dropping into the pouch charm at her waist, she looks down delighted`（pouch：`a card dropping into her hood pouch`） |
| `cheer.png` | 开始关注、领任务、融合 | `both arms raised happily, eyes closed in a big smile` |
| `crack.png` | 碎了一张卡 | `startled, small flinch, eyes wide, hands up near her face` |
| `alert.png` | 有卡快碎或任务能领 | `one hand raised as if calling for attention, eyebrows up` |
| `sleep.png` | 引擎暂停 | `sitting on the ground hugging her knees, eyes closed, sleeping peacefully` |
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

## 出完之后（Claude 这边接）
- 桌宠窗口按新尺寸重排，状态到帧的对应关系沿用现在的 moods：idle、busy、sleep、alert、eat、crack、cheer。
- 谁在干活谁出场：引擎用 Claude 时站 Claude；语音说话时站 DeepSeek；都闲着时站口袋精。
- 说话时按 TTS 音量在 `idle_talk_a` 和 `idle_talk_b` 之间切换。
