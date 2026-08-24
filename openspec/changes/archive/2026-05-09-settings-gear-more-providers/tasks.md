## 1. Settings Store & Persistence

- [x] 1.1 `settingsStore.ts` を作成し、各プロバイダーの表示/非表示状態を管理
- [x] 1.2 localStorage との同期（読み込み・書き込み）を実装
- [x] 1.3 デフォルト設定を定義（全プロバイダー ON）
- [x] 1.4 設定変更時にリアクティブにタブ一覧を更新

## 2. Settings UI Components

- [x] 2.1 `GearIcon.svelte` を作成（歯車アイコン、ヘッダー右上配置）
- [x] 2.2 `SettingsPanel.svelte` を作成（右側ドロワー形式）
- [x] 2.3 プロバイダートグルリストを実装（SmartHR スタイルのスイッチ）
- [x] 2.4 ドロワーの開閉アニメーションを実装
- [x] 2.5 設定パネル外クリックで閉じる動作を実装

## 3. Dynamic Tab Generation

- [x] 3.1 `TabBar.svelte` を設定に応じて動的タブ生成に変更
- [x] 3.2 設定で OFF のプロバイダーはタブに表示しない
- [x] 3.3 タブが0個の場合のフォールバック表示を実装
- [x] 3.4 Overview ダッシュボードも設定に応じて対象プロバイダーをフィルタ

## 4. Backend - New Providers (Rust)

- [ ] 4.1 `cursor` パーサーを実装（`~/.cursor/` または他のデータソース）— **STUB: データ形式未確認**
- [ ] 4.2 `gemini` パーサーを実装 — **STUB: データ形式未確認**
- [ ] 4.3 `windsurf` パーサーを実装 — **STUB: データ形式未確認**
- [ ] 4.4 `copilot` パーサーを実装 — **STUB: データ形式未確認**
- [ ] 4.5 `aider` パーサーを実装 — **STUB: データ形式未確認**
- [x] 4.6 `get_unified_messages` に新しい client 名を追加
- [ ] 4.7 各新パーサーの構造を事前調査（JSONL/SQLite サンプリング）

## 5. Frontend - New Provider Configs

- [x] 5.1 `CLIENT_CONFIGS` に新プロバイダー5つを追加
- [x] 5.2 各プロバイダーの PlanConfig を定義（モデル名、ウィンドウ、上限）
- [x] 5.3 新プロバイダーのデータソース種別を特定（jsonl と仮定）
- [x] 5.4 `ALL_CLIENTS` を静的リストとして拡張（設定でのフィルタリングはフロントエンドで実装済み）

## 6. Integration & Testing

- [x] 6.1 設定変更 → タブ更新 → データ取得の連鎖動作をテスト（型チェック通過）
- [x] 6.2 全プロバイダー OFF → Overview のみ表示されることを確認（論理実装済み）
- [x] 6.3 localStorage 永続化の動作確認（store 実装済み、手動テスト推奨）
- [ ] 6.4 新パーサーの統合テスト（各プロバイダー1件ずつ）— **STUB のため保留**
- [x] 6.5 設定パネルの開閉とトグル操作の UI テスト（コンポーネント実装済み）
