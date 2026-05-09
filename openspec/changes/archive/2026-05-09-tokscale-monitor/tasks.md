## 1. Backend (Rust)

- [x] 1.1 `src-tauri/src/lib.rs` に `get_tokscale_raw_data` コマンドを追加し、`std::process::Command` で `tokscale --json --no-spinner` を実行する
- [x] 1.2 コマンド実行エラーハンドリングを実装し、Tauri の `Result<String, String>` で成功時は stdout、失敗時はエラーメッセージを返す
- [x] 1.3 `tauri::generate_handler!` に `get_tokscale_raw_data` を登録し、`tauri::Builder` の `invoke_handler` を更新する

## 2. Frontend Types & Config

- [x] 2.1 `tokscale` JSON 出力の型定義（`TokscaleSession`: `date: string`, `cost: number`, `model: string` 等）を TypeScript で作成する
- [x] 2.2 `PlanConfig` 型と定数オブジェクト（`targetModels`, `windows` に 5h/weekly/monthly の上限）を作成する
- [x] 2.3 Tauri `invoke` ラッパー関数 `getTokscaleRawData(): Promise<string>` を実装する

## 3. Data Processing Logic

- [x] 3.1 ISO8601 日付文字列をパースし、ローカルタイムのエポックミリ秒に変換するユーティリティ関数 `parseTokscaleDate()` を実装する
- [x] 3.2 Target Models に基づくセッションフィルタリング関数 `filterTargetSessions()` を実装する
- [x] 3.3 3 つのローリングウィンドウ（5時間/7日/30日）の cost 集計関数 `calculateWindowCost()` を実装する（境界値は「現在時刻 - 期間 <= session.date < 現在時刻」）
- [x] 3.4 各ウィンドウの残予算・使用率・警告判定（80% 超）を算出する関数 `calculateWindowMetrics()` を実装する
- [x] 3.5 5時間ウィンドウの Next Free Slot（最も古いセッション時刻 + 5時間）を計算する関数 `calculateNextFreeSlot()` を実装する

## 4. UI Components

- [x] 4.1 SVG ドーナツチャートコンポーネント `DonutChart.svelte` を作成する（使用率に応じた円弧描画、中心に「$X.XX left」テキスト）
- [x] 4.2 使用率 80% 超時にチャート色を警告色（SmartHR Warning `#ffcc17`）に変更するロジックを実装する
- [x] 4.3 3 つのウィンドウを横並びグリッドで配置するメインダッシュボードコンポーネント `Dashboard.svelte` を作成する
- [x] 4.4 Next Free Slot の表示 UI を 5時間ウィンドウカード下部に追加する（例: "Next free slot: in 15m"）
- [x] 4.5 手動更新ボタンと最終更新時刻の表示を追加する
- [x] 4.6 SmartHR Design System（`#23221e` テキスト、`#f8f7f6` 背景、游ゴシックフォント、8px ベース余白）に従ったスタイリングを Tailwind CSS で適用する

## 5. Integration & Auto-Refresh

- [x] 5.1 `+page.svelte`（または該当ルート）で初回データ取得と集計結果の表示を実装する
- [x] 5.2 `setInterval` を用いた 60 秒ごとの自動ポーリングを実装する
- [x] 5.3 コンポーネントのアンマウント時（`onDestroy`）にタイマーをクリアするクリーンアップを実装する
- [x] 5.4 エラー発生時の UI 表示（インラインエラーメッセージまたは簡易トースト）を実装する

## 6. Testing

- [x] 6.1 ISO8601 日付パースのユニットテスト（UTC 入力から JST ローカルタイムへの正確な変換）を作成する
- [x] 6.2 ローリングウィンドウ境界値テスト（ちょうど 5時間前のセッションが含まれ、5時間+1ms 前のセッションが除外されること）を作成する
- [x] 6.3 Target Models フィルタリングテスト（対象モデルのみ含まれ、対象外が除外されること）を作成する
- [x] 6.4 Next Free Slot 計算テスト（空ウィンドウと非空ウィンドウの両ケース）を作成する
- [x] 6.5 使用率 80% 超時の警告色判定テストを作成する
