## Why

現状の tokscale-monitor は3プロバイダー（OpenCode, Claude Code, Codex）のみ対応している。ユーザーが Cursor, Gemini, Windsurf 等他の AI コーディングアシスタントを併用している場合、これらの利用状況を監視できない。さらに、タブ数が増えると画面が圧迫されるため、どのプロバイダーをタブに表示するかをユーザーが選択できる設定 UI が必要。

## What Changes

- **設定画面（Gear アイコン）**: 右上に歯車アイコンを追加し、タブに表示するプロバイダーの ON/OFF を切り替えられるようにする
- **プロバイダー追加**: Cursor IDE, Gemini CLI/アプリ, Windsurf, GitHub Copilot Chat, Aider を新規対応
- **動的タブ生成**: 設定で ON にしたプロバイダーのみタブとして表示されるように変更
- **プロバイダー設定の永続化**: 設定は `localStorage` で永続化し、アプリ再起動後も維持

## Capabilities

### New Capabilities
- `settings-panel`: 歯車アイコンから開く設定パネル。プロバイダー表示/非表示のトグル UI
- `provider-cursor`: Cursor IDE の利用状況監視（`~/.cursor/` から JSONL/SQLite 読み込み）
- `provider-gemini`: Gemini CLI / アプリ の利用状況監視
- `provider-windsurf`: Windsurf の利用状況監視
- `provider-copilot`: GitHub Copilot Chat の利用状況監視
- `provider-aider`: Aider の利用状況監視

### Modified Capabilities
- `multi-client-support`: 動的プロバイダー一覧に対応。タブバーを設定に応じて生成
- `overview-dashboard`: 設定で無効化されたプロバイダーは Overview にも表示しない

## Impact

- `src/lib/components/`: `SettingsPanel.svelte`, `GearIcon.svelte` 新規作成
- `src/lib/clients/config.ts`: 新プロバイダー設定を追加
- `src/lib/stores/`: `settingsStore.ts` 新規（localStorage 永続化）
- `src/routes/+page.svelte`: 動的タブ生成ロジック追加
- `src-tauri/src/lib.rs`: 新パーサー追加
