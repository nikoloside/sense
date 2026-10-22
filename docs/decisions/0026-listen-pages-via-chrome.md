# 0026 · 听歌页面：让 Claude 用作者登录着的 Chrome 去看

日期：2026-10-23 · 状态：已定 · 补充 0023（音乐从哪来）

## 决策
- **来源**：音乐不再只靠 Chrome 履历（只留 90 天，YouTube Music 的履历还没歌名），也不靠脚本拿密码登录。改成让本机的 Claude Code 带 `--chrome`，在作者自己登录着的 Chrome 里打开白名单页面，读页面文字。
- **白名单**（写在 `LISTEN_PAGES`，prompt 里限定只能去这几页）：
  - YouTube Music 播放记录：music.youtube.com/history
  - YouTube Music 喜欢的歌：music.youtube.com/playlist?list=LM
  - Spotify 喜欢的歌：open.spotify.com/collection/tracks
  - 以后加网站，先改这张表，再改决策。
- **只读**：只给开标签、导航、读文字、查找、关标签这几个工具。不给点击、填表、执行脚本、上传；不登录。没登录就记「没登录」并跳过，由作者自己去 Chrome 登录。
- **只留什么**：歌名、歌手、在哪页看到、播放记录里同一首出现几次、哪几天看到过、页面分组（如「2026年9月」）。存在 `S.listen`，不存页面原文。
- **频率**：app 开着时每天一次（引擎暂停就不跑），Sense 页「听歌页面」框里也可以手动「现在去看」。看完接着跑一次品味整理。
- **品味整理的规则**（这次实测发现的问题，一并定下）：
  - 听歌数据只能进 music（偶像团体的歌进 idol）。一首歌出自哪部番 / 剧，只能写在 note 里。不能因为听了主题曲，就建那部番 / 剧的卡；想知道看没看，就问。
  - 只在「喜欢的歌」里、没有播放日期的，strength 记「点过喜欢」，是石卡，不算「常驻」。
- **Chrome 履历**：macOS 不让 app 读 Chrome 的目录（Operation not permitted）。改成用固定的开发者证书签名（身份 `app.sense.pocket`），作者在「完全磁盘访问权限」里打开一次就一直有效。没打开时，品味整理照常用听歌页面，不再整个失败。读失败的原因写进 `profiles_failed`，不再悄悄变成 0 条。

## 原因
- 作者 2026-10-23："claude 执行的话，他没有 web search，然后也不像别的 agent，可以调用浏览器"；"找 script 可以去反复登录我们 spotify 以及还有 music youtube 然后去看有没有 list"；同意用他登录着的 Chrome 去看。
- 实测：YouTube Music 历史 93 首、喜欢 30 首读到；Spotify 在 Chrome 里没登录。第一次整理把主题曲反推成了 20 张番剧和影视卡，已撤掉。

## 影响
- `app/src-tauri/src/lib.rs`：`run_claude` 的 `chrome` 参数、`CHROME_READ`、`profiles_failed`、`--scan-check`、`trace` → engine.log。
- `tauri.conf.json`：`signingIdentity`。
- `m0/pocket.template.html`：`listenScan`、`listenBox`、品味整理的 prompt、引擎表。
