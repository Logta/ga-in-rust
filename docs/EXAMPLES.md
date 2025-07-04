# 使用例とチュートリアル

GA Prisoner's Dilemmaシミュレーションの使用例とチュートリアルです。

## クイックスタート

### 1. 最初のシミュレーション実行

```bash
# プロジェクトディレクトリに移動
cd ga-in-rust

# 小規模なテスト実行
ga-sim run --generations 50 --population 10 --dry-run

# 実際のシミュレーション実行
ga-sim run --generations 100 --population 20
```

期待される出力例:
```
INFO  シミュレーションを開始します
INFO  第0世代: 平均適応度=2.15, 最大適応度=3.20, 多様性=0.85%, 時間=12ms
INFO  第50世代: 平均適応度=3.45, 最大適応度=4.10, 多様性=0.62%, 時間=9ms
INFO  第100世代: 平均適応度=3.78, 最大適応度=4.25, 多様性=0.34%, 時間=8ms
INFO  シミュレーション完了: 2.34秒, 最高適応度: 4.25
```

### 2. 設定ファイルを使用した実行

```bash
# デフォルト設定ファイルを生成
ga-sim config init --format toml --path simulation.toml

# 設定を編集（お好みのエディタで）
nano simulation.toml

# 設定ファイルを使用して実行
ga-sim run --config simulation.toml
```

## 実用的なシミュレーション例

### 例1: 基本的な進化実験

目的: 基本的な囚人のジレンマ環境での戦略進化を観察

```bash
# 中規模シミュレーション
ga-sim run \
  --generations 1000 \
  --population 50 \
  --mutation-rate 0.01 \
  --rounds 20 \
  --report-interval 100 \
  --save-to basic_evolution.json
```

分析ポイント:
- 初期世代での多様性の高さ
- 中期での適応度の急激な向上
- 後期での収束と安定化

### 例2: 異なる突然変異率の比較

目的: 突然変異率が進化に与える影響を比較

```bash
# 低突然変異率
ga-sim run -g 500 -p 30 -m 0.005 --save-to low_mutation.json

# 標準突然変異率
ga-sim run -g 500 -p 30 -m 0.01 --save-to standard_mutation.json

# 高突然変異率
ga-sim run -g 500 -p 30 -m 0.05 --save-to high_mutation.json
```

期待される結果:
- 低突然変異率: 安定だが収束が遅い
- 標準突然変異率: バランスの取れた進化
- 高突然変異率: 多様性は高いが安定性に欠ける

### 例3: 大規模並列シミュレーション

目的: 高性能コンピューティング環境での大規模実験

```bash
# 大規模並列実行
ga-sim run \
  --generations 5000 \
  --population 200 \
  --parallel \
  --seed 42 \
  --save-to large_scale.json \
  --report-interval 250
```

最適化のポイント:
- 並列実行により実行時間を大幅短縮
- 大きな個体数により詳細な進化動態を観察
- 固定シードにより再現可能な結果

### 例4: 異なる環境条件の比較

**目的**: ペイオフ行列の変更が戦略進化に与える影響

```bash
# 協力的環境（協力報酬を高く設定）
cat > cooperative.toml << EOF
[genetic]
population_size = 40
generations = 1000

[simulation.payoff_matrix]
reward = 4      # 協力報酬を高く
temptation = 5
sucker = 1      # 裏切られても少し報酬
punishment = 0  # 両者裏切りは厳しく
EOF

ga-sim run --config cooperative.toml --save-to cooperative_env.json

# 競争的環境（裏切り誘惑を高く設定）
cat > competitive.toml << EOF
[genetic]
population_size = 40
generations = 1000

[simulation.payoff_matrix]
reward = 2      # 協力報酬を低く
temptation = 6  # 裏切り誘惑を高く
sucker = 0
punishment = 1
EOF

ga-sim run --config competitive.toml --save-to competitive_env.json
```

## 🎯 戦略別チュートリアル

### Tit-for-Tat戦略の実験

```bash
# TFT戦略での実行
ga-sim run \
  --strategy tit-for-tat \
  --generations 200 \
  --population 25 \
  --rounds 15 \
  --save-to tft_experiment.json
```

**TFT戦略の特徴:**
- 初回は必ず協力
- 以降は相手の前回行動を模倣
- 協力的だが報復もする「寛容な強さ」

### ランダム戦略での基準実験

```bash
# ランダム戦略（ベースライン）
ga-sim run \
  --strategy random \
  --generations 500 \
  --population 40 \
  --save-to random_baseline.json
```

**ランダム戦略の用途:**
- 他の戦略との比較基準
- 純粋に運による進化の観察
- アルゴリズムの動作確認

## 📈 結果の分析方法

### 1. 基本統計の確認

```bash
# 結果ファイルの概要確認
ga-sim analyze basic_evolution.json --summary

# 世代別進化の詳細表示
ga-sim analyze basic_evolution.json --generations --detailed
```

### 2. 複数実験の比較

```bash
# ベンチマーク実行で複数戦略を比較
ga-sim benchmark \
  --strategies tit-for-tat,random,always-cooperate \
  --iterations 5 \
  --generations 500 \
  --csv
```

### 3. カスタム分析スクリプト

```python
# Python分析スクリプト例
import json
import matplotlib.pyplot as plt

# 結果ファイルを読み込み
with open('basic_evolution.json') as f:
    data = json.load(f)

# 世代別適応度のプロット
generations = [g['generation'] for g in data['generation_history']]
avg_fitness = [g['avg_fitness'] for g in data['generation_history']]
max_fitness = [g['max_fitness'] for g in data['generation_history']]

plt.figure(figsize=(10, 6))
plt.plot(generations, avg_fitness, label='平均適応度', linewidth=2)
plt.plot(generations, max_fitness, label='最大適応度', linewidth=2)
plt.xlabel('世代')
plt.ylabel('適応度')
plt.title('進化の過程')
plt.legend()
plt.grid(True)
plt.savefig('evolution_progress.png')
```

## 🔧 高度な設定例

### パフォーマンス最適化設定

```toml
# performance.toml
[genetic]
population_size = 100
generations = 2000
mutation_rate = 0.008
elite_count = 5

[simulation]
rounds_per_match = 25

[performance]
parallel_enabled = true
thread_count = 8
memory_limit_mb = 2048

[output]
report_interval = 200
save_format = "json"
```

### 研究用詳細設定

```toml
# research.toml
[genetic]
population_size = 80
generations = 3000
mutation_rate = 0.012
elite_count = 3
dna_length = 12

[simulation]
rounds_per_match = 50
tournament_size = 3

[simulation.payoff_matrix]
reward = 3
temptation = 5
sucker = 0
punishment = 1

[analysis]
convergence_threshold = 0.01
convergence_window = 50
detailed_stats = true

[output]
report_interval = 150
save_format = "json"
include_individual_data = true
```

## 🧪 実験計画のテンプレート

### 実験1: 突然変異率の影響調査

```bash
#!/bin/bash
# mutation_study.sh

echo "突然変異率の影響調査を開始"

for rate in 0.001 0.005 0.01 0.02 0.05 0.1; do
    echo "突然変異率: $rate"
    ga-sim run \
        --generations 1000 \
        --population 50 \
        --mutation-rate $rate \
        --seed 123 \
        --save-to "mutation_${rate}.json"
done

echo "実験完了"
```

### 実験2: 個体数スケーリング研究

```bash
#!/bin/bash
# population_scaling.sh

echo "個体数スケーリング研究を開始"

for pop in 10 25 50 100 200; do
    echo "個体数: $pop"
    ga-sim run \
        --generations $((2000 * 50 / pop)) \  # 総計算量を一定に
        --population $pop \
        --parallel \
        --seed 456 \
        --save-to "population_${pop}.json"
done

echo "研究完了"
```

### 実験3: 環境変化への適応

```bash
#!/bin/bash
# environment_adaptation.sh

echo "環境変化への適応実験"

# 段階1: 協力的環境
ga-sim run --config cooperative.toml --save-to phase1_cooperative.json

# 段階2: 中立的環境  
ga-sim run --config neutral.toml --save-to phase2_neutral.json

# 段階3: 競争的環境
ga-sim run --config competitive.toml --save-to phase3_competitive.json

echo "適応実験完了"
```

## 🐛 よくある問題と解決策

### 問題1: メモリ不足エラー

```bash
# 症状: "メモリ不足" エラー
# 解決策: 個体数を減らすか、並列度を調整

ga-sim run --population 30 --parallel false  # 並列無効化
# または
ga-sim run --population 50 --config memory_optimized.toml
```

### 問題2: 収束が遅い

```bash
# 症状: 何千世代経っても適応度が向上しない
# 解決策: 突然変異率を上げるか、初期化を変更

ga-sim run --mutation-rate 0.02  # 突然変異率を上げる
# または
ga-sim run --seed $(date +%s)  # 異なるシードで試行
```

### 問題3: 結果の再現性がない

```bash
# 症状: 同じパラメータで異なる結果
# 解決策: 固定シードの使用

ga-sim run --seed 42  # 固定シード
# または
ga-sim run --config reproducible.toml  # 再現性設定
```

## 📚 次のステップ

1. **基本実験の実行**: このガイドの例1-2を試す
2. **設定のカスタマイズ**: 自分の研究目的に合わせて設定調整
3. **結果の分析**: 分析スクリプトで詳細な検証
4. **高度な実験**: 複数条件での比較実験
5. **論文化**: 結果を学術論文やレポートにまとめる

詳細な理論背景は[STRATEGIES.md](STRATEGIES.md)を、アーキテクチャの詳細は[ARCHITECTURE.md](ARCHITECTURE.md)を参照してください。