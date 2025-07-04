/// 設定管理モジュールの単体テスト
///
/// 設定の読み込み、検証、保存機能をテスト
use anyhow::Result;
use ga_prisoners_dilemma::config::*;
use serde_json;
use std::collections::HashMap;
use tempfile::{tempdir, NamedTempFile};

#[cfg(test)]
mod config_schema_tests {
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
    fn test_config_creation() {
        let config = create_test_config();
        assert_eq!(config.simulation.default_strategy, "tit-for-tat");
        assert_eq!(config.genetic.population_size, 100);
        assert_eq!(config.genetic.generations, 1000);
    }

    #[test]
    fn test_config_validation_success() -> Result<()> {
        let config = create_test_config();
        config.validate()?;
        Ok(())
    }

    #[test]
    fn test_config_validation_failures() {
        let mut config = create_test_config();

        // 無効な個体数
        config.genetic.population_size = 0;
        assert!(config.validate().is_err());

        // 無効な突然変異率
        config = create_test_config();
        config.genetic.mutation_rate = 1.5;
        assert!(config.validate().is_err());

        // 無効なDNA長
        config = create_test_config();
        config.genetic.dna_length = 0;
        assert!(config.validate().is_err());

        // 無効なエリートサイズ
        config = create_test_config();
        config.genetic.elite_count = 150;
        assert!(config.validate().is_err());

        // 無効な世代数
        config = create_test_config();
        config.genetic.generations = 0;
        assert!(config.validate().is_err());

        // 無効なラウンド数
        config = create_test_config();
        config.simulation.rounds_per_match = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_default_configs() {
        let default_config = Config::default();
        assert!(default_config.validate().is_ok());

        let simulation_config = SimulationConfig::default();
        assert!(!simulation_config.default_strategy.is_empty());

        let genetic_config = GeneticConfig::default();
        assert!(genetic_config.population_size > 0);
        assert!(genetic_config.generations > 0);

        let output_config = OutputConfig::default();
        assert!(output_config.report_interval > 0);

        let performance_config = PerformanceConfig::default();
        assert!(performance_config.batch_size > 0);
    }

    #[test]
    fn test_payoff_matrix() -> Result<()> {
        // 標準ペイオフ行列
        let standard = PayoffMatrix::standard();
        standard.validate()?;
        assert_eq!(standard.reward, 3);
        assert_eq!(standard.temptation, 5);
        assert_eq!(standard.sucker, 0);
        assert_eq!(standard.punishment, 1);

        // 協力的ペイオフ行列
        let cooperative = PayoffMatrix::cooperative();
        cooperative.validate()?;
        assert!(cooperative.cooperation_incentive() > standard.cooperation_incentive());

        // 競争的ペイオフ行列
        let competitive = PayoffMatrix::competitive();
        competitive.validate()?;
        assert!(competitive.cooperation_incentive() < standard.cooperation_incentive());

        Ok(())
    }

    #[test]
    fn test_payoff_matrix_validation() {
        // 有効なマトリックス
        let valid = PayoffMatrix::new(3, 5, 0, 1);
        assert!(valid.is_ok());

        // T > R 違反
        let invalid1 = PayoffMatrix::new(5, 3, 0, 1);
        assert!(invalid1.is_err());

        // R > P 違反
        let invalid2 = PayoffMatrix::new(1, 5, 0, 3);
        assert!(invalid2.is_err());

        // P > S 違反
        let invalid3 = PayoffMatrix::new(3, 5, 2, 1);
        assert!(invalid3.is_err());

        // 2R > T + S 違反
        let invalid4 = PayoffMatrix::new(3, 10, 0, 1);
        assert!(invalid4.is_err());
    }

    #[test]
    fn test_tournament_types() {
        let round_robin = TournamentType::RoundRobin;
        let random_pairing = TournamentType::RandomPairing(50);
        let swiss = TournamentType::Swiss;

        // デフォルトはラウンドロビン
        assert!(matches!(TournamentType::default(), TournamentType::RoundRobin));
    }

    #[test]
    fn test_crossover_types() {
        let single_point = CrossoverType::SinglePoint;
        let two_point = CrossoverType::TwoPoint;
        let uniform = CrossoverType::Uniform(0.5);

        // デフォルトは一点交叉
        assert!(matches!(CrossoverType::default(), CrossoverType::SinglePoint));
    }

    #[test]
    fn test_selection_methods() {
        let roulette = SelectionMethod::Roulette;
        let tournament = SelectionMethod::Tournament(3);
        let rank = SelectionMethod::Rank;
        let elite = SelectionMethod::Elite;

        // デフォルトはトーナメント選択
        assert!(matches!(SelectionMethod::default(), SelectionMethod::Tournament(2)));
    }

    #[test]
    fn test_output_formats() {
        let json = OutputFormat::Json;
        let csv = OutputFormat::Csv;
        let yaml = OutputFormat::Yaml;

        // デフォルトはJSON
        assert!(matches!(OutputFormat::default(), OutputFormat::Json));
    }
}

#[cfg(test)]
mod config_loader_tests {
    use super::*;

    #[test]
    fn test_config_loader_creation() {
        let loader = ConfigLoader::new();
        assert!(!loader.find_config_path().is_some()); // テスト環境では設定ファイルは存在しない
    }

    #[test]
    fn test_config_loader_default_path() -> Result<()> {
        let path = ConfigLoader::default_config_path()?;
        assert!(path.to_string_lossy().contains("ga-prisoners-dilemma"));
        assert!(path.to_string_lossy().ends_with("config.toml"));
        Ok(())
    }

    #[test]
    fn test_config_load_default() -> Result<()> {
        let loader = ConfigLoader::new();
        let config = loader.load(None)?; // デフォルト設定を読み込み
        config.validate()?;
        Ok(())
    }

    #[test]
    fn test_config_file_formats() -> Result<()> {
        let config = create_test_config();

        // TOML形式でのテスト
        let temp_toml = NamedTempFile::with_suffix(".toml")?;
        let toml_content = toml::to_string_pretty(&config)?;
        std::fs::write(temp_toml.path(), toml_content)?;
        
        let loaded_toml = Config::from_file(temp_toml.path())?;
        assert_eq!(loaded_toml.genetic.population_size, config.genetic.population_size);

        // JSON形式でのテスト
        let temp_json = NamedTempFile::with_suffix(".json")?;
        let json_content = serde_json::to_string_pretty(&config)?;
        std::fs::write(temp_json.path(), json_content)?;
        
        let loaded_json = Config::from_file(temp_json.path())?;
        assert_eq!(loaded_json.genetic.population_size, config.genetic.population_size);

        // YAML形式でのテスト
        let temp_yaml = NamedTempFile::with_suffix(".yaml")?;
        let yaml_content = serde_yaml::to_string(&config)?;
        std::fs::write(temp_yaml.path(), yaml_content)?;
        
        let loaded_yaml = Config::from_file(temp_yaml.path())?;
        assert_eq!(loaded_yaml.genetic.population_size, config.genetic.population_size);

        Ok(())
    }

    #[test]
    fn test_config_save() -> Result<()> {
        let config = create_test_config();
        let temp_dir = tempdir()?;
        
        // TOML形式で保存
        let toml_path = temp_dir.path().join("test.toml");
        ConfigLoader::save(&config, &toml_path)?;
        assert!(toml_path.exists());
        
        // 保存したファイルを読み込んで検証
        let loaded = Config::from_file(&toml_path)?;
        assert_eq!(loaded.genetic.population_size, config.genetic.population_size);

        // JSON形式で保存
        let json_path = temp_dir.path().join("test.json");
        ConfigLoader::save(&config, &json_path)?;
        assert!(json_path.exists());

        // YAML形式で保存
        let yaml_path = temp_dir.path().join("test.yaml");
        ConfigLoader::save(&config, &yaml_path)?;
        assert!(yaml_path.exists());

        Ok(())
    }

    #[test]
    fn test_config_file_errors() {
        // 存在しないファイル
        let result = Config::from_file("nonexistent.toml");
        assert!(result.is_err());

        // 無効な拡張子
        let temp_file = NamedTempFile::with_suffix(".txt").unwrap();
        let result = Config::from_file(temp_file.path());
        assert!(result.is_err());

        // 無効なJSONファイル
        let temp_json = NamedTempFile::with_suffix(".json").unwrap();
        std::fs::write(temp_json.path(), "invalid json content").unwrap();
        let result = Config::from_file(temp_json.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_config_search_paths() {
        let mut loader = ConfigLoader::new();
        let custom_path = std::path::PathBuf::from("/custom/path");
        loader.add_search_path(custom_path.clone());
        
        // カスタムパスが最初に検索されることを間接的に確認
        // （実際のファイルは作らないが、ロジックの動作確認）
        assert!(loader.find_config_path().is_none()); // ファイルが存在しないのでNone
    }
}

#[cfg(test)]
mod config_validation_tests {
    use super::*;

    #[test]
    fn test_validate_ranges() -> Result<()> {
        let config = create_test_config();
        validate_ranges(&config)?;

        // 無効な世代数
        let mut invalid_config = config.clone();
        invalid_config.genetic.generations = 200_000;
        assert!(validate_ranges(&invalid_config).is_err());

        // 無効な個体数
        let mut invalid_config = config.clone();
        invalid_config.genetic.population_size = 20_000;
        assert!(validate_ranges(&invalid_config).is_err());

        Ok(())
    }

    #[test]
    fn test_validate_combinations() -> Result<()> {
        let config = create_test_config();
        validate_combinations(&config)?;

        // エリート数が個体数の半数以上
        let mut invalid_config = config.clone();
        invalid_config.genetic.elite_count = 60;
        assert!(validate_combinations(&invalid_config).is_err());

        // DNA長がラウンド数に対して長すぎる
        let mut invalid_config = config.clone();
        invalid_config.genetic.dna_length = 200;
        assert!(validate_combinations(&invalid_config).is_err());

        // 高い突然変異率と少ない世代数
        let mut invalid_config = config.clone();
        invalid_config.genetic.mutation_rate = 0.2;
        invalid_config.genetic.generations = 50;
        assert!(validate_combinations(&invalid_config).is_err());

        Ok(())
    }

    #[test]
    fn test_validate_performance_settings() -> Result<()> {
        let config = create_test_config();
        validate_performance_settings(&config)?;

        // バッチサイズが個体数を超える
        let mut invalid_config = config.clone();
        invalid_config.performance.batch_size = 200;
        assert!(validate_performance_settings(&invalid_config).is_err());

        // 過大なスレッド数
        let mut invalid_config = config.clone();
        invalid_config.performance.num_threads = 1000;
        assert!(validate_performance_settings(&invalid_config).is_err());

        // メモリ制限が推定使用量を下回る
        let mut invalid_config = config.clone();
        invalid_config.performance.memory_limit_mb = Some(1); // 1MB制限
        assert!(validate_performance_settings(&invalid_config).is_err());

        Ok(())
    }

    #[test]
    fn test_validate_output_settings() -> Result<()> {
        let config = create_test_config();
        validate_output_settings(&config)?;

        // 自動保存が有効な場合のディレクトリ作成テスト
        let temp_dir = tempdir()?;
        let mut config_with_save = config.clone();
        config_with_save.output.auto_save = true;
        config_with_save.output.output_dir = temp_dir.path().join("test_output");
        
        validate_output_settings(&config_with_save)?;
        assert!(config_with_save.output.output_dir.exists());

        Ok(())
    }

    #[test]
    fn test_validate_strategy_settings() -> Result<()> {
        let config = create_test_config();
        validate_strategy_settings(&config)?;

        // 無効な戦略名
        let mut invalid_config = config.clone();
        invalid_config.simulation.default_strategy = "unknown-strategy".to_string();
        assert!(validate_strategy_settings(&invalid_config).is_err());

        // カスタム戦略のテスト
        let mut config_with_custom = config.clone();
        config_with_custom.simulation.default_strategy = "custom-strategy".to_string();
        config_with_custom.strategies.insert(
            "custom-strategy".to_string(),
            StrategyConfig {
                description: "Custom test strategy".to_string(),
                parameters: {
                    let mut params = HashMap::new();
                    params.insert("cooperation_probability".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.5).unwrap()));
                    params
                },
            },
        );
        
        validate_strategy_settings(&config_with_custom)?;

        Ok(())
    }

    #[test]
    fn test_strategy_parameter_validation() {
        let mut config = create_test_config();
        
        // 有効なパラメータ
        config.strategies.insert(
            "test-strategy".to_string(),
            StrategyConfig {
                description: "Test strategy".to_string(),
                parameters: {
                    let mut params = HashMap::new();
                    params.insert("cooperation_probability".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.7).unwrap()));
                    params.insert("memory_length".to_string(), serde_json::Value::Number(serde_json::Number::from(5)));
                    params
                },
            },
        );
        assert!(validate_strategy_settings(&config).is_ok());

        // 無効なパラメータ値
        config.strategies.insert(
            "invalid-strategy".to_string(),
            StrategyConfig {
                description: "Invalid strategy".to_string(),
                parameters: {
                    let mut params = HashMap::new();
                    params.insert("cooperation_probability".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(1.5).unwrap())); // 範囲外
                    params
                },
            },
        );
        assert!(validate_strategy_settings(&config).is_err());
    }

    #[test]
    fn test_memory_estimation() {
        let config = create_test_config();
        let memory = super::validation::estimate_memory_usage(&config);
        assert!(memory > 0);
        assert!(memory < 1000); // 小さな設定なので1GB未満のはず
    }
}

#[cfg(test)]
mod environment_variable_tests {
    use super::*;
    use std::env;

    #[test]
    fn test_load_from_env() -> Result<()> {
        // 環境変数を設定
        env::set_var("GA_PD_GENERATIONS", "500");
        env::set_var("GA_PD_POPULATION", "50");
        env::set_var("GA_PD_MUTATION_RATE", "0.05");
        env::set_var("GA_PD_STRATEGY", "always-cooperate");

        let config = load_from_env()?;
        
        assert_eq!(config.genetic.generations, 500);
        assert_eq!(config.genetic.population_size, 50);
        assert_eq!(config.genetic.mutation_rate, 0.05);
        assert_eq!(config.simulation.default_strategy, "always-cooperate");

        // 環境変数をクリア
        env::remove_var("GA_PD_GENERATIONS");
        env::remove_var("GA_PD_POPULATION");
        env::remove_var("GA_PD_MUTATION_RATE");
        env::remove_var("GA_PD_STRATEGY");

        Ok(())
    }

    #[test]
    fn test_invalid_env_values() {
        // 無効な値を設定
        env::set_var("GA_PD_GENERATIONS", "invalid_number");
        
        let result = load_from_env();
        assert!(result.is_err());

        env::remove_var("GA_PD_GENERATIONS");
    }
}

#[cfg(test)]
mod config_merge_tests {
    use super::*;

    #[test]
    fn test_merge_configs() {
        let base = create_test_config();
        let mut overrides = Config::default();
        overrides.genetic.population_size = 200;
        overrides.genetic.mutation_rate = 0.05;

        let merged = merge_configs(base.clone(), overrides);
        
        assert_eq!(merged.genetic.population_size, 200);
        assert_eq!(merged.genetic.mutation_rate, 0.05);
        // その他の値はベースの値をそのまま使用
        assert_eq!(merged.genetic.generations, base.genetic.generations);
    }
}