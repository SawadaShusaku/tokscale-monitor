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

## Development

```bash
# Install dependencies
npm install

# Run in dev mode (starts Vite + Tauri)
npm run tauri dev

# Build for production
npm run tauri build
```

## License

MIT
