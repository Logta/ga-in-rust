/// 統合テスト
///
/// システム全体の動作を確認する包括的なテスト
use anyhow::Result;
use ga_prisoners_dilemma::*;
use std::collections::HashMap;
use tempfile::{tempdir, NamedTempFile};
use tokio::test;

#[cfg(test)]
mod end_to_end_tests {
    use super::*;

    /// 基本的なシミュレーション実行のエンドツーエンドテスト
    #[tokio::test]
    async fn test_basic_simulation_execution() -> Result<()> {
        // 最小限の設定でシミュレーションを実行
        let config = config::Config {
            simulation: config::SimulationConfig {
                default_strategy: "tit-for-tat".to_string(),
                rounds_per_match: 5,
                payoff_matrix: simulation::PayoffMatrix::standard(),
                tournament_type: config::TournamentType::RoundRobin,
            },
            genetic: config::GeneticConfig {
                population_size: 10,
                generations: 3,
                mutation_rate: 0.01,
                elite_count: 2,
                dna_length: 6,
                crossover_type: config::CrossoverType::SinglePoint,
                selection_method: config::SelectionMethod::Tournament(2),
            },
            output: config::OutputConfig::default(),
            performance: config::PerformanceConfig::default(),
            strategies: HashMap::new(),
        };

        let mut simulation = simulation::Simulation::new(config, Some(42))?;
        let result = simulation.run().await?;

        // 結果の基本的な検証
        assert!(result.total_generations <= 3);
        assert!(result.best_individual.best_fitness >= 0.0);
        assert!(!result.generation_history.is_empty());
        assert!(result.performance_info.total_elapsed_ms > 0);

        Ok(())
    }

    /// 設定ファイルを使用した実行テスト
    #[tokio::test]
    async fn test_config_file_simulation() -> Result<()> {
        let temp_dir = tempdir()?;
        let config_path = temp_dir.path().join("test_config.toml");

        // テスト用設定を作成
        let config = config::Config {
            simulation: config::SimulationConfig {
                default_strategy: "always-cooperate".to_string(),
                rounds_per_match: 3,
                payoff_matrix: simulation::PayoffMatrix::cooperative(),
                tournament_type: config::TournamentType::RoundRobin,
            },
            genetic: config::GeneticConfig {
                population_size: 8,
                generations: 2,
                mutation_rate: 0.05,
                elite_count: 1,
                dna_length: 4,
                crossover_type: config::CrossoverType::SinglePoint,
                selection_method: config::SelectionMethod::Tournament(2),
            },
            output: config::OutputConfig {
                report_interval: 1,
                verbose: false,
                auto_save: false,
                output_dir: temp_dir.path().to_path_buf(),
                format: config::OutputFormat::Json,
            },
            performance: config::PerformanceConfig::default(),
            strategies: HashMap::new(),
        };

        // 設定をファイルに保存
        config::ConfigLoader::save(&config, &config_path)?;

        // 設定ファイルから読み込み
        let loaded_config = config::Config::from_file(&config_path)?;
        
        // シミュレーション実行
        let mut simulation = simulation::Simulation::new(loaded_config, Some(123))?;
        let result = simulation.run().await?;

        // 結果検証
        assert_eq!(result.total_generations, 2);
        assert!(result.best_individual.best_fitness >= 0.0);

        Ok(())
    }

    /// パラメータスイープのテスト
    #[tokio::test]
    async fn test_parameter_sweep() -> Result<()> {
        let base_config = config::Config {
            simulation: config::SimulationConfig {
                default_strategy: "tit-for-tat".to_string(),
                rounds_per_match: 3,
                payoff_matrix: simulation::PayoffMatrix::standard(),
                tournament_type: config::TournamentType::RoundRobin,
            },
            genetic: config::GeneticConfig {
                population_size: 6,
                generations: 2,
                mutation_rate: 0.01,
                elite_count: 1,
                dna_length: 4,
                crossover_type: config::CrossoverType::SinglePoint,
                selection_method: config::SelectionMethod::Tournament(2),
            },
            output: config::OutputConfig::default(),
            performance: config::PerformanceConfig::default(),
            strategies: HashMap::new(),
        };

        let mut runner = simulation::SimulationRunner::new(base_config)?;

        // パラメータバリエーションを定義
        let variations = vec![
            ("population_size", vec![
                simulation::ParameterValue::Integer(6),
                simulation::ParameterValue::Integer(8),
            ]),
            ("mutation_rate", vec![
                simulation::ParameterValue::Float(0.01),
                simulation::ParameterValue::Float(0.05),
            ]),
        ];

        // パラメータスイープ実行
        let results = runner.run_parameter_sweep("test_sweep", variations, Some(42)).await?;

        // 結果検証
        assert_eq!(results.len(), 4); // 2×2 = 4通りの組み合わせ
        
        for (name, stats) in &results {
            assert!(name.starts_with("test_sweep_"));
            assert!(stats.total_generations <= 2);
            assert!(stats.best_individual.best_fitness >= 0.0);
        }

        Ok(())
    }

    /// 複数回実行による統計的安定性テスト
    #[tokio::test]
    async fn test_multiple_runs() -> Result<()> {
        let config = config::Config {
            simulation: config::SimulationConfig {
                default_strategy: "random".to_string(),
                rounds_per_match: 3,
                payoff_matrix: simulation::PayoffMatrix::standard(),
                tournament_type: config::TournamentType::RoundRobin,
            },
            genetic: config::GeneticConfig {
                population_size: 5,
                generations: 2,
                mutation_rate: 0.1,
                elite_count: 1,
                dna_length: 4,
                crossover_type: config::CrossoverType::SinglePoint,
                selection_method: config::SelectionMethod::Tournament(2),
            },
            output: config::OutputConfig::default(),
            performance: config::PerformanceConfig::default(),
            strategies: HashMap::new(),
        };

        let mut runner = simulation::SimulationRunner::new(config)?;

        // 5回実行
        let results = runner.run_multiple("stability_test", 5, None).await?;

        // 結果検証
        assert_eq!(results.len(), 5);
        
        let fitness_values: Vec<f64> = results.iter()
            .map(|(_, stats)| stats.best_individual.best_fitness)
            .collect();

        // 適応度のばらつきを確認（完全に同じではないはず）
        let min_fitness = fitness_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_fitness = fitness_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        
        // ランダム戦略なので、複数回実行で多少の差が出ることを期待
        // ただし、適応度がすべて0以上であることは確認
        assert!(min_fitness >= 0.0);
        assert!(max_fitness >= min_fitness);

        Ok(())
    }

    /// エラーハンドリングのテスト
    #[tokio::test]
    async fn test_error_handling() -> Result<()> {
        // 無効な設定でのシミュレーション作成
        let invalid_config = config::Config {
            simulation: config::SimulationConfig {
                default_strategy: "tit-for-tat".to_string(),
                rounds_per_match: 0, // 無効
                payoff_matrix: simulation::PayoffMatrix::standard(),
                tournament_type: config::TournamentType::RoundRobin,
            },
            genetic: config::GeneticConfig {
                population_size: 0, // 無効
                generations: 5,
                mutation_rate: 0.01,
                elite_count: 1,
                dna_length: 4,
                crossover_type: config::CrossoverType::SinglePoint,
                selection_method: config::SelectionMethod::Tournament(2),
            },
            output: config::OutputConfig::default(),
            performance: config::PerformanceConfig::default(),
            strategies: HashMap::new(),
        };

        // シミュレーション作成時にエラーになることを確認
        let result = simulation::Simulation::new(invalid_config, None);
        assert!(result.is_err());

        Ok(())
    }

    /// 結果保存・読み込みのテスト
    #[tokio::test]
    async fn test_results_persistence() -> Result<()> {
        let temp_dir = tempdir()?;
        
        let config = config::Config {
            simulation: config::SimulationConfig {
                default_strategy: "tit-for-tat".to_string(),
                rounds_per_match: 3,
                payoff_matrix: simulation::PayoffMatrix::standard(),
                tournament_type: config::TournamentType::RoundRobin,
            },
            genetic: config::GeneticConfig {
                population_size: 5,
                generations: 2,
                mutation_rate: 0.01,
                elite_count: 1,
                dna_length: 4,
                crossover_type: config::CrossoverType::SinglePoint,
                selection_method: config::SelectionMethod::Tournament(2),
            },
            output: config::OutputConfig::default(),
            performance: config::PerformanceConfig::default(),
            strategies: HashMap::new(),
        };

        let mut runner = simulation::SimulationRunner::new(config)?;

        // シミュレーション実行
        let original_result = runner.run_single(
            "persistence_test".to_string(),
            None,
            Some(42)
        ).await?;

        // 結果を保存
        runner.save_results(temp_dir.path()).await?;

        // 新しいランナーで読み込み
        let mut new_runner = simulation::SimulationRunner::new(
            config::Config::default()
        )?;
        new_runner.load_results(temp_dir.path()).await?;

        // 結果が正しく読み込まれることを確認
        let loaded_result = new_runner.get_result("persistence_test").unwrap();
        assert_eq!(loaded_result.total_generations, original_result.total_generations);
        assert_eq!(loaded_result.best_individual.best_fitness, original_result.best_individual.best_fitness);

        Ok(())
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    /// パフォーマンス測定テスト
    #[tokio::test]
    async fn test_simulation_performance() -> Result<()> {
        let config = config::Config {
            simulation: config::SimulationConfig {
                default_strategy: "tit-for-tat".to_string(),
                rounds_per_match: 10,
                payoff_matrix: simulation::PayoffMatrix::standard(),
                tournament_type: config::TournamentType::RoundRobin,
            },
            genetic: config::GeneticConfig {
                population_size: 20,
                generations: 10,
                mutation_rate: 0.01,
                elite_count: 2,
                dna_length: 8,
                crossover_type: config::CrossoverType::SinglePoint,
                selection_method: config::SelectionMethod::Tournament(2),
            },
            output: config::OutputConfig::default(),
            performance: config::PerformanceConfig::default(),
            strategies: HashMap::new(),
        };

        let start = Instant::now();
        let mut simulation = simulation::Simulation::new(config, Some(42))?;
        let result = simulation.run().await?;
        let elapsed = start.elapsed();

        // パフォーマンス指標の確認
        println!("シミュレーション実行時間: {:?}", elapsed);
        println!("総世代数: {}", result.total_generations);
        println!("世代あたり平均時間: {:.2}ms", result.performance_info.avg_generation_time_ms);

        // 基本的な性能要件（調整可能）
        assert!(elapsed.as_secs() < 30); // 30秒以内で完了
        assert!(result.performance_info.avg_generation_time_ms < 5000.0); // 世代あたり5秒以内

        Ok(())
    }

    /// メモリ使用量測定テスト
    #[tokio::test]
    async fn test_memory_usage() -> Result<()> {
        // 大きめの設定でメモリ使用量をテスト
        let config = config::Config {
            simulation: config::SimulationConfig {
                default_strategy: "tit-for-tat".to_string(),
                rounds_per_match: 5,
                payoff_matrix: simulation::PayoffMatrix::standard(),
                tournament_type: config::TournamentType::RoundRobin,
            },
            genetic: config::GeneticConfig {
                population_size: 100,
                generations: 5,
                mutation_rate: 0.01,
                elite_count: 5,
                dna_length: 16,
                crossover_type: config::CrossoverType::SinglePoint,
                selection_method: config::SelectionMethod::Tournament(3),
            },
            output: config::OutputConfig::default(),
            performance: config::PerformanceConfig::default(),
            strategies: HashMap::new(),
        };

        let mut simulation = simulation::Simulation::new(config, Some(42))?;
        let result = simulation.run().await?;

        // メモリリークがないことを間接的に確認
        // （実際のメモリ測定は環境依存のため、完了することのみ確認）
        assert!(result.total_generations <= 5);

        Ok(())
    }

    /// 並列実行のテスト（フィーチャーが有効な場合）
    #[cfg(feature = "parallel")]
    #[tokio::test]
    async fn test_parallel_execution() -> Result<()> {
        let config = config::Config {
            simulation: config::SimulationConfig {
                default_strategy: "tit-for-tat".to_string(),
                rounds_per_match: 5,
                payoff_matrix: simulation::PayoffMatrix::standard(),
                tournament_type: config::TournamentType::RoundRobin,
            },
            genetic: config::GeneticConfig {
                population_size: 50,
                generations: 3,
                mutation_rate: 0.01,
                elite_count: 2,
                dna_length: 8,
                crossover_type: config::CrossoverType::SinglePoint,
                selection_method: config::SelectionMethod::Tournament(2),
            },
            output: config::OutputConfig::default(),
            performance: config::PerformanceConfig {
                parallel: true,
                num_threads: 4,
                batch_size: 10,
                memory_limit_mb: None,
            },
            strategies: HashMap::new(),
        };

        let mut simulation = simulation::Simulation::new(config, Some(42))?;
        simulation.enable_parallel();
        let result = simulation.run().await?;

        // 並列実行が記録されていることを確認
        assert!(result.performance_info.parallel_enabled);
        assert_eq!(result.performance_info.thread_count, 4);

        Ok(())
    }
}

#[cfg(test)]
mod strategy_tests {
    use super::*;

    /// 異なる戦略での実行テスト
    #[tokio::test]
    async fn test_different_strategies() -> Result<()> {
        let strategies = vec![
            "always-cooperate",
            "always-defect", 
            "tit-for-tat",
            "random",
        ];

        for strategy in strategies {
            let config = config::Config {
                simulation: config::SimulationConfig {
                    default_strategy: strategy.to_string(),
                    rounds_per_match: 5,
                    payoff_matrix: simulation::PayoffMatrix::standard(),
                    tournament_type: config::TournamentType::RoundRobin,
                },
                genetic: config::GeneticConfig {
                    population_size: 8,
                    generations: 3,
                    mutation_rate: 0.01,
                    elite_count: 1,
                    dna_length: 6,
                    crossover_type: config::CrossoverType::SinglePoint,
                    selection_method: config::SelectionMethod::Tournament(2),
                },
                output: config::OutputConfig::default(),
                performance: config::PerformanceConfig::default(),
                strategies: HashMap::new(),
            };

            let mut simulation = simulation::Simulation::new(config, Some(42))?;
            let result = simulation.run().await?;

            // 各戦略で正常に実行されることを確認
            assert!(result.total_generations <= 3);
            assert!(result.best_individual.best_fitness >= 0.0);
            
            println!("戦略 '{}' の最高適応度: {:.2}", strategy, result.best_individual.best_fitness);
        }

        Ok(())
    }

    /// カスタム戦略のテスト
    #[tokio::test]
    async fn test_custom_strategy() -> Result<()> {
        let mut strategies = HashMap::new();
        strategies.insert(
            "test-custom".to_string(),
            config::StrategyConfig {
                description: "テスト用カスタム戦略".to_string(),
                parameters: {
                    let mut params = HashMap::new();
                    params.insert("cooperation_probability".to_string(), 
                        serde_json::Value::Number(serde_json::Number::from_f64(0.7).unwrap()));
                    params
                },
            },
        );

        let config = config::Config {
            simulation: config::SimulationConfig {
                default_strategy: "test-custom".to_string(),
                rounds_per_match: 5,
                payoff_matrix: simulation::PayoffMatrix::standard(),
                tournament_type: config::TournamentType::RoundRobin,
            },
            genetic: config::GeneticConfig {
                population_size: 6,
                generations: 2,
                mutation_rate: 0.01,
                elite_count: 1,
                dna_length: 4,
                crossover_type: config::CrossoverType::SinglePoint,
                selection_method: config::SelectionMethod::Tournament(2),
            },
            output: config::OutputConfig::default(),
            performance: config::PerformanceConfig::default(),
            strategies,
        };

        // 設定の検証
        config.validate()?;

        // 注意: 実際のカスタム戦略実行はストラテジーモジュールの実装に依存
        // ここでは設定検証のみテスト

        Ok(())
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    /// 極端に小さな設定でのテスト
    #[tokio::test]
    async fn test_minimal_configuration() -> Result<()> {
        let config = config::Config {
            simulation: config::SimulationConfig {
                default_strategy: "tit-for-tat".to_string(),
                rounds_per_match: 1, // 最小
                payoff_matrix: simulation::PayoffMatrix::standard(),
                tournament_type: config::TournamentType::RoundRobin,
            },
            genetic: config::GeneticConfig {
                population_size: 2, // 最小
                generations: 1, // 最小
                mutation_rate: 0.0, // 突然変異なし
                elite_count: 1,
                dna_length: 1, // 最小
                crossover_type: config::CrossoverType::SinglePoint,
                selection_method: config::SelectionMethod::Tournament(2),
            },
            output: config::OutputConfig::default(),
            performance: config::PerformanceConfig::default(),
            strategies: HashMap::new(),
        };

        let mut simulation = simulation::Simulation::new(config, Some(42))?;
        let result = simulation.run().await?;

        // 最小設定でも正常に動作することを確認
        assert_eq!(result.total_generations, 1);
        assert_eq!(result.generation_history.len(), 1);
        assert!(result.best_individual.best_fitness >= 0.0);

        Ok(())
    }

    /// 異なるペイオフ行列での実行テスト
    #[tokio::test]
    async fn test_different_payoff_matrices() -> Result<()> {
        let matrices = vec![
            ("standard", simulation::PayoffMatrix::standard()),
            ("cooperative", simulation::PayoffMatrix::cooperative()),
            ("competitive", simulation::PayoffMatrix::competitive()),
        ];

        for (name, matrix) in matrices {
            let config = config::Config {
                simulation: config::SimulationConfig {
                    default_strategy: "tit-for-tat".to_string(),
                    rounds_per_match: 3,
                    payoff_matrix: matrix,
                    tournament_type: config::TournamentType::RoundRobin,
                },
                genetic: config::GeneticConfig {
                    population_size: 6,
                    generations: 2,
                    mutation_rate: 0.01,
                    elite_count: 1,
                    dna_length: 4,
                    crossover_type: config::CrossoverType::SinglePoint,
                    selection_method: config::SelectionMethod::Tournament(2),
                },
                output: config::OutputConfig::default(),
                performance: config::PerformanceConfig::default(),
                strategies: HashMap::new(),
            };

            let mut simulation = simulation::Simulation::new(config, Some(42))?;
            let result = simulation.run().await?;

            // 各ペイオフ行列で正常に実行されることを確認
            assert!(result.total_generations <= 2);
            assert!(result.best_individual.best_fitness >= 0.0);
            
            println!("ペイオフ行列 '{}' の最高適応度: {:.2}", name, result.best_individual.best_fitness);
        }

        Ok(())
    }

    /// 高い突然変異率でのテスト
    #[tokio::test]
    async fn test_high_mutation_rate() -> Result<()> {
        let config = config::Config {
            simulation: config::SimulationConfig {
                default_strategy: "tit-for-tat".to_string(),
                rounds_per_match: 3,
                payoff_matrix: simulation::PayoffMatrix::standard(),
                tournament_type: config::TournamentType::RoundRobin,
            },
            genetic: config::GeneticConfig {
                population_size: 10,
                generations: 5,
                mutation_rate: 0.5, // 高い突然変異率
                elite_count: 1,
                dna_length: 6,
                crossover_type: config::CrossoverType::SinglePoint,
                selection_method: config::SelectionMethod::Tournament(2),
            },
            output: config::OutputConfig::default(),
            performance: config::PerformanceConfig::default(),
            strategies: HashMap::new(),
        };

        let mut simulation = simulation::Simulation::new(config, Some(42))?;
        let result = simulation.run().await?;

        // 高い突然変異率でも収束することを確認
        assert_eq!(result.total_generations, 5);
        assert!(result.best_individual.best_fitness >= 0.0);

        // 多様性が保たれていることを期待
        let final_diversity = result.final_stats.diversity;
        assert!(final_diversity > 0.0);

        Ok(())
    }
}

#[cfg(test)]
mod regression_tests {
    use super::*;

    /// 回帰テスト: 特定のシードで期待される結果
    #[tokio::test]
    async fn test_deterministic_results() -> Result<()> {
        let config = config::Config {
            simulation: config::SimulationConfig {
                default_strategy: "tit-for-tat".to_string(),
                rounds_per_match: 5,
                payoff_matrix: simulation::PayoffMatrix::standard(),
                tournament_type: config::TournamentType::RoundRobin,
            },
            genetic: config::GeneticConfig {
                population_size: 10,
                generations: 3,
                mutation_rate: 0.01,
                elite_count: 2,
                dna_length: 6,
                crossover_type: config::CrossoverType::SinglePoint,
                selection_method: config::SelectionMethod::Tournament(2),
            },
            output: config::OutputConfig::default(),
            performance: config::PerformanceConfig::default(),
            strategies: HashMap::new(),
        };

        // 同じシードで2回実行
        let mut simulation1 = simulation::Simulation::new(config.clone(), Some(12345))?;
        let result1 = simulation1.run().await?;

        let mut simulation2 = simulation::Simulation::new(config, Some(12345))?;
        let result2 = simulation2.run().await?;

        // 同じシードなので同じ結果になるはず
        assert_eq!(result1.total_generations, result2.total_generations);
        assert_eq!(result1.best_individual.best_fitness, result2.best_individual.best_fitness);
        assert_eq!(result1.best_individual.best_dna, result2.best_individual.best_dna);

        Ok(())
    }

    /// 以前のバージョンとの互換性テスト
    #[tokio::test]
    async fn test_legacy_compatibility() -> Result<()> {
        // レガシー設定形式での実行テスト
        // （実際の実装では、古い設定ファイル形式のサポートをテスト）
        
        let temp_dir = tempdir()?;
        let legacy_config_path = temp_dir.path().join("legacy.toml");

        // 簡略化されたレガシー設定
        let legacy_toml = r#"
[simulation]
default_strategy = "tit-for-tat"
rounds_per_match = 5

[genetic]
population_size = 8
generations = 3
mutation_rate = 0.01
elite_count = 1
dna_length = 4
"#;

        std::fs::write(&legacy_config_path, legacy_toml)?;

        // 設定を読み込み（デフォルト値で補完される）
        let config = config::Config::from_file(&legacy_config_path)?;
        
        let mut simulation = simulation::Simulation::new(config, Some(42))?;
        let result = simulation.run().await?;

        // レガシー設定でも正常に動作することを確認
        assert!(result.total_generations <= 3);
        assert!(result.best_individual.best_fitness >= 0.0);

        Ok(())
    }
}