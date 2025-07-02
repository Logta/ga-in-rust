/// 遺伝的アルゴリズムのコア型定義
/// 
/// このモジュールでは、遺伝的アルゴリズムで使用される基本的な型エイリアスと
/// 定数を定義しています。型安全性を保ちつつ、コードの可読性を向上させます。

/// エージェントの一意識別子
/// 
/// 各エージェント（個体）を識別するためのユニークなID
pub type AgentId = u64;

/// ゲーム内で獲得できるポイント数
/// 
/// 囚人のジレンマゲームでエージェントが獲得する得点を表現
pub type Points = u64;

/// エージェントの遺伝子情報（DNA）
/// 
/// エージェントの戦略を表現する遺伝子配列（例: "CDCDCD"）
/// - 'C': 協力（Cooperate）
/// - 'D': 裏切り（Defect）
pub type Dna = String;

/// 世代数
/// 
/// 遺伝的アルゴリズムの進化における世代の番号
pub type Generation = usize;

/// 個体数
/// 
/// 集団（Population）内の個体（Agent）の数
pub type Population = usize;

/// 突然変異率
/// 
/// 遺伝子が突然変異する確率（0.0-1.0の範囲）
pub type MutationRate = f64;

/// 交叉点
/// 
/// 遺伝子交叉を行う際の分割点のインデックス
pub type CrossoverPoint = usize;

/// 適応度
/// 
/// 個体の環境適応度を表す数値（通常は獲得ポイントと同じ）
pub type Fitness = u64;

/// ## 設定定数
/// 
/// 遺伝的アルゴリズムのデフォルト設定値

/// デフォルトの個体数
/// 
/// 小規模な実験に適した個体数設定
pub const DEFAULT_POPULATION: Population = 20;

/// デフォルトの世代数
/// 
/// 十分な進化を促すための世代数設定
pub const DEFAULT_GENERATIONS: Generation = 50_000;

/// デフォルトの突然変異率
/// 
/// 探索と収束のバランスを取った突然変異率
pub const DEFAULT_MUTATION_RATE: MutationRate = 0.01;

/// デフォルトのDNA長
/// 
/// 囚人のジレンマの基本的な戦略を表現するのに十分な長さ
pub const DEFAULT_DNA_LENGTH: usize = 6;

/// デフォルトのレポート間隔
/// 
/// 進捗状況を報告する世代間隔
pub const DEFAULT_REPORT_INTERVAL: usize = 5_000;

/// デフォルトのエリート個体数
/// 
/// 次世代に確実に引き継がれる優秀な個体の数
pub const DEFAULT_ELITE_SIZE: usize = 2;

/// ## ゲーム固有定数
/// 
/// 囚人のジレンマゲームの報酬マトリックス

/// 両者協力時の報酬
/// 
/// 双方が協力した場合に各プレイヤーが得る報酬
pub const COOPERATE_COOPERATE_REWARD: Points = 3;

/// 一方裏切り時の裏切り者報酬
/// 
/// 自分が裏切り、相手が協力した場合の報酬（最大報酬）
pub const DEFECT_COOPERATE_REWARD: Points = 5;

/// 一方裏切り時の協力者報酬
/// 
/// 自分が協力し、相手が裏切った場合の報酬（最小報酬）
pub const COOPERATE_DEFECT_REWARD: Points = 0;

/// 両者裏切り時の報酬
/// 
/// 双方が裏切った場合に各プレイヤーが得る報酬
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
