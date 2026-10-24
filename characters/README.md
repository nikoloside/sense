# 角色

Sense 里会说话的三个角色，每个都有女版和男版（男版 id 后面加 `-m`，人设一样，只换性别）。每个角色一个文件夹：

- `soul.md`：她是谁、怎么说话、知道什么、不做什么。语音通话时，这份文件就是角色的系统提示，后面再接上当时的口袋摘要。中文是正本。
- `soul.en.md` / `soul.ja.md`：正本的英文、日文译本。通话语言是英语或日语时，整份系统提示（soul、成长层、口袋摘要、规矩）都用那门语言，不混中文指令。改正本后要重新翻译。
- `voice.md`：用哪个声音（voice-live 的 Fish 音色）。

| id | 名字 | 什么时候出场 |
|---|---|---|
| `claude` | Claude（大小姐） | 引擎用 Claude 时：桌上站她，通话也是她（0030） |
| `deepseek` | DeepSeek（大肥鱼） | 引擎用 DeepSeek 时 |
| `pouch` | 口袋精（小可爱） | 引擎闲着时站在桌上；口袋本身 |
| `claude-m` / `deepseek-m` / `pouch-m` | 对应的男版 | 快捷菜单里切到男版时，桌宠、通话的 soul 和声音、成长层都用男版 |

图在 `assets/characters/<id>/`，出图需求见 docs/art/characters.md。

三个角色都是原创的拟人形象，不代表 Anthropic 或 DeepSeek 公司本身。
