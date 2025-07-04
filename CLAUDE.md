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
- `src/strategies/`: 選択戦略（ルーレット選択、直接互恵、間接互恵など）
- `src/strategies/direct_reciprocity.rs`: 直接互恵戦略（TFT、GTFT、Pavlov）
- `src/strategies/indirect_reciprocity.rs`: 間接互恵戦略（評判ベース、イメージスコアリング、スタンディング）
- `src/strategies/dynamic_strategy.rs`: 動的戦略切り替えシステム
- `src/strategies/strategy_selector.rs`: 戦略選択器

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
mise run dev      # 開発モードで実行（デフォルト戦略）
mise run build    # リリースビルド
```

### 戦略を指定した実行

```bash
# 特定の戦略で実行
./target/debug/ga-sim -s tft -g 1000 -p 30
./target/debug/ga-sim --strategy reputation -g 500 -p 20

# 利用可能な戦略を確認
./target/debug/ga-sim --help
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

## 実装された戦略

### 直接互恵戦略

#### Tit-for-Tat (TFT)
- 初回は協力し、その後は相手の前回の行動を真似る
- 単純で効果的な戦略として知られる
- 協力的で応報性がある

#### Generous Tit-for-Tat (GTFT)
- TFTに寛容性を追加
- 一定確率で相手の裏切りを許す
- ノイズがある環境で有効

#### Pavlov (Win-Stay, Lose-Shift)
- 前回の結果が良ければ同じ選択を継続
- 悪ければ行動を変更する
- 学習的側面を持つ戦略

### 間接互恵戦略

#### 評判ベース戦略
- エージェントの評判スコアに基づいて協力を決定
- 評判が良い相手には協力、悪い相手には非協力
- 社会的評判システムをモデル化

#### イメージスコアリング
- 自分の観察に基づいて他者の評判を管理
- 未知の相手に対する初期協力確率を設定
- より現実的な評判形成をモデル化

#### スタンディング戦略
- 行動の文脈を考慮した評判管理
- 悪い相手への裏切りを正当化
- より洗練された道徳的判断をモデル化

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

### 戦略実装

- 新しい戦略を追加する際は以下の手順を守る：
  1. `strategies/` ディレクトリに適切なファイルを作成
  2. `StrategyOperation` トレイトを実装
  3. `DynamicStrategy` 列挙型に新しい戦略を追加
  4. `StrategySelector` に戦略名とデフォルトパラメータを追加
  5. CLI ヘルプメッセージに戦略の説明を追加
  6. テストを作成して動作を確認

### ドキュメント

- 新機能追加時は必ず README.md を更新すること
- 戦略の追加時は理論的背景と動作原理を説明すること
- コマンドライン引数の変更時はヘルプメッセージも更新すること
