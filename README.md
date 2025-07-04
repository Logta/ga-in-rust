# GA Prisoner's Dilemma in Rust

遺伝的アルゴリズム（GA）を用いて囚人のジレンマ戦略を進化させるRust実装です。

## 概要

このプロジェクトは、繰り返し囚人のジレンマゲームにおいて、遺伝的アルゴリズムを使用してエージェントの戦略を進化させます。各エージェントは過去の対戦履歴に基づいて協力（C）または裏切り（D）を選択し、世代を重ねるごとに最適な戦略を学習していきます。

## 主な特徴

- 🧬 遺伝的アルゴリズムによる戦略進化
- 🎮 囚人のジレンマゲームの実装
- 🤝 **新機能**: 直接互恵・間接互恵戦略の実装
- 🎯 **新機能**: 複数の戦略アルゴリズムから選択可能
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
# 開発モードで実行（デフォルト戦略: ルーレット選択）
mise run dev

# リリースビルドで実行（高速）
mise run build && ./target/release/ga-sim
```

### 戦略を指定した実行

```bash
# Tit-for-Tat戦略で実行
./target/debug/ga-sim -s tft

# 評判ベース戦略で実行
./target/debug/ga-sim --strategy reputation

# パラメータも同時に指定
./target/debug/ga-sim -s pavlov -g 1000 -p 30 -m 0.02

# ヘルプを表示（利用可能な戦略一覧を確認）
./target/debug/ga-sim --help
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
│   ├── main.rs                    # エントリーポイント
│   ├── lib.rs                     # ライブラリルート
│   ├── core/                      # コア機能
│   │   ├── errors.rs              # エラーハンドリング
│   │   ├── traits.rs              # 共通トレイト
│   │   └── types.rs               # 型定義
│   ├── domain/                    # ドメインロジック
│   │   └── simulation.rs          # シミュレーション管理
│   ├── engine/                    # 遺伝的アルゴリズムエンジン
│   │   ├── genetic/               # GA実装
│   │   ├── selection/             # 選択戦略
│   │   └── crossover/             # 交叉戦略
│   ├── infrastructure/            # インフラ層
│   │   ├── config.rs              # 設定管理
│   │   └── logging.rs             # ログ機能
│   ├── interface/                 # インターフェース層
│   │   ├── cli.rs                 # CLI実装
│   │   └── api.rs                 # API実装
│   ├── models/                    # データモデル
│   │   ├── game.rs                # ゲームロジック
│   │   └── model.rs               # エージェントモデル
│   └── strategies/                # ゲーム戦略
│       ├── utils.rs               # 基本戦略（ルーレット選択など）
│       ├── direct_reciprocity.rs  # 直接互恵戦略
│       ├── indirect_reciprocity.rs# 間接互恵戦略
│       ├── dynamic_strategy.rs    # 動的戦略切り替え
│       └── strategy_selector.rs   # 戦略選択器
├── tests/                         # テストファイル
├── Cargo.toml                     # Rust依存関係
└── .mise.toml                     # mise設定（タスク定義）
```

## 利用可能な戦略

### 基本戦略
- **roulette** - ルーレット選択（デフォルト）
- **threshold** - 閾値選択

### 直接互恵戦略
- **tft** - Tit-for-Tat（しっぺ返し戦略）
- **gtft** - Generous Tit-for-Tat（寛容なしっぺ返し）
- **pavlov** - Pavlov戦略（Win-Stay, Lose-Shift）

### 間接互恵戦略
- **reputation** - 評判ベース戦略
- **image-scoring** - イメージスコアリング戦略
- **standing** - スタンディング戦略

## パラメータ設定

コマンドライン引数で以下のパラメータをカスタマイズできます：

| パラメータ | 短縮形 | デフォルト値 | 説明 |
|-----------|--------|------------|------|
| `--generations` | `-g` | 50,000 | 実行する世代数 |
| `--population` | `-p` | 20 | エージェント数 |
| `--mutation-rate` | `-m` | 0.01 | 突然変異率 |
| `--dna-length` | `-d` | 6 | 戦略を表すDNAの長さ |
| `--report-interval` | `-r` | 5,000 | レポート出力間隔 |
| `--elite-size` | `-e` | 2 | エリート保存数 |
| `--strategy` | `-s` | roulette | 使用する戦略 |
| `--help` | `-h` | - | ヘルプを表示 |

## アルゴリズムの仕組み

### 戦略の表現

基本戦略（`roulette`, `threshold`）では、各エージェントはDNA文字列（例：`110101`）を持ち、確率的または閾値ベースで行動を決定します。

互恵戦略では、エージェント間の過去の相互作用履歴や評判スコアに基づいて協力/裏切りを決定します。

### 進化プロセス

1. **評価**: 全エージェントが総当たり戦を行い、得点を獲得
2. **選択**: 指定された戦略により高得点エージェントが優先的に選ばれる
3. **交叉**: 選ばれた親から新しい子を生成
4. **突然変異**: 一定確率でDNAをランダムに変更
5. **世代交代**: 新しい世代で1から繰り返し

### 戦略別の動作

#### 直接互恵戦略
- **TFT**: 初回は協力し、その後は相手の前回の行動を真似る
- **GTFT**: TFTに寛容性を追加、一定確率で裏切りを許す
- **Pavlov**: 前回の結果が良ければ同じ選択を、悪ければ変更

#### 間接互恵戦略
- **Reputation**: 相手の評判スコアに基づいて協力を決定
- **Image Scoring**: 自分の観察に基づいて他者の評判を管理
- **Standing**: 行動の文脈を考慮した評判管理

## 実行例とサンプル結果

### 基本的な実行例

```bash
# デフォルト戦略（ルーレット選択）で実行
./target/debug/ga-sim -g 100 -p 10
```

### TFT戦略の実行例

```bash
./target/debug/ga-sim -s tft -g 100 -p 10
```

**特徴**: TFT戦略では、すべてのエージェントが初回は協力するため、初期世代では全員が同じ点数を獲得します。

### 評判ベース戦略の実行例

```bash
./target/debug/ga-sim -s reputation -g 50 -p 5
```

**特徴**: 評判ベース戦略では、エージェント間の評判スコアに基づいて行動が決定されるため、より複雑な社会的相互作用が観察できます。

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
./target/release/ga-sim
```

### 戦略が期待通りに動作しない場合

```bash
# ヘルプで利用可能な戦略を確認
./target/debug/ga-sim --help

# 少ない世代数で動作確認
./target/debug/ga-sim -s tft -g 10 -p 5
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

## 詳細ドキュメント

プロジェクトの詳細については、以下のドキュメントを参照してください：

- [`docs/STRATEGY_GUIDE.md`](docs/STRATEGY_GUIDE.md) - 各戦略の詳細説明と理論的背景
- [`docs/USAGE_EXAMPLES.md`](docs/USAGE_EXAMPLES.md) - 実行例とサンプル結果
- [`CLAUDE.md`](CLAUDE.md) - 開発者向けガイドと実装詳細

## 参考資料

### 理論的背景
- [遺伝的アルゴリズムについて](https://ja.wikipedia.org/wiki/遺伝的アルゴリズム)
- [囚人のジレンマ](https://ja.wikipedia.org/wiki/囚人のジレンマ)
- Axelrod, R. (1984). *The Evolution of Cooperation*
- Nowak, M. A. (2006). *Five Rules for the Evolution of Cooperation*

### 技術的資料
- [Rust公式ドキュメント](https://doc.rust-lang.org/)
- [mise公式サイト](https://mise.jdx.dev/)
