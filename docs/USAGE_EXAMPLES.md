# 使用例とサンプル結果

このドキュメントでは、GA Prisoner's Dilemmaの様々な実行例とその結果を示します。

## 基本的な使用例

### 1. デフォルト設定での実行

```bash
./target/debug/ga-sim
```

**結果の特徴:**
- ルーレット選択戦略を使用
- 50,000世代、20個体で実行
- DNAベースの確率的行動選択

### 2. パラメータ調整例

```bash
./target/debug/ga-sim -g 1000 -p 50 -m 0.05
```

**パラメータ説明:**
- `-g 1000`: 世代数を1000に短縮（高速実行）
- `-p 50`: 個体数を50に増加（多様性向上）
- `-m 0.05`: 突然変異率を5%に上昇（探索性向上）

## 戦略別実行例

### 3. Tit-for-Tat戦略

```bash
./target/debug/ga-sim -s tft -g 100 -p 10
```

**期待される結果:**
```
Generation 0
----------------------------------------
Agent  0: 100110 (points: 27)
Agent  1: 011010 (points: 27)
Agent  2: 101001 (points: 27)
Agent  3: 011100 (points: 27)
Agent  4: 001001 (points: 27)
Agent  5: 001111 (points: 27)
Agent  6: 001110 (points: 27)
Agent  7: 101010 (points: 27)
Agent  8: 100001 (points: 27)
Agent  9: 110100 (points: 27)
Average points: 27.00
```

**特徴:**
- 全エージェントが同じ点数（27点）を獲得
- TFTは初回協力するため、全員が協力し合う
- 安定した協力状態が維持される

### 4. 評判ベース戦略

```bash
./target/debug/ga-sim -s reputation -g 50 -p 5
```

**期待される結果:**
```
Generation 0
----------------------------------------
Agent  0: 001010 (points: 12)
Agent  1: 100001 (points: 12)
Agent  2: 111111 (points: 12)
Agent  3: 011110 (points: 12)
Agent  4: 011001 (points: 12)
Average points: 12.00
```

**特徴:**
- 評判スコア（初期値0.0）に基づく行動
- 初期は中立的評判のため、閾値0.0で協力
- 時間とともに評判システムが発達

### 5. Pavlov戦略

```bash
./target/debug/ga-sim -s pavlov -g 200 -p 15
```

**特徴:**
- Win-Stay, Lose-Shift原理
- 前回の結果に基づく適応的行動
- 学習効果により最適戦略に収束

## 戦略比較実験

### 6. 短期実験での戦略比較

#### ルーレット選択（確率的）
```bash
./target/debug/ga-sim -s roulette -g 50 -p 10
```

#### TFT（決定論的）
```bash
./target/debug/ga-sim -s tft -g 50 -p 10
```

#### 寛容TFT（確率的寛容性）
```bash
./target/debug/ga-sim -s gtft -g 50 -p 10
```

**比較ポイント:**
- 初期世代での点数分布
- 収束速度の違い
- 安定性の比較

## パフォーマンステスト

### 7. 大規模実験

```bash
./target/release/ga-sim -s reputation -g 10000 -p 100 -r 1000
```

**設定説明:**
- リリースビルドで高速実行
- 10,000世代の長期実験
- 100個体の大規模集団
- 1,000世代ごとにレポート

### 8. 高変異率実験

```bash
./target/debug/ga-sim -s pavlov -m 0.1 -g 500 -p 20
```

**目的:**
- 高い突然変異率（10%）の影響を観察
- 探索と利用のバランステスト
- 戦略の頑健性評価

## 実験結果の解釈

### 点数の意味

囚人のジレンマの利得行列：
```
           相手
        協力  裏切り
自分 協力  3,3   0,5
    裏切り 5,0   1,1
```

- **27点**: 9回戦で全て相互協力（3×9 = 27）
- **12点**: 相互協力と他の組み合わせが混在
- **0〜45点**: 様々な戦略の混合結果

### 収束パターン

1. **協力収束**: TFT系戦略で見られる
2. **裏切り収束**: 短期的利益を重視する場合
3. **混合均衡**: 複数戦略が共存する状態

## トラブルシューティング

### よくある問題と解決法

#### 1. 予期しない結果
```bash
# デバッグ用短期実行
./target/debug/ga-sim -s tft -g 10 -p 5 -r 5
```

#### 2. パフォーマンス問題
```bash
# リリースビルドの使用
mise run build
./target/release/ga-sim
```

#### 3. 戦略の動作確認
```bash
# ヘルプで利用可能戦略を確認
./target/debug/ga-sim --help
```

## 実験の設計指針

### 比較実験のベストプラクティス

1. **統制変数**: 一度に一つのパラメータのみ変更
2. **複数試行**: 乱数の影響を考慮して複数回実行
3. **適切な世代数**: 戦略の特性に応じた十分な世代数
4. **記録保持**: 結果を記録して比較分析

### 推奨実験パターン

```bash
# 基本性能比較
for strategy in roulette threshold tft gtft pavlov reputation; do
    echo "Testing $strategy"
    ./target/debug/ga-sim -s $strategy -g 100 -p 20
done
```

この実行例集を参考に、様々な条件下での囚人のジレンマ戦略の進化を観察してください。