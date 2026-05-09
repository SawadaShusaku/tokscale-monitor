# Tokscale Monitor

> **⚠️ Early development / Work in progress**  
> This project is actively being built and is not yet feature-complete. Some parsers are stubs, and the UI/UX will evolve.

An **unofficial** desktop dashboard for [tokscale](https://github.com/junhoyeo/tokscale) — a popular CLI tool for tracking token usage across AI coding assistants. This app adds a native GUI, system tray, per-provider toggles, and additional parsers on top of the original tokscale concept.

![Screenshot](docs/assets/screenshot.png)

## What is Tokscale?

**[Tokscale](https://github.com/junhoyeo/tokscale)** (by [@junhoyeo](https://github.com/junhoyeo)) is a CLI tool that tracks token usage and costs across multiple AI coding assistants. This app (`tokscale-monitor`) is an **independent, unofficial desktop companion** that provides:

- **A native GUI** — No more terminal-only monitoring
- **System tray integration** — Runs in the background
- **Dynamic provider tabs** — Toggle which AI tools to monitor via settings
- **Direct parsers** — Native parsers for Claude Code, Codex, Qwen, etc. that read local data files directly (no CLI required)
- **Additional providers** — Qwen support added beyond the original tokscale scope

## Features

- **Multi-provider support** — Track usage across OpenCode, Claude Code, Codex, Cursor, Windsurf, and Qwen
- **Overview dashboard** — Provider totals or per-model breakdown with rolling windows (5h / Weekly / Monthly)
- **Settings panel** — Toggle which providers appear as tabs; settings persist via localStorage
- **System tray** — Runs in the tray; click to show/hide, right-click for menu

## Supported Providers

| Provider | Status | Notes |
|----------|--------|-------|
| OpenCode | ✅ Live | SQLite parser |
| Claude Code | ✅ Live | JSONL parser with token-based cost estimation |
| Codex | ✅ Live | JSONL parser with per-response cost tiers |
| Qwen | ✅ Live | JSONL parser with real token usage (added by this project) |
| Cursor | 🚧 Stub | Config ready; parser awaits data format confirmation |
| Windsurf | 🚧 Stub | Config ready; parser awaits data format confirmation |

## Tech Stack

- **Frontend:** Svelte 5, Tailwind CSS, SmartHR Design System
- **Backend:** Rust (Tauri), rusqlite
- **Build:** Vite
- **Platforms:** macOS, Windows, Linux

## Platform Notes

The app builds and runs on **macOS, Windows, and Linux**.  
File paths in parsers use `dirs::home_dir()`, so they resolve automatically on each platform (e.g. `~/.opencode/` on macOS → `C:\Users\<you>\.opencode\` on Windows).

> ⚠️ Some provider data locations may differ on Windows. Parsers are tested primarily on macOS.

## Installation

Download the latest release or build from source.

### Prerequisites

- **Node.js** (v20+)
- **Rust** (for Tauri)
- (Optional) **[tokscale CLI](https://github.com/junhoyeo/tokscale)** — The original CLI tool. Not required for this app to function.

### Build from source

```bash
# Install dependencies
npm install

# Build for production
npm run tauri build
```

Artifacts are generated in `src-tauri/target/release/bundle/`:

| Platform | Artifact | Path |
|----------|----------|------|
| macOS | `.app` | `macos/tokscale-monitor.app` |
| macOS | `.dmg` | `dmg/tokscale-monitor_0.1.0_aarch64.dmg` |
| Windows | `.exe` | `nsis/tokscale-monitor_0.1.0_x64-setup.exe` |
| Windows | `.msi` | `msi/tokscale-monitor_0.1.0_x64_en-US.msi` |

### Install on macOS

```bash
# Copy to Applications
cp -R "src-tauri/target/release/bundle/macos/tokscale-monitor.app" /Applications/

# Launch
open /Applications/tokscale-monitor.app
```

**Auto-start on login:**
```bash
osascript -e 'tell application "System Events" to make login item at end with properties {path:"/Applications/tokscale-monitor.app", hidden:false}'
```
Or manually: **System Settings → Users & Groups → Login Items → +**

### Install on Windows

1. Run the `.msi` or `.exe` installer from the `bundle/` directory
2. The app installs to `%LocalAppData%\tokscale-monitor\`
3. A Start Menu shortcut is created automatically

**Auto-start on login:**
- Press `Win + R`, type `shell:startup`, press Enter
- Copy a shortcut of `tokscale-monitor.exe` into that folder

## Development

```bash
# Install dependencies
npm install

# Run in dev mode (starts Vite + Tauri)
npm run tauri dev
```

## Relationship to Original Tokscale

This project is **not affiliated with** the original [junhoyeo/tokscale](https://github.com/junhoyeo/tokscale). It is an independent, community-driven GUI extension that:

- Adds a native desktop interface
- Implements direct file parsers for providers (no CLI dependency)
- Adds new providers (e.g., Qwen) not in the original scope
- Adds system tray and settings persistence

If you use the original tokscale CLI, this app can complement it. If you don't, the native parsers work standalone.

## License

MIT
