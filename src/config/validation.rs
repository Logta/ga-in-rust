/// 設定検証ユーティリティ
///
/// 設定値の妥当性を包括的に検証する関数群
use anyhow::{Context, Result};
use crate::config::Config;

/// 設定の包括的な検証を実行
pub fn validate_config(config: &Config) -> Result<()> {
    config.validate()
        .context("設定の検証に失敗しました")
}

/// 設定値の範囲チェック
pub fn validate_ranges(config: &Config) -> Result<()> {
    // 世代数の範囲チェック
    if config.genetic.generations > 100_000 {
        anyhow::bail!(
            "世代数が大きすぎます（最大: 100,000, 現在: {}）",
            config.genetic.generations
        );
    }
    
    // 個体数の範囲チェック
    if config.genetic.population_size > 10_000 {
        anyhow::bail!(
            "個体数が大きすぎます（最大: 10,000, 現在: {}）",
            config.genetic.population_size
        );
    }
    
    // メモリ使用量の推定チェック
    let estimated_memory_mb = estimate_memory_usage(config);
    if estimated_memory_mb > 8192 { // 8GB制限
        anyhow::bail!(
            "推定メモリ使用量が制限を超えています（推定: {}MB, 制限: 8192MB）",
            estimated_memory_mb
        );
    }
    
    Ok(())
}

/// メモリ使用量を推定
fn estimate_memory_usage(config: &Config) -> u64 {
    let individual_size = config.genetic.dna_length * 8; // 8 bytes per character (rough estimate)
    let population_memory = config.genetic.population_size as u64 * individual_size as u64;
    let history_memory = config.genetic.generations as u64 * 1024; // 1KB per generation stats
    
    (population_memory + history_memory) / (1024 * 1024) // Convert to MB
}

/// 設定の組み合わせの妥当性を検証
pub fn validate_combinations(config: &Config) -> Result<()> {
    // エリート数と個体数の関係
    if config.genetic.elite_count >= config.genetic.population_size / 2 {
        anyhow::bail!(
            "エリート数が個体数の半数以上です（エリート: {}, 個体数: {}）。多様性が失われる可能性があります",
            config.genetic.elite_count,
            config.genetic.population_size
        );
    }
    
    // DNA長とゲーム設定の関係
    if config.genetic.dna_length > config.simulation.rounds_per_match * 10 {
        anyhow::bail!(
            "DNA長がラウンド数に対して長すぎます（DNA: {}, ラウンド数: {}）",
            config.genetic.dna_length,
            config.simulation.rounds_per_match
        );
    }
    
    // 突然変異率と世代数の関係
    if config.genetic.mutation_rate > 0.1 && config.genetic.generations < 100 {
        anyhow::bail!(
            "高い突然変異率（{}）に対して世代数（{}）が少なすぎます。収束しない可能性があります",
            config.genetic.mutation_rate,
            config.genetic.generations
        );
    }
    
    Ok(())
}

/// パフォーマンス設定の妥当性を検証
pub fn validate_performance_settings(config: &Config) -> Result<()> {
    // バッチサイズのチェック
    if config.performance.batch_size > config.genetic.population_size {
        anyhow::bail!(
            "バッチサイズ（{}）が個体数（{}）を超えています",
            config.performance.batch_size,
            config.genetic.population_size
        );
    }
    
    // スレッド数のチェック
    if config.performance.num_threads > 0 {
        let max_threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);
        
        if config.performance.num_threads > max_threads * 2 {
            anyhow::bail!(
                "指定されたスレッド数（{}）が利用可能なCPUコア数の2倍（{}）を超えています",
                config.performance.num_threads,
                max_threads * 2
            );
        }
    }
    
    // メモリ制限のチェック
    if let Some(memory_limit) = config.performance.memory_limit_mb {
        let estimated = estimate_memory_usage(config);
        if memory_limit < estimated as usize {
            anyhow::bail!(
                "メモリ制限（{}MB）が推定使用量（{}MB）を下回っています",
                memory_limit,
                estimated
            );
        }
    }
    
    Ok(())
}

/// 出力設定の妥当性を検証
pub fn validate_output_settings(config: &Config) -> Result<()> {
    // 出力ディレクトリの書き込み権限チェック
    if config.output.auto_save {
        let output_dir = &config.output.output_dir;
        
        // ディレクトリが存在しない場合は作成を試行
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir)
                .context(format!("出力ディレクトリの作成に失敗しました: {}", output_dir.display()))?;
        }
        
        // 書き込み権限のチェック
        let test_file = output_dir.join(".write_test");
        std::fs::write(&test_file, "test")
            .context(format!("出力ディレクトリへの書き込み権限がありません: {}", output_dir.display()))?;
        std::fs::remove_file(&test_file).ok(); // エラーは無視
    }
    
    Ok(())
}

/// 戦略設定の妥当性を検証
pub fn validate_strategy_settings(config: &Config) -> Result<()> {
    // デフォルト戦略の存在確認
    let valid_strategies = [
        "random", "always-cooperate", "always-defect",
        "tit-for-tat", "generous-tft", "pavlov",
        "generalized-reciprocity", "third-party-influence",
        "reputation", "image-scoring", "standing"
    ];
    
    if !valid_strategies.contains(&config.simulation.default_strategy.as_str()) 
        && !config.strategies.contains_key(&config.simulation.default_strategy) {
        anyhow::bail!(
            "不明な戦略が指定されています: '{}'. 利用可能な戦略: {}",
            config.simulation.default_strategy,
            valid_strategies.join(", ")
        );
    }
    
    // カスタム戦略の妥当性チェック
    for (name, strategy_config) in &config.strategies {
        if name.is_empty() {
            anyhow::bail!("戦略名が空です");
        }
        
        if strategy_config.description.is_empty() {
            anyhow::bail!("戦略'{}'の説明が空です", name);
        }
        
        // パラメータの妥当性チェック（戦略固有の検証）
        validate_strategy_parameters(name, strategy_config)?;
    }
    
    Ok(())
}

/// 戦略パラメータの妥当性を検証
fn validate_strategy_parameters(
    name: &str, 
    config: &crate::config::schema::StrategyConfig
) -> Result<()> {
    for (param_name, value) in &config.parameters {
        match param_name.as_str() {
            "cooperation_probability" | "forgiveness_rate" | "noise_level" => {
                if let Some(val) = value.as_f64() {
                    if !(0.0..=1.0).contains(&val) {
                        anyhow::bail!(
                            "戦略'{}'のパラメータ'{}'は0.0から1.0の間である必要があります: {}",
                            name, param_name, val
                        );
                    }
                } else {
                    anyhow::bail!(
                        "戦略'{}'のパラメータ'{}'は数値である必要があります",
                        name, param_name
                    );
                }
            }
            "memory_length" | "threshold" => {
                if let Some(val) = value.as_i64() {
                    if val < 0 || val > 1000 {
                        anyhow::bail!(
                            "戦略'{}'のパラメータ'{}'は0から1000の間である必要があります: {}",
                            name, param_name, val
                        );
                    }
                } else {
                    anyhow::bail!(
                        "戦略'{}'のパラメータ'{}'は整数である必要があります",
                        name, param_name
                    );
                }
            }
            _ => {
                // 未知のパラメータは警告のみ
                tracing::warn!(
                    "戦略'{}'に未知のパラメータがあります: '{}'",
                    name, param_name
                );
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::schema::*;
    use std::collections::HashMap;

    fn create_test_config() -> Config {
        Config {
            simulation: SimulationConfig {
                default_strategy: "tit-for-tat".to_string(),
                rounds_per_match: 10,
                payoff_matrix: PayoffMatrix::default(),
                tournament_type: TournamentType::RoundRobin,
            },
            genetic: GeneticConfig {
                population_size: 100,
                generations: 1000,
                mutation_rate: 0.01,
                elite_count: 2,
                dna_length: 8,
                crossover_type: CrossoverType::SinglePoint,
                selection_method: SelectionMethod::Tournament(2),
            },
            output: OutputConfig::default(),
            performance: PerformanceConfig::default(),
            strategies: HashMap::new(),
        }
    }

    #[test]
    fn test_validate_ranges_valid() {
        let config = create_test_config();
        assert!(validate_ranges(&config).is_ok());
    }

    #[test]
    fn test_validate_ranges_invalid_generations() {
        let mut config = create_test_config();
        config.genetic.generations = 200_000;
        assert!(validate_ranges(&config).is_err());
    }

    #[test]
    fn test_validate_ranges_invalid_population() {
        let mut config = create_test_config();
        config.genetic.population_size = 20_000;
        assert!(validate_ranges(&config).is_err());
    }

    #[test]
    fn test_validate_combinations_valid() {
        let config = create_test_config();
        assert!(validate_combinations(&config).is_ok());
    }

    #[test]
    fn test_validate_combinations_invalid_elite() {
        let mut config = create_test_config();
        config.genetic.elite_count = 60; // Half of population
        assert!(validate_combinations(&config).is_err());
    }

    #[test]
    fn test_validate_strategy_settings_valid() {
        let config = create_test_config();
        assert!(validate_strategy_settings(&config).is_ok());
    }

    #[test]
    fn test_validate_strategy_settings_invalid() {
        let mut config = create_test_config();
        config.simulation.default_strategy = "unknown-strategy".to_string();
        assert!(validate_strategy_settings(&config).is_err());
    }

    #[test]
    fn test_memory_estimation() {
        let config = create_test_config();
        let memory = estimate_memory_usage(&config);
        assert!(memory > 0);
        assert!(memory < 1000); // Should be reasonable for small config
    }

    #[test]
    fn test_validate_performance_settings() {
        let config = create_test_config();
        assert!(validate_performance_settings(&config).is_ok());
    }
}