/// シミュレーション統計情報の管理
///
/// 世代ごとの統計データと最終結果の統計を管理
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// 世代ごとの統計情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationStats {
    /// 世代番号
    pub generation: usize,
    
    /// 平均適応度
    pub avg_fitness: f64,
    
    /// 最大適応度
    pub max_fitness: f64,
    
    /// 最小適応度
    pub min_fitness: f64,
    
    /// 適応度の標準偏差
    pub fitness_std_dev: f64,
    
    /// 遺伝的多様性（0.0-1.0）
    pub diversity: f64,
    
    /// 収束度（0.0-1.0、高いほど収束）
    pub convergence: f64,
    
    /// エリート個体数
    pub elite_count: usize,
    
    /// 実行時間（ミリ秒）
    pub elapsed_ms: u64,
    
    /// 戦略分布（戦略名: パーセンテージ）
    pub strategy_distribution: std::collections::HashMap<String, f64>,
}

impl GenerationStats {
    /// 新しい世代統計を作成
    pub fn new(
        generation: usize,
        fitness_values: &[f64],
        diversity: f64,
        elite_count: usize,
        elapsed_ms: u64,
        strategy_distribution: std::collections::HashMap<String, f64>,
    ) -> Result<Self> {
        if fitness_values.is_empty() {
            anyhow::bail!("適応度データが空です");
        }

        let avg_fitness = fitness_values.iter().sum::<f64>() / fitness_values.len() as f64;
        let max_fitness = fitness_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let min_fitness = fitness_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        
        // 標準偏差の計算
        let variance = fitness_values
            .iter()
            .map(|&x| {
                let diff = x - avg_fitness;
                diff * diff
            })
            .sum::<f64>() / fitness_values.len() as f64;
        let fitness_std_dev = variance.sqrt();
        
        // 収束度の計算（標準偏差の逆数を正規化）
        let convergence = if max_fitness > min_fitness {
            1.0 - (fitness_std_dev / (max_fitness - min_fitness)).min(1.0)
        } else {
            1.0 // 全て同じ適応度の場合は完全収束
        };

        Ok(Self {
            generation,
            avg_fitness,
            max_fitness,
            min_fitness,
            fitness_std_dev,
            diversity,
            convergence,
            elite_count,
            elapsed_ms,
            strategy_distribution,
        })
    }
}

/// シミュレーション全体の統計情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationStats {
    /// 実行した総世代数
    pub total_generations: usize,
    
    /// 最終世代の統計
    pub final_stats: GenerationStats,
    
    /// 各世代の統計履歴
    pub generation_history: Vec<GenerationStats>,
    
    /// 最良個体の情報
    pub best_individual: BestIndividualInfo,
    
    /// 収束情報
    pub convergence_info: ConvergenceInfo,
    
    /// パフォーマンス情報
    pub performance_info: PerformanceInfo,
}

/// 最良個体の情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestIndividualInfo {
    /// 最高適応度
    pub best_fitness: f64,
    
    /// 最良個体が出現した世代
    pub best_generation: usize,
    
    /// 最良個体のDNA
    pub best_dna: String,
    
    /// 最良個体の説明
    pub description: String,
}

/// 収束情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceInfo {
    /// 収束したかどうか
    pub converged: bool,
    
    /// 収束した世代（収束していない場合はNone）
    pub convergence_generation: Option<usize>,
    
    /// 収束の基準値
    pub convergence_threshold: f64,
    
    /// 収束判定に使用した連続世代数
    pub convergence_window: usize,
}

/// パフォーマンス情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceInfo {
    /// 総実行時間（ミリ秒）
    pub total_elapsed_ms: u64,
    
    /// 世代あたりの平均実行時間（ミリ秒）
    pub avg_generation_time_ms: f64,
    
    /// 最高速度の世代（ミリ秒）
    pub fastest_generation_ms: u64,
    
    /// 最低速度の世代（ミリ秒）
    pub slowest_generation_ms: u64,
    
    /// 並列処理が有効だったか
    pub parallel_enabled: bool,
    
    /// 使用したスレッド数
    pub thread_count: usize,
}

impl SimulationStats {
    /// 新しいシミュレーション統計を作成
    pub fn new(
        generation_history: Vec<GenerationStats>,
        best_individual: BestIndividualInfo,
        convergence_info: ConvergenceInfo,
        parallel_enabled: bool,
        thread_count: usize,
    ) -> Result<Self> {
        if generation_history.is_empty() {
            anyhow::bail!("世代履歴が空です");
        }

        let total_generations = generation_history.len();
        let final_stats = generation_history.last().unwrap().clone();
        
        let total_elapsed_ms: u64 = generation_history
            .iter()
            .map(|stats| stats.elapsed_ms)
            .sum();
        
        let avg_generation_time_ms = total_elapsed_ms as f64 / total_generations as f64;
        
        let fastest_generation_ms = generation_history
            .iter()
            .map(|stats| stats.elapsed_ms)
            .min()
            .unwrap_or(0);
        
        let slowest_generation_ms = generation_history
            .iter()
            .map(|stats| stats.elapsed_ms)
            .max()
            .unwrap_or(0);

        let performance_info = PerformanceInfo {
            total_elapsed_ms,
            avg_generation_time_ms,
            fastest_generation_ms,
            slowest_generation_ms,
            parallel_enabled,
            thread_count,
        };

        Ok(Self {
            total_generations,
            final_stats,
            generation_history,
            best_individual,
            convergence_info,
            performance_info,
        })
    }

    /// 統計のサマリーを生成
    pub fn summary(&self) -> String {
        let mut strategy_summary = String::new();
        let mut sorted_strategies: Vec<_> = self.final_stats.strategy_distribution.iter().collect();
        sorted_strategies.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        for (strategy, percentage) in sorted_strategies {
            strategy_summary.push_str(&format!("   - {}: {:.1}%\n", strategy, percentage));
        }
        
        format!(
            "シミュレーション結果サマリー:\n\
             - 総世代数: {}\n\
             - 最終平均適応度: {:.2}\n\
             - 最高適応度: {:.2} (第{}世代)\n\
             - 遺伝的多様性: {:.2}%\n\
             - 収束状態: {}\n\
             - 総実行時間: {:.2}秒\n\
             - 世代あたり平均時間: {:.2}ms\n\
             - 戦略分布:\n{}",
            self.total_generations,
            self.final_stats.avg_fitness,
            self.best_individual.best_fitness,
            self.best_individual.best_generation,
            self.final_stats.diversity * 100.0,
            if self.convergence_info.converged { "収束済み" } else { "未収束" },
            self.performance_info.total_elapsed_ms as f64 / 1000.0,
            self.performance_info.avg_generation_time_ms,
            strategy_summary
        )
    }

    /// 改善傾向を分析
    pub fn analyze_improvement_trend(&self) -> ImprovementTrend {
        if self.generation_history.len() < 10 {
            return ImprovementTrend::Insufficient;
        }

        let recent_avg = self.generation_history
            .iter()
            .rev()
            .take(10)
            .map(|stats| stats.avg_fitness)
            .sum::<f64>() / 10.0;

        let early_avg = self.generation_history
            .iter()
            .take(10)
            .map(|stats| stats.avg_fitness)
            .sum::<f64>() / 10.0;

        let improvement_rate = (recent_avg - early_avg) / early_avg;

        match improvement_rate {
            rate if rate > 0.1 => ImprovementTrend::Improving,
            rate if rate > 0.01 => ImprovementTrend::SlowImprovement,
            rate if rate > -0.01 => ImprovementTrend::Stable,
            _ => ImprovementTrend::Declining,
        }
    }

    /// 多様性の変化を分析
    pub fn analyze_diversity_trend(&self) -> DiversityTrend {
        if self.generation_history.len() < 5 {
            return DiversityTrend::Insufficient;
        }

        let recent_diversity = self.generation_history
            .iter()
            .rev()
            .take(5)
            .map(|stats| stats.diversity)
            .sum::<f64>() / 5.0;

        match recent_diversity {
            div if div > 0.7 => DiversityTrend::HighDiversity,
            div if div > 0.3 => DiversityTrend::ModerateDiversity,
            div if div > 0.1 => DiversityTrend::LowDiversity,
            _ => DiversityTrend::VeryLowDiversity,
        }
    }
}

/// 改善傾向の分析結果
#[derive(Debug, Clone, PartialEq)]
pub enum ImprovementTrend {
    Improving,
    SlowImprovement,
    Stable,
    Declining,
    Insufficient,
}

/// 多様性傾向の分析結果
#[derive(Debug, Clone, PartialEq)]
pub enum DiversityTrend {
    HighDiversity,
    ModerateDiversity,
    LowDiversity,
    VeryLowDiversity,
    Insufficient,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generation_stats_creation() {
        let fitness_values = vec![10.0, 20.0, 15.0, 25.0, 12.0];
        let mut strategy_distribution = std::collections::HashMap::new();
        strategy_distribution.insert("tit-for-tat".to_string(), 60.0);
        strategy_distribution.insert("always-cooperate".to_string(), 40.0);
        
        let stats = GenerationStats::new(1, &fitness_values, 0.8, 2, 100, strategy_distribution).unwrap();
        
        assert_eq!(stats.generation, 1);
        assert_eq!(stats.avg_fitness, 16.4);
        assert_eq!(stats.max_fitness, 25.0);
        assert_eq!(stats.min_fitness, 10.0);
        assert_eq!(stats.diversity, 0.8);
        assert_eq!(stats.elite_count, 2);
        assert_eq!(stats.elapsed_ms, 100);
        assert!(stats.strategy_distribution.contains_key("tit-for-tat"));
    }

    #[test]
    fn test_generation_stats_empty_fitness() {
        let fitness_values = vec![];
        let strategy_distribution = std::collections::HashMap::new();
        let result = GenerationStats::new(1, &fitness_values, 0.8, 2, 100, strategy_distribution);
        assert!(result.is_err());
    }

    #[test]
    fn test_simulation_stats_creation() {
        let mut strategy_distribution = std::collections::HashMap::new();
        strategy_distribution.insert("tit-for-tat".to_string(), 50.0);
        strategy_distribution.insert("always-cooperate".to_string(), 50.0);
        
        let gen_stats = vec![
            GenerationStats::new(0, &[10.0, 15.0], 0.9, 1, 50, strategy_distribution.clone()).unwrap(),
            GenerationStats::new(1, &[15.0, 20.0], 0.8, 1, 60, strategy_distribution).unwrap(),
        ];
        
        let best_individual = BestIndividualInfo {
            best_fitness: 20.0,
            best_generation: 1,
            best_dna: "101010".to_string(),
            description: "Best agent".to_string(),
        };
        
        let convergence_info = ConvergenceInfo {
            converged: false,
            convergence_generation: None,
            convergence_threshold: 0.95,
            convergence_window: 10,
        };
        
        let stats = SimulationStats::new(
            gen_stats,
            best_individual,
            convergence_info,
            false,
            1
        ).unwrap();
        
        assert_eq!(stats.total_generations, 2);
        assert_eq!(stats.performance_info.total_elapsed_ms, 110);
        assert_eq!(stats.performance_info.avg_generation_time_ms, 55.0);
    }

    #[test]
    fn test_improvement_trend_analysis() {
        // 改善傾向のテストケースを作成
        let mut gen_history = Vec::new();
        let strategy_distribution = std::collections::HashMap::new();
        for i in 0..20 {
            let fitness = vec![10.0 + i as f64, 15.0 + i as f64]; // 徐々に改善
            gen_history.push(GenerationStats::new(i, &fitness, 0.5, 1, 50, strategy_distribution.clone()).unwrap());
        }
        
        let best_individual = BestIndividualInfo {
            best_fitness: 34.0,
            best_generation: 19,
            best_dna: "101010".to_string(),
            description: "Best".to_string(),
        };
        
        let convergence_info = ConvergenceInfo {
            converged: false,
            convergence_generation: None,
            convergence_threshold: 0.95,
            convergence_window: 10,
        };
        
        let stats = SimulationStats::new(
            gen_history,
            best_individual,
            convergence_info,
            false,
            1
        ).unwrap();
        
        assert_eq!(stats.analyze_improvement_trend(), ImprovementTrend::Improving);
    }

    #[test]
    fn test_diversity_trend_analysis() {
        let strategy_distribution = std::collections::HashMap::new();
        let gen_stats = vec![
            GenerationStats::new(0, &[10.0, 15.0], 0.8, 1, 50, strategy_distribution.clone()).unwrap(),
            GenerationStats::new(1, &[15.0, 20.0], 0.9, 1, 60, strategy_distribution).unwrap(),
        ];
        
        let best_individual = BestIndividualInfo {
            best_fitness: 20.0,
            best_generation: 1,
            best_dna: "101010".to_string(),
            description: "Best agent".to_string(),
        };
        
        let convergence_info = ConvergenceInfo {
            converged: false,
            convergence_generation: None,
            convergence_threshold: 0.95,
            convergence_window: 10,
        };
        
        let stats = SimulationStats::new(
            gen_stats,
            best_individual,
            convergence_info,
            false,
            1
        ).unwrap();
        
        // 少ないデータなので Insufficient が返される
        assert_eq!(stats.analyze_diversity_trend(), DiversityTrend::Insufficient);
    }

    #[test]
    fn test_summary_generation() {
        let mut strategy_distribution = std::collections::HashMap::new();
        strategy_distribution.insert("tit-for-tat".to_string(), 70.0);
        strategy_distribution.insert("always-cooperate".to_string(), 30.0);
        let gen_stats = vec![
            GenerationStats::new(0, &[10.0, 15.0], 0.9, 1, 1000, strategy_distribution).unwrap(),
        ];
        
        let best_individual = BestIndividualInfo {
            best_fitness: 15.0,
            best_generation: 0,
            best_dna: "101010".to_string(),
            description: "Best agent".to_string(),
        };
        
        let convergence_info = ConvergenceInfo {
            converged: true,
            convergence_generation: Some(0),
            convergence_threshold: 0.95,
            convergence_window: 10,
        };
        
        let stats = SimulationStats::new(
            gen_stats,
            best_individual,
            convergence_info,
            false,
            1
        ).unwrap();
        
        let summary = stats.summary();
        assert!(summary.contains("総世代数: 1"));
        assert!(summary.contains("収束済み"));
    }
}