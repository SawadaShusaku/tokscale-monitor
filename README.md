# Tokscale Monitor

> **⚠️ Early development / Work in progress**  
> This project is actively being built and is not yet feature-complete. Some parsers are stubs, and the UI/UX will evolve.

A desktop app (built with Tauri + SvelteKit) that monitors token usage and estimated costs across multiple AI coding assistants.

![Screenshot](docs/assets/screenshot.png)

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
| Qwen | ✅ Live | JSONL parser with real token usage |
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

## License

MIT
