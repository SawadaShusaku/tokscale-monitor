## Context

現在の tokscale-monitor は OpenCode のみ対応しており、SQLite DB（`~/.local/share/opencode/opencode.db`）を直接読み込んでローリングウィンドウ計算を行っている。tokscale-core には `UnifiedMessage` という共通構造体があり、すべてのクライアントがこのフォーマットに変換される設計になっている。これを活用すれば、他クライアント（Claude Code, Codex CLI 等）も同じ集計ロジックで監視可能。ただし各クライアントの生データ形式（SQLite / JSON / JSONL）とファイル構造が異なるため、個別のパーサーが必要。

## Goals / Non-Goals

**Goals:**
- クライアント選択タブ UI を実装し、OpenCode / Claude Code / Codex CLI を切り替えて監視できるようにする。
- 最左「Overview」タブで、全クライアントの全モデルの利用状況をプログレスバーで一覧表示する。
- バックエンドに `get_unified_messages(client)` コマンドを追加し、指定クライアントの生データを `UnifiedMessage` 形式で返す。
- Phase 1 として OpenCode（SQLite）、Claude Code（JSON ファイル群）、Codex CLI（JSON ファイル群）を対応。
- Overview のプログレスバーは使用率に応じて色を変化（80% 超で警告色）。

**Non-Goals:**
- 全 21 クライアントの同時対応（Phase 1 は 3 クライアントのみ）。
- クライアントの認証や API 連携（Cursor IDE の API キャッシュ等は Phase 2）。
- 履歴データの永続化やクラウド同期。
- 各クライアントの設定 UI（PlanConfig はコードでハードコード）。

## Decisions

### 1. タブ形式でクライアント切り替え
- **Decision**: 各クライアントのウィンドウをタブで切り替え、円グラフ x3 を表示。
- **Rationale**: クライアント数が増えると画面が圧迫されるため、タブで整理。Overview タブは別ビュー。
- **Alternatives**: 全クライアントをグリッドで一括表示 → 情報量が多すぎて見づらい。

### 2. Overview はプログレスバー（円グラフではない）
- **Decision**: Overview タブでは各モデルの使用率を水平プログレスバーで一覧表示。
- **Rationale**: モデル数が多くても縦スクロールで収まる。円グラフは 1 モデルあたり大きな領域を必要とする。
- **Alternatives**: ミニ円グラフ → 小さすぎて読み取りにくい。

### 3. バックエンド: `get_unified_messages(client)` で統一
- **Decision**: フロントエンドからクライアント名（`"opencode"`, `"claude"`, `"codex"` 等）を引数に渡し、対応するパーサーで生データを読み込む。
- **Rationale**: コマンドを 1 つに統一し、フロントエンドはクライアント名を指定するだけで済む。新規クライアント追加時もコマンド増加なし。
- **Alternatives**: クライアントごとに別コマンド → フロントエンドの `invoke` 呼び出しが増えて煩雑。

### 4. Phase 1: OpenCode / Claude Code / Codex CLI のみ
- **Decision**: 最初のリリースでは 3 クライアントのみ対応。
- **Rationale**: OpenCode は SQLite で既に実装済み。Claude Code と Codex CLI は JSON ファイル群で形式が比較的シンプル。他クライアントは調査・実装工数が不明確。

### 5. tokscale-core のパーサーを参考に自前実装
- **Decision**: tokscale-core のソースコードを参考に、Rust バックエンドに各クライアントのパーサーを移植する。
- **Rationale**: tokscale-core は crates.io 未公開の workspace 内 crate なので、直接依存できない。ソースコードは調査済み（`/tmp/tokscale` に clone）。
- **Alternatives**: tokscale-core を git submodule 化 → ライセンス・ビルド複雑性のため見送り。

## Risks / Trade-offs

- **[Risk] Claude Code / Codex CLI の JSON 構造が想定と異なる**  
  → **Mitigation**: 実装前に実際の JSON ファイルをサンプリングして構造を確認。フィールド名の違い（`model` vs `modelID` 等）はパーサー内で正規化する。

- **[Risk] ファイル数が多いクライアント（Claude Code: `~/.claude/projects/`）の読み込みが遅い**  
  → **Mitigation**: ディレクトリ走査は `std::fs::read_dir` で行い、非対話的に全ファイルを読む。遅延が顕著な場合はページネーションを検討。

- **[Risk] Overview の全モデル表示が縦に長くなりすぎる**  
  → **Mitigation**: プログレスバーはコンパクトに（高さ 8px、モデル名 + 使用率テキストのみ）。スクロール可能なコンテナに配置。

- **[Trade-off] Phase 1 で非対応のクライアントは「未対応」タブとして灰色表示**  
  → **Acceptance**: UI 上に存在しないクライアントより、灰色表示の方が今後の拡張を予示できて良い。

## Migration Plan

- 既存の OpenCode 単体表示はタブ UI に統合。「OpenCode」タブとして継続利用可能。
- `get_opencode_messages` は `get_unified_messages("opencode")` に統合され廃止予定。移行期間中は両方を維持。

## Open Questions

- Claude Code / Codex CLI の JSON ファイルの実際のフィールド構成は実装時にサンプル確認が必要。
- Overview のプログレスバーに表示するモデルは「全モデル」か「使用率上位 N 件」か？
