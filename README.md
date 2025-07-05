# GA Prisoner's Dilemma in Rust

囚人のジレンマの戦略を遺伝的アルゴリズムで進化させるシミュレーション

## 概要

囚人のジレンマゲームにおける戦略の進化を研究するためのツールです。

## セットアップ

```bash
git clone https://github.com/yourusername/ga-in-rust.git
cd ga-in-rust
mise trust
mise install
```

## 使い方

```bash
# 開発モード
mise run dev

# シミュレーション実行
./target/debug/ga-sim -s tft -g 1000 -p 50

# ヘルプ
./target/debug/ga-sim --help
```

### パラメータ

- `-g, --generations`: 世代数
- `-p, --population`: 個体数  
- `-s, --strategy`: 戦略（tft, gtft, pavlov, reputation, image-scoring, standing）
- `-m, --mutation-rate`: 突然変異率
- `-r, --rounds`: ラウンド数

## プロジェクト構造

```
src/
├── main.rs       # エントリーポイント
├── ga/           # 遺伝的アルゴリズム
├── models/       # ゲームロジック
└── strategies/   # 戦略実装
    ├── direct_reciprocity.rs    # 直接互恵（TFT, GTFT, Pavlov）
    ├── indirect_reciprocity.rs  # 間接互恵（評判ベース）
    └── dynamic_strategy.rs      # 動的戦略選択
```

## 実装戦略

### 直接互恵
- **TFT**: しっぺ返し戦略
- **GTFT**: 寛容なTFT（10%で裏切りを許す）
- **Pavlov**: Win-Stay, Lose-Shift

### 間接互恵
- **評判ベース**: 評判スコアで協力判断
- **イメージスコアリング**: 観察による評判管理  
- **スタンディング**: 文脈考慮の評判判断


## 開発

```bash
# 全チェック
mise run all

# 個別実行
mise run fmt    # フォーマット
mise run lint   # 静的解析
mise run test   # テスト
```


## 参考文献

- Axelrod, R. (1984). *The Evolution of Cooperation*
- Nowak, M. A. (2006). *Five Rules for the Evolution of Cooperation*

## ライセンス

MIT License
