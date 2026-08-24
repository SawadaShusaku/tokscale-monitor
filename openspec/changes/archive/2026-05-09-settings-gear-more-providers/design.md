## Context

現在の tokscale-monitor はハードコードされた3タブ（Overview, OpenCode, Claude, Codex）構成。タブ数が増えると UI が圧迫されるため、ユーザーが表示するタブを選択できる必要がある。また、Cursor, Gemini, Windsurf 等の他の AI クライアントも利用しており、これらに対応するパーサーが必要。

## Goals / Non-Goals

**Goals:**
- 右上に歯車アイコンを配置し、クリックで設定パネルを表示
- 設定パネルで各プロバイダーのタブ表示/非表示をトグル
- 設定は localStorage に永続化
- 新プロバイダー: Cursor, Gemini, Windsurf, GitHub Copilot Chat, Aider
- タブバーを設定に応じて動的に生成
- Overview ダッシュボードも設定に応じて対象プロバイダーをフィルタ

**Non-Goals:**
- 全 21+ プロバイダーの同時対応（Phase 1 は追加5プロバイダーのみ）
- バックエンドでの認証や API 連携
- 各プロバイダーの PlanConfig をユーザーが編集（コードでハードコード）

## Decisions

### 1. 設定を localStorage で永続化
- **Decision**: Svelte store + localStorage で設定を保存
- **Rationale**: Tauri の設定 API を使うよりシンプル。フロントエンドのみで完結

### 2. 歯車アイコンはヘッダー右上
- **Decision**: タイトル「Tokscale Monitor」の右横に配置
- **Rationale**: 設定へのアクセスが容易。SmartHR のパターンに近い

### 3. 新プロバイダーは Phase 1 で5つ
- **Decision**: Cursor, Gemini, Windsurf, Copilot Chat, Aider
- **Rationale**: よく使われるツールを優先。データソースの調査工数が現実的

### 4. バックエンド: 各プロバイダー個別パーサー
- **Decision**: `get_unified_messages(client)` を拡張し、新しい client 名を追加
- **Rationale**: 既存のコマンド統合設計を維持

### 5. 設定パネルはドロワー/モーダル形式
- **Decision**: 画面右側からスライドインするドロワー
- **Rationale**: モーダルよりコンテキストを失わない

## Risks / Trade-offs

- **[Risk] 新プロバイダーの JSONL/SQLite 構造が不明**  
  → **Mitigation**: 実装前に各ツールの実際のデータファイルをサンプリング

- **[Risk] タブが0個になる設定**  
  → **Mitigation**: Overview は常に表示。プロバイダータブが0個の場合は「タブを有効化してください」メッセージ

- **[Trade-off] localStorage vs Tauri 設定 API**  
  → localStorage を選択。シンプルだが、複数ウィンドウ間の同期はされない

## Migration Plan

- 既存の3タブ構成はデフォルトで全て ON に設定
- 新規ユーザーにも同様のデフォルトを適用

## Open Questions

- Cursor のデータソースは `~/.cursor/` か？実際のパスと構造を確認が必要
- Gemini CLI は存在するか？または Gemini デスクトップアプリのみ？
- Windsurf のログファイルの場所は？
