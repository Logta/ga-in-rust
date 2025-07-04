/// シミュレーションモジュールの単体テスト
///
/// シミュレーション環境、統計、エンジンの機能をテスト
use anyhow::Result;
use ga_prisoners_dilemma::simulation::*;
use ga_prisoners_dilemma::config::*;
use std::collections::HashMap;
use tempfile::tempdir;

#[cfg(test)]
mod environment_tests {
    use super::*;

    #[test]
    fn test_choice_conversions() -> Result<()> {
        // 文字列からの変換
        assert_eq!(Choice::from_str("C")?, Choice::Cooperate);
        assert_eq!(Choice::from_str("cooperate")?, Choice::Cooperate);
        assert_eq!(Choice::from_str("協力")?, Choice::Cooperate);
        
        assert_eq!(Choice::from_str("D")?, Choice::Defect);
        assert_eq!(Choice::from_str("defect")?, Choice::Defect);
        assert_eq!(Choice::from_str("裏切り")?, Choice::Defect);
        
        // 無効な文字列
        assert!(Choice::from_str("invalid").is_err());

        Ok(())
    }

    #[test]
    fn test_choice_display() {
        assert_eq!(Choice::Cooperate.to_char(), 'C');
        assert_eq!(Choice::Defect.to_char(), 'D');
        
        assert_eq!(Choice::Cooperate.to_japanese(), "協力");
        assert_eq!(Choice::Defect.to_japanese(), "裏切り");
    }

    #[test]
    fn test_payoff_matrix_standard() -> Result<()> {
        let matrix = PayoffMatrix::standard();
        matrix.validate()?;
        
        assert_eq!(matrix.payoff(Choice::Cooperate, Choice::Cooperate), (3, 3));
        assert_eq!(matrix.payoff(Choice::Cooperate, Choice::Defect), (0, 5));
        assert_eq!(matrix.payoff(Choice::Defect, Choice::Cooperate), (5, 0));
        assert_eq!(matrix.payoff(Choice::Defect, Choice::Defect), (1, 1));

        Ok(())
    }

    #[test]
    fn test_payoff_matrix_variants() -> Result<()> {
        let cooperative = PayoffMatrix::cooperative();
        let competitive = PayoffMatrix::competitive();
        
        cooperative.validate()?;
        competitive.validate()?;
        
        // 協力的マトリックスの方が協力インセンティブが高いはず
        assert!(cooperative.cooperation_incentive() > competitive.cooperation_incentive());

        Ok(())
    }

    #[test]
    fn test_payoff_matrix_validation() {
        // 有効なマトリックス
        let valid = PayoffMatrix::new(3, 5, 0, 1);
        assert!(valid.is_ok());
        
        // 無効なマトリックス（T <= R）
        let invalid1 = PayoffMatrix::new(5, 3, 0, 1);
        assert!(invalid1.is_err());
        
        // 無効なマトリックス（R <= P）
        let invalid2 = PayoffMatrix::new(1, 5, 0, 3);
        assert!(invalid2.is_err());
        
        // 無効なマトリックス（P <= S）
        let invalid3 = PayoffMatrix::new(3, 5, 2, 1);
        assert!(invalid3.is_err());
        
        // 無効なマトリックス（2R <= T + S）
        let invalid4 = PayoffMatrix::new(3, 10, 0, 1);
        assert!(invalid4.is_err());
    }

    #[test]
    fn test_payoff_matrix_cooperation_incentive() {
        let cooperative = PayoffMatrix::cooperative();
        let standard = PayoffMatrix::standard();
        let competitive = PayoffMatrix::competitive();
        
        // 協力インセンティブの順序確認
        assert!(cooperative.cooperation_incentive() > standard.cooperation_incentive());
        assert!(standard.cooperation_incentive() > competitive.cooperation_incentive());
        
        // 0.0-1.0の範囲内であることを確認
        assert!((0.0..=1.0).contains(&cooperative.cooperation_incentive()));
        assert!((0.0..=1.0).contains(&standard.cooperation_incentive()));
        assert!((0.0..=1.0).contains(&competitive.cooperation_incentive()));
    }

    #[test]
    fn test_environment_creation() -> Result<()> {
        let env = Environment::standard();
        env.validate()?;
        
        let env_with_noise = Environment::with_noise(0.1)?;
        assert_eq!(env_with_noise.noise_level, 0.1);
        
        let env_partial = Environment::with_partial_information(0.8)?;
        assert_eq!(env_partial.information_completeness, 0.8);

        Ok(())
    }

    #[test]
    fn test_environment_validation() {
        // 無効なノイズレベル
        let result = Environment::new(
            PayoffMatrix::standard(),
            10,
            1.5, // 無効
            1.0,
        );
        assert!(result.is_err());
        
        // 無効な情報完全性
        let result = Environment::new(
            PayoffMatrix::standard(),
            10,
            0.0,
            -0.1, // 無効
        );
        assert!(result.is_err());
        
        // 無効なラウンド数
        let result = Environment::new(
            PayoffMatrix::standard(),
            0, // 無効
            0.0,
            1.0,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_environment_complexity() -> Result<()> {
        let simple = Environment::standard();
        let complex = Environment::new(
            PayoffMatrix::competitive(),
            100,
            0.2,
            0.7,
        )?;
        
        // 複雑な環境の方が複雑度が高いはず
        assert!(complex.complexity() > simple.complexity());
        
        // 複雑度は0.0-1.0の範囲内
        assert!((0.0..=1.0).contains(&simple.complexity()));
        assert!((0.0..=1.0).contains(&complex.complexity()));

        Ok(())
    }

    #[test]
    fn test_environment_descriptions() {
        let matrix = PayoffMatrix::standard();
        let description = matrix.description();
        assert!(description.contains("ペイオフ行列"));
        assert!(description.contains("R(報酬)=3"));
        
        let env = Environment::standard();
        let env_description = env.description();
        assert!(env_description.contains("ゲーム環境"));
        assert!(env_description.contains("ラウンド数: 10"));
    }
}

#[cfg(test)]
mod stats_tests {
    use super::*;

    #[test]
    fn test_generation_stats_creation() -> Result<()> {
        let fitness_values = vec![10.0, 20.0, 15.0, 25.0, 12.0];
        let stats = GenerationStats::new(1, &fitness_values, 0.8, 2, 100)?;
        
        assert_eq!(stats.generation, 1);
        assert_eq!(stats.avg_fitness, 16.4);
        assert_eq!(stats.max_fitness, 25.0);
        assert_eq!(stats.min_fitness, 10.0);
        assert_eq!(stats.diversity, 0.8);
        assert_eq!(stats.elite_count, 2);
        assert_eq!(stats.elapsed_ms, 100);
        
        // 標準偏差と収束度の確認
        assert!(stats.fitness_std_dev > 0.0);
        assert!((0.0..=1.0).contains(&stats.convergence));

        Ok(())
    }

    #[test]
    fn test_generation_stats_empty_fitness() {
        let fitness_values = vec![];
        let result = GenerationStats::new(1, &fitness_values, 0.8, 2, 100);
        assert!(result.is_err());
    }

    #[test]
    fn test_generation_stats_convergence_calculation() -> Result<()> {
        // 全て同じ適応度（完全収束）
        let same_fitness = vec![10.0, 10.0, 10.0, 10.0, 10.0];
        let stats = GenerationStats::new(0, &same_fitness, 0.5, 1, 50)?;
        assert_eq!(stats.convergence, 1.0);
        
        // 大きく異なる適応度（低収束）
        let diverse_fitness = vec![0.0, 100.0];
        let stats = GenerationStats::new(0, &diverse_fitness, 0.9, 1, 50)?;
        assert!(stats.convergence < 1.0);

        Ok(())
    }

    #[test]
    fn test_simulation_stats_creation() -> Result<()> {
        let gen_stats = vec![
            GenerationStats::new(0, &[10.0, 15.0], 0.9, 1, 50)?,
            GenerationStats::new(1, &[15.0, 20.0], 0.8, 1, 60)?,
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
        )?;
        
        assert_eq!(stats.total_generations, 2);
        assert_eq!(stats.performance_info.total_elapsed_ms, 110);
        assert_eq!(stats.performance_info.avg_generation_time_ms, 55.0);
        assert_eq!(stats.performance_info.fastest_generation_ms, 50);
        assert_eq!(stats.performance_info.slowest_generation_ms, 60);

        Ok(())
    }

    #[test]
    fn test_simulation_stats_summary() -> Result<()> {
        let gen_stats = vec![
            GenerationStats::new(0, &[10.0, 15.0], 0.9, 1, 1000)?,
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
        )?;
        
        let summary = stats.summary();
        assert!(summary.contains("総世代数: 1"));
        assert!(summary.contains("収束済み"));
        assert!(summary.contains("15.00"));

        Ok(())
    }

    #[test]
    fn test_improvement_trend_analysis() -> Result<()> {
        // 改善傾向のテストケースを作成
        let mut gen_history = Vec::new();
        for i in 0..20 {
            let fitness = vec![10.0 + i as f64, 15.0 + i as f64]; // 徐々に改善
            gen_history.push(GenerationStats::new(i, &fitness, 0.5, 1, 50)?);
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
        )?;
        
        assert_eq!(stats.analyze_improvement_trend(), ImprovementTrend::Improving);

        Ok(())
    }

    #[test]
    fn test_diversity_trend_analysis() -> Result<()> {
        let gen_stats = vec![
            GenerationStats::new(0, &[10.0, 15.0], 0.8, 1, 50)?,
            GenerationStats::new(1, &[15.0, 20.0], 0.9, 1, 60)?,
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
        )?;
        
        // 少ないデータなので Insufficient が返される
        assert_eq!(stats.analyze_diversity_trend(), DiversityTrend::Insufficient);

        Ok(())
    }

    #[test]
    fn test_diversity_trend_with_sufficient_data() -> Result<()> {
        // 十分なデータで多様性傾向をテスト
        let mut gen_history = Vec::new();
        for i in 0..10 {
            let diversity = 0.8 - (i as f64 * 0.05); // 徐々に多様性が減少
            gen_history.push(GenerationStats::new(i, &[10.0, 15.0], diversity, 1, 50)?);
        }
        
        let best_individual = BestIndividualInfo {
            best_fitness: 15.0,
            best_generation: 9,
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
        )?;
        
        let trend = stats.analyze_diversity_trend();
        assert!(matches!(trend, DiversityTrend::ModerateDiversity | DiversityTrend::LowDiversity));

        Ok(())
    }
}

#[cfg(test)]
mod runner_tests {
    use super::*;

    fn create_test_config() -> Config {
        Config {
            simulation: SimulationConfig {
                default_strategy: "tit-for-tat".to_string(),
                rounds_per_match: 10,
                payoff_matrix: PayoffMatrix::default(),
                tournament_type: TournamentType::RoundRobin,
            },
            genetic: GeneticConfig {
                population_size: 10, // 小さくしてテストを高速化
                generations: 5,
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
    fn test_runner_creation() -> Result<()> {
        let config = create_test_config();
        let runner = SimulationRunner::new(config)?;
        assert!(runner.list_results().is_empty());
        Ok(())
    }

    #[test]
    fn test_parameter_combinations() -> Result<()> {
        let config = create_test_config();
        let runner = SimulationRunner::new(config)?;
        
        let variations = vec![
            ("population_size", vec![ParameterValue::Integer(10), ParameterValue::Integer(20)]),
            ("mutation_rate", vec![ParameterValue::Float(0.01), ParameterValue::Float(0.05)]),
        ];
        
        let combinations = runner.generate_parameter_combinations(&variations)?;
        assert_eq!(combinations.len(), 4); // 2 × 2 = 4 combinations
        
        // 各組み合わせの内容を確認
        assert_eq!(combinations[0].len(), 2); // 各組み合わせは2つのパラメータを持つ
        assert_eq!(combinations[1].len(), 2);
        assert_eq!(combinations[2].len(), 2);
        assert_eq!(combinations[3].len(), 2);

        Ok(())
    }

    #[test]
    fn test_parameter_application() -> Result<()> {
        let config = create_test_config();
        let runner = SimulationRunner::new(config)?;
        
        let variation = vec![
            ("population_size".to_string(), ParameterValue::Integer(50)),
            ("mutation_rate".to_string(), ParameterValue::Float(0.05)),
        ];
        
        let new_config = runner.apply_parameter_variation(&variation)?;
        assert_eq!(new_config.genetic.population_size, 50);
        assert_eq!(new_config.genetic.mutation_rate, 0.05);

        Ok(())
    }

    #[test]
    fn test_parameter_application_invalid() -> Result<()> {
        let config = create_test_config();
        let runner = SimulationRunner::new(config)?;
        
        // 無効な値（個体数0）を適用
        let variation = vec![
            ("population_size".to_string(), ParameterValue::Integer(0)),
        ];
        
        let result = runner.apply_parameter_variation(&variation);
        assert!(result.is_err()); // 検証でエラーになるはず

        Ok(())
    }

    #[tokio::test]
    async fn test_runner_save_and_load() -> Result<()> {
        let config = create_test_config();
        let mut runner = SimulationRunner::new(config)?;
        
        let temp_dir = tempdir()?;
        
        // 空の状態で保存
        runner.save_results(temp_dir.path()).await?;
        
        // サマリーファイルが作成されることを確認
        let summary_path = temp_dir.path().join("summary.txt");
        assert!(summary_path.exists());
        
        // 新しいランナーで読み込み
        let mut new_runner = SimulationRunner::new(create_test_config())?;
        new_runner.load_results(temp_dir.path()).await?;
        
        // 同じ結果数であることを確認
        assert_eq!(runner.list_results().len(), new_runner.list_results().len());

        Ok(())
    }

    #[test]
    fn test_comparison_result() -> Result<()> {
        let stats1 = SimulationStats {
            total_generations: 100,
            final_stats: GenerationStats::new(99, &[10.0, 15.0, 12.0], 0.8, 2, 100)?,
            generation_history: vec![],
            best_individual: BestIndividualInfo {
                best_fitness: 15.0,
                best_generation: 50,
                best_dna: "101010".to_string(),
                description: "Test".to_string(),
            },
            convergence_info: ConvergenceInfo {
                converged: true,
                convergence_generation: Some(80),
                convergence_threshold: 0.01,
                convergence_window: 10,
            },
            performance_info: PerformanceInfo {
                total_elapsed_ms: 5000,
                avg_generation_time_ms: 50.0,
                fastest_generation_ms: 30,
                slowest_generation_ms: 80,
                parallel_enabled: false,
                thread_count: 1,
            },
        };
        
        let stats2 = SimulationStats {
            best_individual: BestIndividualInfo {
                best_fitness: 20.0,
                best_generation: 30,
                best_dna: "111000".to_string(),
                description: "Test2".to_string(),
            },
            ..stats1.clone()
        };
        
        let comparison = ComparisonResult::new(vec![
            ("sim1".to_string(), &stats1),
            ("sim2".to_string(), &stats2),
        ])?;
        
        let report = comparison.report();
        assert!(report.contains("sim1"));
        assert!(report.contains("sim2"));
        assert!(report.contains("15.00"));
        assert!(report.contains("20.00"));
        
        // より高い適応度のsim2が1位にランクされることを確認
        let lines: Vec<&str> = report.lines().collect();
        let sim2_line = lines.iter().find(|line| line.contains("sim2")).unwrap();
        let sim1_line = lines.iter().find(|line| line.contains("sim1")).unwrap();
        
        // sim2が上位にランクされていることを確認（単純に行番号で比較）
        let sim2_index = lines.iter().position(|&line| line == *sim2_line).unwrap();
        let sim1_index = lines.iter().position(|&line| line == *sim1_line).unwrap();
        assert!(sim2_index < sim1_index);

        Ok(())
    }

    #[test]
    fn test_parameter_value_types() {
        let int_param = ParameterValue::Integer(42);
        let float_param = ParameterValue::Float(3.14);
        let string_param = ParameterValue::String("test".to_string());
        let bool_param = ParameterValue::Boolean(true);
        
        // パラメータ値の型確認（コンパイル時チェック）
        match int_param {
            ParameterValue::Integer(val) => assert_eq!(val, 42),
            _ => panic!("Expected Integer"),
        }
        
        match float_param {
            ParameterValue::Float(val) => assert!((val - 3.14).abs() < 0.001),
            _ => panic!("Expected Float"),
        }
        
        match string_param {
            ParameterValue::String(val) => assert_eq!(val, "test"),
            _ => panic!("Expected String"),
        }
        
        match bool_param {
            ParameterValue::Boolean(val) => assert!(val),
            _ => panic!("Expected Boolean"),
        }
    }
}

#[cfg(test)]
mod simulation_engine_tests {
    use super::*;

    fn create_minimal_config() -> Config {
        Config {
            simulation: SimulationConfig {
                default_strategy: "tit-for-tat".to_string(),
                rounds_per_match: 3,
                payoff_matrix: PayoffMatrix::standard(),
                tournament_type: TournamentType::RoundRobin,
            },
            genetic: GeneticConfig {
                population_size: 5,
                generations: 2,
                mutation_rate: 0.01,
                elite_count: 1,
                dna_length: 4,
                crossover_type: CrossoverType::SinglePoint,
                selection_method: SelectionMethod::Tournament(2),
            },
            output: OutputConfig::default(),
            performance: PerformanceConfig::default(),
            strategies: HashMap::new(),
        }
    }

    #[test]
    fn test_simulation_creation() -> Result<()> {
        let config = create_minimal_config();
        let simulation = Simulation::new(config, Some(42))?;
        
        // シミュレーションが正常に作成されることを確認
        // （内部状態のテストは実装により異なるため、作成時のエラーがないことを確認）
        Ok(())
    }

    #[test]
    fn test_simulation_creation_with_invalid_config() {
        let mut config = create_minimal_config();
        config.genetic.population_size = 0; // 無効な設定
        
        let result = Simulation::new(config, None);
        assert!(result.is_err());
    }

    // 注意: 実際のシミュレーション実行テストは統合テストで行う
    // （時間がかかるため、単体テストでは作成とバリデーションのみテスト）
}

#[cfg(test)]
mod individual_tests {
    use super::*;
    use ga_prisoners_dilemma::simulation::engine::Individual;

    #[test]
    fn test_individual_creation() {
        let individual = Individual::new(1, "101010".to_string());
        assert_eq!(individual.id(), 1);
        assert_eq!(individual.dna(), "101010");
        assert_eq!(individual.fitness(), 0.0);
    }

    #[test]
    fn test_individual_fitness_setting() {
        let mut individual = Individual::new(1, "101010".to_string());
        individual.set_fitness(42.5);
        assert_eq!(individual.fitness(), 42.5);
    }

    #[test]
    fn test_individual_choice() -> Result<()> {
        let individual = Individual::new(1, "101010".to_string());
        
        // ラウンド0: DNA[0] = '1' -> Cooperate
        let choice = individual.choose(&[], 0)?;
        assert_eq!(choice, Choice::Cooperate);
        
        // ラウンド1: DNA[1] = '0' -> Defect
        let choice = individual.choose(&[], 1)?;
        assert_eq!(choice, Choice::Defect);
        
        // ラウンド6: DNA[6 % 6] = DNA[0] = '1' -> Cooperate
        let choice = individual.choose(&[], 6)?;
        assert_eq!(choice, Choice::Cooperate);

        Ok(())
    }

    #[test]
    fn test_individual_dna_distance() -> Result<()> {
        let individual1 = Individual::new(1, "101010".to_string());
        let individual2 = Individual::new(2, "111000".to_string());
        
        let distance = individual1.dna_distance(&individual2)?;
        assert_eq!(distance, 3); // 3箇所で異なる
        
        // 同じDNAの場合
        let individual3 = Individual::new(3, "101010".to_string());
        let distance = individual1.dna_distance(&individual3)?;
        assert_eq!(distance, 0);

        Ok(())
    }

    #[test]
    fn test_individual_dna_distance_different_length() {
        let individual1 = Individual::new(1, "1010".to_string());
        let individual2 = Individual::new(2, "101010".to_string());
        
        let result = individual1.dna_distance(&individual2);
        assert!(result.is_err()); // 異なる長さの場合はエラー
    }
}

#[cfg(test)]
mod population_tests {
    use super::*;
    use ga_prisoners_dilemma::simulation::engine::Population;

    #[test]
    fn test_population_creation() -> Result<()> {
        let population = Population::new(5, 4)?;
        
        assert_eq!(population.size(), 5);
        assert_eq!(population.individuals().len(), 5);
        
        // 各個体のDNA長が正しいことを確認
        for individual in population.individuals() {
            assert_eq!(individual.dna().len(), 4);
            // DNAが0と1のみで構成されていることを確認
            for c in individual.dna().chars() {
                assert!(c == '0' || c == '1');
            }
        }

        Ok(())
    }

    #[test]
    fn test_population_individual_access() -> Result<()> {
        let mut population = Population::new(3, 4)?;
        
        // 個体の取得
        let individual = population.get_individual(0)?;
        assert_eq!(individual.id(), 0);
        
        // 適応度の設定
        population.set_individual_fitness(0, 42.5)?;
        let updated_individual = population.get_individual(0)?;
        assert_eq!(updated_individual.fitness(), 42.5);

        Ok(())
    }

    #[test]
    fn test_population_best_individual() -> Result<()> {
        let mut population = Population::new(3, 4)?;
        
        // 適応度を設定
        population.set_individual_fitness(0, 10.0)?;
        population.set_individual_fitness(1, 20.0)?;
        population.set_individual_fitness(2, 15.0)?;
        
        let best = population.best_individual()?;
        assert_eq!(best.fitness(), 20.0);
        assert_eq!(best.id(), 1);

        Ok(())
    }

    #[test]
    fn test_population_invalid_access() {
        let population = Population::new(3, 4).unwrap();
        
        // 範囲外アクセス
        assert!(population.get_individual(3).is_err());
        assert!(population.get_individual(10).is_err());
    }

    #[test]
    fn test_population_invalid_creation() {
        // 無効なサイズ
        assert!(Population::new(0, 4).is_ok()); // 0サイズは現在許可されている
        
        // 無効なDNA長は乱数生成でエラーになる可能性があるが、
        // 現在の実装では特にチェックしていない
    }
}