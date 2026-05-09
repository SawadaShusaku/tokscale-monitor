## Why

現在の tokscale-monitor は OpenCode のみ対応しており、Claude Code / Codex CLI / Gemini 等の他の AI コーディングアシスタントの利用状況を監視できない。ユーザーは複数のクライアントを併用しており、各クライアントごとにコスト上限を管理したい。また、全クライアントを横断した全体像（Overview）を一目で把握できる UI が欲しい。

## What Changes

- **クライアント選択タブ UI**: OpenCode、Claude Code、Codex CLI、Gemini CLI 等のクライアントをタブで切り替えられるようにする。最左に「Overview」タブを配置。
- **Overview タブ**: 全クライアント・全モデルの利用状況をプログレスバーで一覧表示。収まりを良くするため円グラフではなく水平バーで表示。
- **バックエンド拡張**: `get_unified_messages(client)` コマンドを追加。指定されたクライアントの生データ（SQLite または JSON）を読み込み、`UnifiedMessage` 形式で返す。
- **フロントエンド拡張**: `TabBar`、`ProgressBar`、`OverviewDashboard` コンポーネントを新規作成。SmartHR Design System に準拠したスタイリング。
- **PlanConfig 拡張**: クライアントごとに独立した PlanConfig（対象モデル、ウィンドウ上限）を定義できるようにする。

## Capabilities

### New Capabilities
- `multi-client-support`: 複数 AI クライアント（OpenCode, Claude Code, Codex CLI, Gemini CLI 等）の生データを個別に読み込み、タブ切り替えで監視できる機能。
- `overview-dashboard`: 全クライアントの全モデルの利用状況をプログレスバーで横断表示する Overview 機能。

### Modified Capabilities
- `plan-limit-monitor`: タブ切り替えに対応。選択中のクライアントの PlanConfig に基づいてウィンドウ計算を行う。

## Impact

- `src-tauri/src/lib.rs`: `get_unified_messages(client)` コマンド追加。各クライアントのパーサーモジュール追加。
- `src/lib/`: `clients/` ディレクトリ新設。クライアントごとの型定義・パーサー・設定を配置。
- `src/lib/components/`: `TabBar.svelte`、`ProgressBar.svelte`、`OverviewDashboard.svelte` 新規作成。
- `src/routes/+page.svelte`: タブ状態管理・クライアント切り替えロジック追加。
- 依存関係: フロントエンドに変更なし。バックエンドはクライアントごとに SQLite / JSON パーサーを追加（既存の `rusqlite` を再利用）。
