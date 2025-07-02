# GA Prisoner's Dilemma - 開発ガイド

## 🔨 最重要ルール - 新しいルールの追加プロセス

ユーザーから今回限りではなく常に対応が必要だと思われる指示を受けた場合：

1. 「これを標準のルールにしますか？」と質問する
2. YES の回答を得た場合、CLAUDE.md に追加ルールとして[Append Rules]に記載する
3. 以降は標準ルールとして常に適用する

このプロセスにより、プロジェクトのルールを継続的に改善していきます。

## プロジェクト概要

遺伝的アルゴリズム（GA）を用いて囚人のジレンマ戦略を進化させる Rust プロジェクトです。

### 主要コンポーネント

- `src/main.rs`: エントリーポイント
- `src/ga/`: 遺伝的アルゴリズムの実装
- `src/models/`: ゲームロジックとエージェントモデル
- `src/strategies/`: 選択戦略（ルーレット選択など）

## 開発環境セットアップ

### 必要なツール

- [mise](https://mise.jdx.dev/)
- Rust (stable)

### 初期化

```bash
mise trust
mise install
```

## 開発コマンド

### 基本的な実行

```bash
mise run dev      # 開発モードで実行
mise run build    # リリースビルド
```

### 開発支援コマンド

```bash
mise run all      # 全チェック実行 (fmt, check, lint, test)
mise run fmt      # コードフォーマット
mise run check    # 型チェック
mise run lint     # Clippy による静的解析
mise run test     # テスト実行
mise run watch    # ファイル監視実行
mise run doc      # ドキュメント生成
```

### メンテナンス

```bash
mise run clean    # ビルドキャッシュクリア
mise run update   # 依存関係更新
mise run audit    # セキュリティ監査
```

## コーディング規約

### Rust コーディングスタイル

- `cargo fmt` で自動フォーマット
- `cargo clippy` の警告をすべて解決する
- コメントは日本語で記載する
- パニックを避け、`Result` 型を適切に使用する

### ネーミング規約

- 構造体: `PascalCase`
- 関数・変数: `snake_case`
- 定数: `SCREAMING_SNAKE_CASE`
- モジュール: `snake_case`

### エラーハンドリング

- 回復可能なエラーは `Result<T, E>` を使用
- 回復不可能なエラーは `panic!` を使用（最小限に）
- エラーメッセージは分かりやすく記述する

## アーキテクチャ

### モジュール構成

```
src/
├── main.rs           # エントリーポイント
├── lib.rs            # ライブラリルート
├── ga/               # 遺伝的アルゴリズム
│   ├── mod.rs
│   └── ga.rs
├── models/           # データモデル
│   ├── mod.rs
│   ├── game.rs       # ゲームロジック
│   └── model.rs      # エージェントモデル
└── strategies/       # 選択戦略
    ├── mod.rs
    └── utils.rs
```

### 設計原則

- 単一責任の原則を守る
- 疎結合・高凝集を意識する
- トレイトを活用してポリモーフィズムを実現
- テスタブルな設計を心がける

## テスト戦略

### テストの分類

- 単体テスト: 各モジュールの機能テスト
- 統合テスト: モジュール間の連携テスト
- パフォーマンステスト: 大量データでの性能確認

### テスト実行

```bash
mise run test         # 全テスト実行
mise run test-verbose # 詳細出力付きテスト
mise run watch-test   # テスト監視実行
```

## パフォーマンス考慮事項

### 最適化のポイント

- リリースビルドを使用（`--release`）
- アロケーションを最小限に抑える
- イテレータを活用する
- プロファイリングを定期的に実行

### ベンチマーク

```bash
mise run bench  # ベンチマーク実行
```

## Append Rules

### 全般

- コメントは日本語で記載すること
- `cargo clippy` の警告は必ず解決すること
- 新機能追加時はテストも同時に作成すること

### コード品質

- 関数は 50 行以内に収める
- 複雑な処理は適切に分割する
- マジックナンバーは定数として定義する

### Git 運用

- コミットメッセージは日本語で記載
- プルリクエスト前に `mise run all` を実行
- 1 コミット 1 機能を心がける
