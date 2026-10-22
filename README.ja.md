# Sense

**あなたの好みを理解するエージェント。** · [中文](README.md) · [English](README.en.md)

**ダウンロード：[ludics.space](https://ludics.space)** · デモ：[ludics.space/demo](https://ludics.space/demo/) · MIT License

Sense は、あなたが見たもの、聴いたもの、スクショしたもの、AI と話したことから好みを学びます。それをカードにしてポケットに入れ、残す・捨てる・カードを引いて答える、を通して、あなたが本当に大事にしているものを覚え、記事・ゲーム・作品・作り手を探しに行きます。Sense があなたをどう理解しているかは、いつでも見て、直して、持ち出せます。

- 点数もパーセントもなし。好みは根拠までたどれる具体的な判断でできています。
- 領域をまたぐ推測（聴いている曲から観ているアニメを推す、など）は質問としてだけ出し、あなたが認めて初めて使います。
- 生のログは Mac の外に出ません。匿名化は読み込みの時点で行います。

## カード

| 段階 | 意味 |
|---|---|
| 石 | 入ったばかりで未処理。7 日放っておくと砕けて、「見つけた」という記録だけが残ります。 |
| 銅 | あなたが残したカード。 |
| 銀 | フォロー中。その作品自体の新しい話数・新作・新しい論文を調べます。 |
| 金 | 好みそのもの。同じ確かな「なぜ」を持つ銀カード 2 枚が融合して金になり、外へ向けておすすめし、銅カードを生みます。 |

## ダウンロードと起動

- **インストーラ**：[GitHub Releases](https://github.com/nikoloside/sense/releases/latest) の `.dmg`。Apple シリコンの macOS のみ。Apple の公証を受けていないので、初回起動がブロックされたら システム設定 › プライバシーとセキュリティ ›「このまま開く」。
- **必要なもの**：インストール済みでログインした [Claude Code](https://claude.com/claude-code)。エージェントの呼び出しはすべてローカルの `claude -p` で、あなた自身のアカウントです。YouTube Music / Spotify のページを読むには Claude in Chrome 拡張が必要です。
- **権限**：⌃⌘A のスクショには「画面収録」、Chrome 履歴を読むには「フルディスクアクセス」（任意）。
- **ソースからビルド**：[app/README.md](app/README.md) を参照（Rust、`cargo install tauri-cli`、Node）。`APPLE_SIGNING_IDENTITY` で署名を渡すと、ビルドし直しても macOS の権限が消えません。

## データの出どころ

- ⌃⌘A のスクショ（クリップボードにもコピー）、オンにしたときだけのクリップボード記録、デスクトップペットへの画像ドロップ。
- あなた自身の Chrome 履歴と YouTube 視聴履歴。一時コピーをローカルで読み、すぐ削除します。他人の Chrome プロフィールは `~/Library/Application Support/Sense/config.json` の `exclude_profiles` に書き、開きません。
- ログイン中の YouTube Music / Spotify のページ。Claude in Chrome を読み取り専用のツールと固定の許可リストで使い、曲名とアーティストだけを残します。
- Claude Code との会話であなたが述べた判断。

## このリポジトリのデータについて

`m0/pocket.json` とデモは作者本人の実際のポケットで、作者の同意のもとそのまま公開しています。第三者の名前は削除済みです。インストール後、あなたのポケットはあなた自身のデータから `~/Library/Application Support/Sense/` に育ち、アップロードされません。

## ドキュメント

設計ドキュメントと決定ログは中国語です：[docs/SENSE.md](docs/SENSE.md)（唯一の定義文書）、[docs/decisions/](docs/decisions/)、[docs/framework.html](docs/framework.html)。
