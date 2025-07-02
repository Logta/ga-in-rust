# GA Prisoner's Dilemma in Rust

遺伝的アルゴリズム（GA）を用いて囚人のジレンマ戦略を進化させるRust実装です。

## 概要

このプロジェクトは、繰り返し囚人のジレンマゲームにおいて、遺伝的アルゴリズムを使用してエージェントの戦略を進化させます。各エージェントは過去の対戦履歴に基づいて協力（C）または裏切り（D）を選択し、世代を重ねるごとに最適な戦略を学習していきます。

## 主な特徴

- 🧬 遺伝的アルゴリズムによる戦略進化
- 🎮 囚人のジレンマゲームの実装
- 📊 世代ごとの進化過程を可視化
- ⚡ 高速なRust実装
- 🛠 カスタマイズ可能なパラメータ

## セットアップ

### 前提条件

- [mise](https://mise.jdx.dev/)がインストールされていること
- Git

### インストール

```bash
# リポジトリのクローン
git clone https://github.com/yourusername/ga-in-rust.git
cd ga-in-rust

# mise環境の初期化
mise trust
mise install
```

## 使い方

### 基本的な実行

```bash
# 開発モードで実行
mise run dev

# リリースビルドで実行（高速）
mise run build && ./target/release/ga_prisoners_dilemma
```

### 開発コマンド

```bash
# コードチェック（全項目）
mise run all

# 個別のチェック
mise run fmt      # コードフォーマット
mise run check    # 型チェック
mise run lint     # Clippyによる静的解析
mise run test     # テスト実行

# 開発支援
mise run watch    # ファイル変更時に自動実行
mise run doc      # ドキュメント生成
```

### 利用可能なコマンド一覧

```bash
mise tasks
```

## プロジェクト構造

```
ga-in-rust/
├── src/
│   ├── main.rs           # エントリーポイント
│   ├── ga/               # 遺伝的アルゴリズム実装
│   │   ├── ga.rs         # GA中核ロジック
│   │   └── mod.rs
│   ├── models/           # データモデル
│   │   ├── game.rs       # ゲームロジック
│   │   ├── model.rs      # エージェントモデル
│   │   └── mod.rs
│   └── strategies/       # 選択戦略
│       ├── utils.rs      # ルーレット選択など
│       └── mod.rs
├── Cargo.toml            # Rust依存関係
└── .mise.toml            # mise設定（タスク定義）
```

## パラメータ設定

`src/main.rs`で以下のパラメータをカスタマイズできます：

| パラメータ | デフォルト値 | 説明 |
|-----------|------------|------|
| `GENERATIONS` | 50,000 | 実行する世代数 |
| `POPULATION` | 20 | エージェント数 |
| `MUTATION_RATE` | 0.01 | 突然変異率 |
| `DNA_LENGTH` | 6 | 戦略を表すDNAの長さ |
| `REPORT_INTERVAL` | 5,000 | レポート出力間隔 |

## アルゴリズムの仕組み

### 戦略の表現

各エージェントはDNA文字列（例：`CDCCDC`）を持ち、過去の対戦履歴に基づいて次の行動を決定します。

### 進化プロセス

1. **評価**: 全エージェントが総当たり戦を行い、得点を獲得
2. **選択**: ルーレット選択により高得点エージェントが優先的に選ばれる
3. **交叉**: 選ばれた親から新しい子を生成
4. **突然変異**: 一定確率でDNAをランダムに変更
5. **世代交代**: 新しい世代で1から繰り返し

## トラブルシューティング

### ビルドエラーが発生する場合

```bash
# キャッシュクリア
mise run clean

# 依存関係の更新
mise run update
```

### パフォーマンスを改善したい場合

```bash
# リリースビルドを使用
mise run build
./target/release/ga_prisoners_dilemma
```

## 開発に参加する

1. このリポジトリをフォーク
2. 機能ブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'Add amazing feature'`)
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. プルリクエストを作成

### 開発前のチェック

```bash
# 全チェックを実行
mise run all
```

## ライセンス

MITライセンス - 詳細は[LICENSE](LICENSE)ファイルを参照してください。

## 作者

- Logta (takenobu15@gmail.com)

## 参考資料

- [遺伝的アルゴリズムについて](https://ja.wikipedia.org/wiki/遺伝的アルゴリズム)
- [囚人のジレンマ](https://ja.wikipedia.org/wiki/囚人のジレンマ)
- [Rust公式ドキュメント](https://doc.rust-lang.org/)
