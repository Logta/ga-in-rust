# GA Prisoner's Dilemma in Rust

遺伝的アルゴリズムによる囚人のジレンマシミュレーション

[![Rust](https://img.shields.io/badge/rust-1.81%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![CI](https://github.com/yourusername/ga-in-rust/workflows/CI/badge.svg)](https://github.com/yourusername/ga-in-rust/actions)

## 概要

遺伝的アルゴリズム（GA）を用いて囚人のジレンマゲームの戦略を進化させるRustシミュレーションです。

## 主な特徴

- SFMT乱数生成器による高速実行
- 繰り返し囚人のジレンマゲーム
- カスタマイズ可能な遺伝的アルゴリズム
- 基本的な戦略システム
- 設定ファイル（TOML/YAML/JSON）対応
- CLI インターフェース（clap v4）
- 構造化ログ（tracing）

## セットアップ

### 前提条件

- Rust 1.81以上
- mise（推奨）または rustup

### インストール

```bash
# リポジトリのクローン
git clone https://github.com/yourusername/ga-in-rust.git
cd ga-in-rust

# mise環境の初期化とツールのインストール
mise trust
mise install
```

## 使い方

### 基本的な実行

```bash
# 開発モードでビルド・実行
mise run dev

# 本格的なシミュレーション実行
ga-sim run --generations 1000 --population 50 --mutation-rate 0.02

# ヘルプの表示
ga-sim --help
```

### CLIコマンド

| コマンド | 説明 | 例 |
|---------|------|---|
| `run` | シミュレーション実行 | `ga-sim run -g 500 -p 30` |
| `config` | 設定管理 | `ga-sim config show` |
| `strategies` | 利用可能戦略一覧 | `ga-sim strategies --detailed` |
| `benchmark` | ベンチマーク実行 | `ga-sim benchmark --iterations 5` |

### パラメータ設定

```bash
# 詳細パラメータでの実行
ga-sim run \
  --generations 2000 \        # 世代数
  --population 100 \          # 個体数  
  --mutation-rate 0.01 \      # 突然変異率
  --rounds 50 \               # 1試合あたりのラウンド数
  --strategy tit-for-tat \    # 使用戦略
  --parallel \                # 並列実行
  --seed 42 \                 # 再現可能性のためのシード
  --save-to results.json      # 結果の保存
```

### 設定ファイルの使用

```bash
# 設定ファイルの生成
ga-sim config init --format toml

# 設定を使った実行
ga-sim run --config simulation.toml

# 設定の検証
ga-sim config validate
```

## 📁 プロジェクト構造

```
ga-in-rust/
├── src/
│   ├── main.rs                 # エントリーポイント
│   ├── lib.rs                  # ライブラリルート
│   ├── core/                   # コア機能
│   │   ├── logging.rs          # tracing統合ロギング
│   │   ├── random.rs           # SFMT乱数生成器
│   │   ├── types.rs            # 型定義
│   │   └── validation.rs       # バリデーション
│   ├── config/                 # 設定管理
│   │   ├── loader.rs           # 設定ローダー
│   │   ├── schema.rs           # 設定スキーマ
│   │   └── validation.rs       # 設定検証
│   ├── cli/                    # CLI実装
│   │   ├── app.rs              # clap v4アプリ定義
│   │   ├── commands/           # コマンド実装
│   │   │   ├── run.rs          # runコマンド
│   │   │   ├── config.rs       # configコマンド
│   │   │   └── init.rs         # initコマンド
│   │   └── output.rs           # 出力フォーマット
│   ├── genetic/                # 遺伝的アルゴリズム
│   │   ├── algorithm.rs        # GAメインエンジン
│   │   ├── individual.rs       # 個体管理
│   │   ├── operations.rs       # 遺伝的操作
│   │   └── population.rs       # 個体群管理
│   ├── simulation/             # シミュレーション
│   │   ├── engine.rs           # シミュレーションエンジン
│   │   ├── environment.rs      # ゲーム環境
│   │   ├── runner.rs           # 実行管理
│   │   └── stats.rs            # 統計処理
│   └── strategies/             # 戦略実装
│       └── mod.rs              # 戦略モジュール
├── tests/                      # テスト
│   ├── integration/            # 統合テスト
│   ├── benchmarks/             # ベンチマーク
│   └── unit/                   # 単体テスト
├── configs/                    # サンプル設定
│   ├── default.toml            # デフォルト設定
│   ├── competitive.toml        # 競争的環境
│   └── cooperative.toml        # 協力的環境
├── docs/                       # ドキュメント
│   ├── ARCHITECTURE.md         # アーキテクチャ説明
│   ├── STRATEGIES.md           # 戦略ガイド
│   └── EXAMPLES.md             # 使用例
├── Cargo.toml                  # Rust依存関係
├── .mise.toml                  # mise設定
└── CLAUDE.md                   # 開発ガイド
```

## 戦略システム

### 基本戦略

- AlwaysCooperate: 常に協力
- AlwaysDefect: 常に裏切り  
- TitForTat: しっぺ返し戦略
- Random: ランダム選択

### 実装予定の戦略

- Generous TFT: 寛容なしっぺ返し
- Pavlov: Win-Stay, Lose-Shift戦略
- Reputation-based: 評判ベース戦略
- Image Scoring: イメージスコアリング

## 設定ファイル

### 設定例 (TOML)

```toml
[genetic]
population_size = 50
generations = 1000
mutation_rate = 0.01
elite_count = 2
dna_length = 8

[simulation]
rounds_per_match = 10
default_strategy = "tit-for-tat"

[simulation.payoff_matrix]
reward = 3      # 両者協力
temptation = 5  # 裏切り成功
sucker = 0      # 裏切られ
punishment = 1  # 両者裏切り

[output]
report_interval = 100
save_format = "json"

[performance]
parallel_enabled = true
thread_count = 4
memory_limit_mb = 1024
```

## 開発

### 開発環境セットアップ

```bash
# 全チェック実行
mise run all

# 個別コマンド
mise run fmt      # コードフォーマット
mise run check    # 型チェック  
mise run lint     # Clippyによる静的解析
mise run test     # テスト実行
mise run doc      # ドキュメント生成
mise run bench    # ベンチマーク実行
```

### ファイル監視実行

```bash
# ファイル変更時に自動実行
mise run watch
```

### テスト実行

```bash
# 全テスト実行
mise run test

# 特定カテゴリのテスト
cargo test unit::       # 単体テスト
cargo test integration:: # 統合テスト
cargo test bench::      # ベンチマーク
```

### セキュリティ監査

```bash
# セキュリティ監査実行
mise run audit
```

## 使用例とベンチマーク

### 基本的なシミュレーション

```bash
# 小規模テスト実行
ga-sim run --generations 100 --population 20 --dry-run

# 標準的なシミュレーション
ga-sim run --generations 1000 --population 50 --mutation-rate 0.01

# 高性能実行（並列処理）
ga-sim run --generations 5000 --population 100 --parallel --seed 42
```

### パフォーマンステスト

```bash
# ベンチマーク実行
ga-sim benchmark --strategies tit-for-tat,random --iterations 5 --generations 1000

# 大規模シミュレーション
ga-sim run --generations 10000 --population 200 --parallel --save-to large_sim.json
```

### 設定テンプレート使用

```bash
# 競争的環境でのシミュレーション
ga-sim run --config configs/competitive.toml

# 協力的環境でのシミュレーション  
ga-sim run --config configs/cooperative.toml
```

## トラブルシューティング

### ビルドエラー

```bash
# キャッシュクリア
mise run clean

# 依存関係更新
mise run update

# Rustツールチェーン確認
rustc --version  # 1.81+が必要
```

### パフォーマンス問題

```bash
# リリースビルドの使用
mise run build
./target/release/ga-sim run --generations 1000

# 並列実行の有効化
ga-sim run --parallel --generations 1000
```

### ログ設定

```bash
# ログレベル調整
export RUST_LOG=debug
ga-sim run --generations 100

# 詳細ログ
ga-sim run --log-level trace --generations 100
```

## コントリビューション

1. リポジトリをフォーク
2. 機能ブランチを作成: `git checkout -b feature/amazing-feature`
3. 変更をコミット: `git commit -m 'Add amazing feature'`
4. ブランチにプッシュ: `git push origin feature/amazing-feature`
5. プルリクエストを作成

### コントリビューション前のチェック

```bash
# 全チェック実行
mise run all

# セキュリティ監査
mise run audit

# ドキュメント更新確認
mise run doc
```

## 参考資料

### 理論的背景
- [遺伝的アルゴリズム](https://ja.wikipedia.org/wiki/遺伝的アルゴリズム)
- [囚人のジレンマ](https://ja.wikipedia.org/wiki/囚人のジレンマ)
- Axelrod, R. (1984). *The Evolution of Cooperation*
- Nowak, M. A. (2006). *Five Rules for the Evolution of Cooperation*

### 技術的資料
- [Rust公式ドキュメント](https://doc.rust-lang.org/)
- [mise公式サイト](https://mise.jdx.dev/)
- [clap v4ドキュメント](https://docs.rs/clap/latest/clap/)
- [tokio非同期ランタイム](https://tokio.rs/)
- [tracing構造化ログ](https://tracing.rs/)

## ライセンス

このプロジェクトはMITライセンスの下で公開されています。詳細は[LICENSE](LICENSE)ファイルを参照してください。
