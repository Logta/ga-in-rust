/// 遺伝的アルゴリズムのコアトレイト定義
///
/// このモジュールでは、遺伝的アルゴリズムの各構成要素が実装すべき
/// トレイトを定義しています。Rustのトレイトシステムを活用し、
/// 型安全性と拡張性を両立した設計を実現しています。
use crate::core::types::*;

/// 全ての遺伝的アルゴリズムエンティティの基底トレイト
///
/// 遺伝的アルゴリズムで扱う全てのオブジェクトが実装すべき基本的な機能を定義。
/// Clone, Send, Syncトレイトを要求することで、並行処理に対応しています。
///
/// # 必須メソッド
/// * `id()` - エンティティの一意識別子を返す
pub trait BaseEntity: Clone + Send + Sync {
    /// エンティティの一意識別子を取得
    ///
    /// # 戻り値
    /// エンティティのユニークなID
    fn id(&self) -> AgentId;
}

/// 遺伝的操作を行うトレイト
///
/// 遺伝的アルゴリズムの核となる遺伝的操作（交叉、突然変異、適応度評価）
/// を定義します。BaseEntityを継承し、基本的な機能も利用可能です。
///
/// # 必須メソッド
/// * `crossover()` - 他の個体との交叉操作
/// * `mutate()` - 突然変異操作
/// * `fitness()` - 適応度評価
pub trait GeneticOperations: BaseEntity {
    /// 他の個体との交叉を実行
    ///
    /// # 引数
    /// * `other` - 交叉相手の個体
    /// * `point` - 交叉点（遺伝子を分割する位置）
    ///
    /// # 戻り値
    /// 交叉により生成された新しい個体
    fn crossover(&self, other: &Self, point: CrossoverPoint) -> Self;

    /// 突然変異を実行
    ///
    /// # 引数
    /// * `rate` - 突然変異率（0.0-1.0）
    ///
    /// # 戻り値
    /// 突然変異が適用された個体（変異が発生しない場合は元の個体のクローン）
    fn mutate(&self, rate: MutationRate) -> Self;

    /// 個体の適応度を取得
    ///
    /// # 戻り値
    /// 環境への適応度を表す数値（通常は高いほど良い）
    fn fitness(&self) -> Fitness;
}

/// DNA操作に関するトレイト
///
/// 遺伝子情報（DNA）に対する基本的な操作を定義します。
/// 囚人のジレンマでは、DNAは戦略を表現する文字列として扱われます。
///
/// # 必須メソッド
/// * `dna()` - DNA文字列の参照を取得
/// * `dna_length()` - DNAの長さを取得
/// * `dna_sum()` - DNA内の文字の数値合計
/// * `dna_binary()` - DNAの文字列表現を取得
pub trait DnaOperations {
    /// DNA文字列の参照を取得
    ///
    /// # 戻り値
    /// DNAを表現する文字列への参照
    fn dna(&self) -> &Dna;

    /// DNAの長さを取得
    ///
    /// # 戻り値
    /// DNA文字列の文字数
    fn dna_length(&self) -> usize;

    /// DNA内の文字の数値合計を取得
    ///
    /// 統計情報や多様性の計算に使用されます。
    ///
    /// # 戻り値
    /// DNA内の全文字を数値として合計した値
    fn dna_sum(&self) -> u64;

    /// DNAの文字列表現を取得
    ///
    /// # 戻り値
    /// DNAの文字列表現
    fn dna_binary(&self) -> &str;
}

/// ゲーム内でのエージェント行動を定義するトレイト
///
/// 遺伝的操作とDNA操作の両方を継承し、ゲーム固有の機能を追加します。
/// 囚人のジレンマゲームにおけるエージェントの基本的な行動を定義。
///
/// # 必須メソッド
/// * `points()` - 現在の獲得ポイント
/// * `with_points()` - ポイントを設定した新しいインスタンス
/// * `is_active()` - アクティブ状態の確認
/// * `activate()` - エージェントをアクティブ化
/// * `deactivate()` - エージェントを非アクティブ化
pub trait Agent: GeneticOperations + DnaOperations {
    /// 現在の獲得ポイントを取得
    ///
    /// # 戻り値
    /// ゲームで獲得した累計ポイント
    fn points(&self) -> Points;

    /// 指定されたポイントを持つ新しいインスタンスを作成
    ///
    /// Rustの不変性原則に従い、既存のインスタンスを変更せず、
    /// 新しいインスタンスを作成して返します。
    ///
    /// # 引数
    /// * `points` - 設定するポイント数
    ///
    /// # 戻り値
    /// 指定されたポイントを持つ新しいエージェント
    fn with_points(&self, points: Points) -> Self;

    /// エージェントがアクティブかどうかを確認
    ///
    /// # 戻り値
    /// アクティブな場合true、そうでなければfalse
    fn is_active(&self) -> bool;

    /// エージェントをアクティブ状態にする
    fn activate(&mut self);

    /// エージェントを非アクティブ状態にする
    fn deactivate(&mut self);
}

/// Trait for selection strategies
pub trait SelectionStrategy<T: Agent> {
    fn select_parents(&self, population: &[T]) -> (T, T);
    fn select_survivors(&self, population: &[T], count: usize) -> Vec<T>;
}

/// Trait for game strategies
pub trait GameStrategy<T: Agent> {
    fn play_match(&self, agent1: &T, agent2: &T) -> (T, T);
}

/// Trait for genetic algorithm operations
pub trait GeneticAlgorithm<T: Agent> {
    fn population(&self) -> &[T];
    fn generation(&self) -> Generation;
    fn evolve(&mut self) -> anyhow::Result<()>;
    fn best_agent(&self) -> Option<&T>;
    fn average_fitness(&self) -> f64;
}

/// Trait for statistical operations
pub trait Statistics {
    fn mean(&self) -> f64;
    fn max(&self) -> Option<Points>;
    fn min(&self) -> Option<Points>;
    fn std_deviation(&self) -> f64;
}

impl Statistics for Vec<Points> {
    fn mean(&self) -> f64 {
        if self.is_empty() {
            0.0
        } else {
            self.iter().sum::<u64>() as f64 / self.len() as f64
        }
    }

    fn max(&self) -> Option<Points> {
        self.iter().max().copied()
    }

    fn min(&self) -> Option<Points> {
        self.iter().min().copied()
    }

    fn std_deviation(&self) -> f64 {
        if self.len() <= 1 {
            return 0.0;
        }

        let mean = self.mean();
        let variance = self
            .iter()
            .map(|&x| {
                let diff = x as f64 - mean;
                diff * diff
            })
            .sum::<f64>()
            / (self.len() - 1) as f64;

        variance.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statistics_mean() {
        let data = vec![1, 2, 3, 4, 5];
        assert_eq!(data.mean(), 3.0);

        let empty: Vec<Points> = vec![];
        assert_eq!(empty.mean(), 0.0);
    }

    #[test]
    fn test_statistics_max_min() {
        let data = [1, 5, 3, 2, 4];
        assert_eq!(data.iter().max(), Some(&5));
        assert_eq!(data.iter().min(), Some(&1));

        let empty: Vec<Points> = vec![];
        assert_eq!(empty.iter().max(), None);
        assert_eq!(empty.iter().min(), None);
    }

    #[test]
    fn test_statistics_std_deviation() {
        let data = vec![2, 4, 4, 4, 5, 5, 7, 9];
        let std_dev = data.std_deviation();
        assert!((std_dev - 2.138).abs() < 0.01);

        let single = vec![5];
        assert_eq!(single.std_deviation(), 0.0);
    }
}
