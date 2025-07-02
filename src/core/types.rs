/// 遺伝的アルゴリズムの基本型定義
///
/// 遺伝的アルゴリズムで使用される型エイリアスと定数を定義
/// エージェントの一意識別子
pub type AgentId = u64;

/// ゲーム内で獲得するポイント
pub type Points = u64;

/// エージェントの遺伝子情報（戦略を表現する文字列）
///
/// 例: "CDCDCD" where C=協力, D=裏切り
pub type Dna = String;

/// 進化世代番号
pub type Generation = usize;

/// 個体群サイズ
pub type Population = usize;

/// 突然変異率（0.0-1.0）
pub type MutationRate = f64;

/// 遺伝子交叉点
pub type CrossoverPoint = usize;

/// 個体の適応度
pub type Fitness = u64;

/// デフォルトの個体数
pub const DEFAULT_POPULATION: Population = 20;

/// デフォルトの世代数
pub const DEFAULT_GENERATIONS: Generation = 50_000;

/// デフォルトの突然変異率
pub const DEFAULT_MUTATION_RATE: MutationRate = 0.01;

/// デフォルトのDNA長
pub const DEFAULT_DNA_LENGTH: usize = 6;

/// デフォルトのレポート間隔
pub const DEFAULT_REPORT_INTERVAL: usize = 5_000;

/// デフォルトのエリート個体数
pub const DEFAULT_ELITE_SIZE: usize = 2;

/// 囚人のジレンマゲームの報酬マトリックス
/// 両者協力時の報酬
pub const COOPERATE_COOPERATE_REWARD: Points = 3;

/// 裏切り者の報酬（相手が協力）
pub const DEFECT_COOPERATE_REWARD: Points = 5;

/// 協力者の報酬（相手が裏切り）
pub const COOPERATE_DEFECT_REWARD: Points = 0;

/// 両者裏切り時の報酬
pub const DEFECT_DEFECT_REWARD: Points = 1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_sizes() {
        assert_eq!(std::mem::size_of::<AgentId>(), 8);
        assert_eq!(std::mem::size_of::<Points>(), 8);
        assert_eq!(
            std::mem::size_of::<Generation>(),
            std::mem::size_of::<usize>()
        );
    }

    #[test]
    fn test_constants() {
        assert!(DEFAULT_MUTATION_RATE > 0.0 && DEFAULT_MUTATION_RATE < 1.0);
        assert!(DEFAULT_POPULATION > 0);
        assert!(DEFAULT_GENERATIONS > 0);
        assert!(DEFAULT_ELITE_SIZE < DEFAULT_POPULATION);
    }
}
