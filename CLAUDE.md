# GA Prisoner's Dilemma - 開発ガイド

## ルール追加プロセス

継続的な対応が必要な指示を受けた場合：
1. 「これを標準のルールにしますか？」と確認
2. YES の場合、[Append Rules]セクションに追加
3. 以降は標準ルールとして適用

## プロジェクト概要

囚人のジレンマの戦略を遺伝的アルゴリズムで進化させる Rust プロジェクト。

### 主要ファイル

- `src/main.rs`: エントリーポイント
- `src/ga/`: 遺伝的アルゴリズム
- `src/models/`: ゲームロジック
- `src/strategies/`: 戦略実装（直接互恵、間接互恵など）

## セットアップ

```bash
mise trust
mise install
```

## コマンド

```bash
# 実行
mise run dev      # 開発モード
mise run build    # リリースビルド

# 戦略指定
./target/debug/ga-sim -s tft -g 1000 -p 30
./target/debug/ga-sim --help  # ヘルプ

# 開発
mise run all      # 全チェック
mise run fmt      # フォーマット
mise run lint     # 静的解析
mise run test     # テスト
```

## コーディング規約

- `cargo fmt` で自動フォーマット
- `cargo clippy` の警告を解決
- コメントは日本語
- `Result` 型を使用（`panic!` は最小限）
- 構造体: `PascalCase`、関数: `snake_case`、定数: `SCREAMING_SNAKE_CASE`

## 戦略

### 直接互恵
- **TFT**: 初回協力、以降は相手の前回行動を真似
- **GTFT**: TFT + 一定確率で裏切りを許す
- **Pavlov**: 良い結果なら継続、悪ければ変更

### 間接互恵
- **評判ベース**: 評判スコアで協力判断
- **イメージスコアリング**: 観察に基づく評判管理
- **スタンディング**: 文脈を考慮した評判判断

## ディレクトリ構成

```
src/
├── main.rs      # エントリーポイント
├── ga/          # 遺伝的アルゴリズム
├── models/      # ゲームロジック
└── strategies/  # 戦略実装
```


## Append Rules

### 基本ルール
- コメントは日本語
- `cargo clippy` 警告を解決
- 新機能にはテスト追加
- 関数は50行以内
- マジックナンバーは定数化

### Git
- コミットメッセージは日本語
- PR前に `mise run all` 実行

### 戦略追加手順
1. `strategies/` にファイル作成
2. `StrategyOperation` トレイト実装
3. `DynamicStrategy` に追加
4. `StrategySelector` に登録
5. ヘルプメッセージ更新
6. テスト作成
