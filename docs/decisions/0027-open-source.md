# 0027 · 开源：nikoloside/sense，下载页 ludics.space 首页

日期：2026-10-23 · 状态：已定

## 决策
- 仓库以 nikoloside 的名义公开在 GitHub：nikoloside/sense，MIT 许可证。
- **数据**：`m0/pocket.json` 和 demo 用作者本人的真实口袋，作者选择原样公开。第三方的信息不公开：
  - 别人的 Chrome profile 名单从代码里移到本机 `config.json` 的 `exclude_profiles`。
  - 卡片里提到别人 profile 名的那句话改成泛指。
  - 签名证书的名字不进 `tauri.conf.json`，改用环境变量 `APPLE_SIGNING_IDENTITY`。
- **下载页**：ludics.space，源码在 `site/`。
  - 页面上放三样：下载 .dmg（GitHub Releases）、GitHub 源码、在线 demo（ludics.space/demo，用的是 `app/ui/index.html` 的网页版）。
  - 不加统计脚本。
- **展示卡二维码**：Sense 页「存成图片」的展示卡右上角，加一个指向 ludics.space 的二维码。矩阵在构建前算好，嵌在模板里（`QR`）。白底黑格，任何配色都扫得出。
- **安装包**：GitHub Release 放 .dmg，只支持 Apple 芯片，没有经过 Apple 公证。下载页和 README 都写明这一点，以及需要 Claude Code。

## 原因
作者 2026-10-23："请把这个项目作为 nikoloside 直接开源，然后 sense 的 share 图片里面添加 qrcode，直接给到下载页，下载页里面直接放 github 和这个 demo"。选项里选了：真实口袋原样、ludics.space、MIT、Release 放 .dmg。

## 修订（同日）
作者："https://ludics.space/sense 不用这个，直接覆盖 https://ludics.space/"。下载页改为 ludics.space 首页（原来的「Worldlines Space」首页被替换，旧的 index.html 留了备份），demo 在 ludics.space/demo/，二维码改指 https://ludics.space。
- 再修订：作者要求「里面全部删除，ludics.space 只保留一个 sense」。S3 bucket `ludics.space` 清空后只放下载页（index.html、icon.png）和 demo/。原有 387 个文件（旧首页、app-*、covers/、旧 demo/ 等）删除前完整备份到作者本机 `~/Backups/ludics.space-2026-10-23/`，bucket 没开版本控制，要恢复就从这里传回去。
