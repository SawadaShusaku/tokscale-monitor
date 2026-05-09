## 1. Backend (Rust) - Multi-Client Support

- [x] 1.1 `UnifiedMessage` 構造体と `TokenBreakdown` を `src-tauri/src/lib.rs` に定義する
- [x] 1.2 `get_unified_messages(client: String)` コマンドを追加し、クライアント名に応じて分岐する
- [x] 1.3 OpenCode パーサー（SQLite）を `get_opencode_messages` から移植・統合する
- [x] 1.4 Claude Code パーサー（`~/.claude/projects/` JSON ファイル群）を実装する
- [x] 1.5 Codex CLI パーサー（`~/.codex/sessions/` JSON ファイル群）を実装する
- [x] 1.6 未対応クライアントの場合は空配列を返すハンドリングを実装する
- [x] 1.7 `tauri::generate_handler!` に `get_unified_messages` を登録する

## 2. Frontend Types & Config

- [x] 2.1 `UnifiedMessage` 型（`client`, `model_id`, `cost`, `timestamp`）を TypeScript で定義する
- [x] 2.2 `ClientConfig` 型と定数（クライアント名、ラベル、PlanConfig、データソース種別）を作成する
- [x] 2.3 `getUnifiedMessages(client: string): Promise<string>` ラッパー関数を実装する
- [x] 2.4 クライアントごとの `PlanConfig`（OpenCode / Claude / Codex）を定義する

## 3. Data Processing Logic

- [x] 3.1 `filterTargetMessages()` をクライアント別 PlanConfig に対応させる
- [x] 3.2 `calculateWindowCost()` を `UnifiedMessage`（timestamp）に対応させる
- [x] 3.3 `calculateWindowMetrics()` は既存のまま利用可能か確認・修正する
- [x] 3.4 `calculateNextFreeSlot()` を `UnifiedMessage` に対応させる
- [x] 3.5 Overview 用の全モデル集計関数 `aggregateAllModels()` を実装する

## 4. UI Components

- [x] 4.1 `TabBar.svelte` を作成する（Overview + クライアントタブ、SmartHR スタイル）
- [x] 4.2 `ProgressBar.svelte` を作成する（水平バー、使用率表示、80% 超警告色）
- [x] 4.3 `OverviewDashboard.svelte` を作成する（全モデルのプログレスバーリスト、スクロール対応）
- [x] 4.4 `ClientDashboard.svelte` を作成する（既存の円グラフ x3 をクライアント別に表示）
- [x] 4.5 タブ切り替え時にデータ取得とタイマーリセットを行うロジックを実装する

## 5. Integration & State Management

- [x] 5.1 `+page.svelte` にタブ状態（`activeTab: 'overview' | 'opencode' | 'claude' | 'codex'`）を追加する
- [x] 5.2 Overview タブ選択時に全クライアントのデータを並列取得する
- [x] 5.3 クライアントタブ選択時に該当クライアントのデータのみ取得する
- [x] 5.4 自動更新（60秒）をアクティブタブに応じて動作させる
- [x] 5.5 エラー表示をタブごとに分離（Overview 全体エラー / クライアント個別エラー）

## 6. Testing

- [x] 6.1 `get_unified_messages("opencode")` の統合テストを作成する
- [x] 6.2 `get_unified_messages("claude")` の統合テストを作成する（モック JSON ファイル使用）
- [x] 6.3 `get_unified_messages("codex")` の統合テストを作成する（モック JSON ファイル使用）
- [x] 6.4 `aggregateAllModels()` の集計ロジックテストを作成する
- [x] 6.5 ProgressBar コンポーネントの警告色判定テストを作成する
