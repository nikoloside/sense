# Sense

**Your personal taste agent.** · [中文](README.md) · [日本語](README.ja.md)

**Download: [ludics.space](https://ludics.space)** · Demo: [ludics.space/demo](https://ludics.space/demo/) · MIT License

Sense learns your taste from what you watch, listen to, screenshot and talk about with AI. It turns those traces into cards in a pocket; as you keep, drop and answer quick draws, it learns what you actually care about, then goes looking for new articles, games, works and creators for you. You can inspect, edit and take its understanding of you with you at any time.

- No scores, no percentages: the profile is made of concrete judgments, each traceable to its evidence.
- Cross-domain guesses (e.g. from the music you play to the anime you watch) are only ever asked as questions; they count once you confirm.
- Raw logs never leave your Mac; redaction happens at the read boundary.

## Cards

| Tier | Meaning |
|---|---|
| Stone | Just arrived, not handled. Ignored for 7 days, it shatters and leaves only a "found once" record. |
| Bronze | You kept it. |
| Silver | Followed: Sense checks for new episodes, works or papers of the thing itself. |
| Gold | A taste: two silver cards sharing the same confirmed reason fuse into gold, which recommends outward and spawns bronze cards. |

## Download and run

- **Installer**: the `.dmg` in [GitHub Releases](https://github.com/nikoloside/sense/releases/latest). macOS on Apple silicon only. It is not notarized; if the first launch is blocked, use System Settings › Privacy & Security › "Open Anyway".
- **Requires** [Claude Code](https://claude.com/claude-code), installed and signed in. Every agent call is a local `claude -p` on your own account. Reading YouTube Music / Spotify pages needs the Claude in Chrome extension.
- **Permissions**: Screen Recording for ⌃⌘A screenshots; Full Disk Access to read Chrome history (optional).
- **Build from source**: see [app/README.md](app/README.md) (Rust, `cargo install tauri-cli`, Node). Pass a signing identity with `APPLE_SIGNING_IDENTITY` so macOS permissions survive rebuilds.

## Where the data comes from

- ⌃⌘A screenshots (also copied to the clipboard), opt-in clipboard capture, images dropped onto the desktop pet.
- Your own Chrome history and YouTube watch history, read locally from a temporary copy that is deleted right after. Other people's Chrome profiles are listed in `~/Library/Application Support/Sense/config.json` (`exclude_profiles`) and never opened.
- Your signed-in YouTube Music / Spotify pages, read through Claude in Chrome with read-only tools and a fixed allowlist; only titles and artists are kept.
- Judgments you stated in your Claude Code sessions.

## About the data in this repo

`m0/pocket.json` and the demo are the author's real pocket, published as-is with the author's consent; third-party names were removed. Once installed, your pocket grows from your own data under `~/Library/Application Support/Sense/` and is never uploaded.

## Docs

Design docs and the decision log are written in Chinese: [docs/SENSE.md](docs/SENSE.md) (the single source of truth), [docs/decisions/](docs/decisions/), [docs/framework.html](docs/framework.html).
