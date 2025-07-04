/// コアモジュールの単体テスト
///
/// 新しいアーキテクチャのコア機能をテスト
use anyhow::Result;
use ga_prisoners_dilemma::core::*;
use std::time::Duration;

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_population_size_validation() -> Result<()> {
        // 有効な個体数
        assert!(validate_population_size(1).is_ok());
        assert!(validate_population_size(100).is_ok());
        assert!(validate_population_size(1000).is_ok());

        // 無効な個体数
        assert!(validate_population_size(0).is_err());

        Ok(())
    }

    #[test]
    fn test_mutation_rate_validation() -> Result<()> {
        // 有効な突然変異率
        assert!(validate_mutation_rate(0.0).is_ok());
        assert!(validate_mutation_rate(0.01).is_ok());
        assert!(validate_mutation_rate(0.5).is_ok());
        assert!(validate_mutation_rate(1.0).is_ok());

        // 無効な突然変異率
        assert!(validate_mutation_rate(-0.1).is_err());
        assert!(validate_mutation_rate(1.1).is_err());
        assert!(validate_mutation_rate(f64::NAN).is_err());

        Ok(())
    }

    #[test]
    fn test_dna_validation() -> Result<()> {
        // 有効なDNA
        assert!(validate_dna("0").is_ok());
        assert!(validate_dna("1").is_ok());
        assert!(validate_dna("010101").is_ok());
        assert!(validate_dna("111000").is_ok());

        // 無効なDNA
        assert!(validate_dna("").is_err());
        assert!(validate_dna("012").is_err());
        assert!(validate_dna("abc").is_err());
        assert!(validate_dna("10a01").is_err());

        Ok(())
    }

    #[test]
    fn test_elite_size_validation() -> Result<()> {
        // 有効なエリートサイズ
        assert!(validate_elite_size(0, 10).is_ok());
        assert!(validate_elite_size(2, 10).is_ok());
        assert!(validate_elite_size(9, 10).is_ok());

        // 無効なエリートサイズ
        assert!(validate_elite_size(10, 10).is_err());
        assert!(validate_elite_size(15, 10).is_err());

        Ok(())
    }

    #[test]
    fn test_crossover_point_validation() -> Result<()> {
        // 有効な交叉点
        assert!(validate_crossover_point(0, 6).is_ok());
        assert!(validate_crossover_point(3, 6).is_ok());
        assert!(validate_crossover_point(5, 6).is_ok());

        // 無効な交叉点
        assert!(validate_crossover_point(6, 6).is_err());
        assert!(validate_crossover_point(10, 6).is_err());

        Ok(())
    }

    #[test]
    fn test_generation_count_validation() -> Result<()> {
        // 有効な世代数
        assert!(validate_generation_count(1).is_ok());
        assert!(validate_generation_count(1000).is_ok());

        // 無効な世代数
        assert!(validate_generation_count(0).is_err());

        Ok(())
    }

    #[test]
    fn test_dna_length_validation() -> Result<()> {
        // 有効なDNA長
        assert!(validate_dna_length(1).is_ok());
        assert!(validate_dna_length(32).is_ok());
        assert!(validate_dna_length(64).is_ok());

        // 無効なDNA長
        assert!(validate_dna_length(0).is_err());
        assert!(validate_dna_length(65).is_err());

        Ok(())
    }

    #[test]
    fn test_game_config_validation() -> Result<()> {
        // 有効なゲーム設定
        assert!(validate_game_config(1).is_ok());
        assert!(validate_game_config(10).is_ok());
        assert!(validate_game_config(100).is_ok());

        // 無効なゲーム設定
        assert!(validate_game_config(0).is_err());

        Ok(())
    }

    #[test]
    fn test_report_interval_validation() -> Result<()> {
        // 有効なレポート間隔
        assert!(validate_report_interval(1, 100).is_ok());
        assert!(validate_report_interval(10, 100).is_ok());
        assert!(validate_report_interval(100, 100).is_ok());

        // 無効なレポート間隔
        assert!(validate_report_interval(0, 100).is_err());
        assert!(validate_report_interval(101, 100).is_err());

        Ok(())
    }
}

#[cfg(test)]
mod random_tests {
    use super::*;

    #[test]
    fn test_random_generator_creation() -> Result<()> {
        let rng = RandomGenerator::new(Some(42));
        assert!(rng.gen_f64().is_ok());

        let rng_no_seed = RandomGenerator::new(None);
        assert!(rng_no_seed.gen_f64().is_ok());

        Ok(())
    }

    #[test]
    fn test_deterministic_random() -> Result<()> {
        let rng1 = RandomGenerator::new(Some(42));
        let rng2 = RandomGenerator::new(Some(42));

        let val1 = rng1.gen_f64()?;
        let val2 = rng2.gen_f64()?;

        // 同じシードなので同じ値が生成されるはず
        assert_eq!(val1, val2);

        Ok(())
    }

    #[test]
    fn test_gen_range() -> Result<()> {
        let rng = RandomGenerator::new(Some(42));

        // 範囲内の値が生成されることをテスト
        for _ in 0..100 {
            let val = rng.gen_range(10)?;
            assert!(val < 10);
        }

        // 無効な範囲はエラー
        assert!(rng.gen_range(0).is_err());

        Ok(())
    }

    #[test]
    fn test_gen_bool() -> Result<()> {
        let rng = RandomGenerator::new(Some(42));

        // 確率0では常にfalse
        for _ in 0..10 {
            assert!(!rng.gen_bool(0.0)?);
        }

        // 確率1では常にtrue
        for _ in 0..10 {
            assert!(rng.gen_bool(1.0)?);
        }

        // 無効な確率はエラー
        assert!(rng.gen_bool(-0.1).is_err());
        assert!(rng.gen_bool(1.1).is_err());

        Ok(())
    }

    #[test]
    fn test_choose() -> Result<()> {
        let rng = RandomGenerator::new(Some(42));
        let items = [1, 2, 3, 4, 5];

        for _ in 0..100 {
            let chosen = rng.choose(&items)?;
            assert!(items.contains(chosen));
        }

        // 空の配列はエラー
        let empty: &[i32] = &[];
        assert!(rng.choose(empty).is_err());

        Ok(())
    }

    #[test]
    fn test_shuffle() -> Result<()> {
        let rng = RandomGenerator::new(Some(42));
        let mut items = vec![1, 2, 3, 4, 5];
        let original = items.clone();

        rng.shuffle(&mut items)?;

        // 要素は同じだが順序が変わる可能性がある
        items.sort();
        assert_eq!(items, original);

        Ok(())
    }

    #[test]
    fn test_weighted_choice() -> Result<()> {
        let rng = RandomGenerator::new(Some(42));

        // 重み付き選択のテスト
        let items = [("A", 0.1), ("B", 0.9)];
        
        let mut count_b = 0;
        for _ in 0..1000 {
            if rng.weighted_choice(&items)? == &"B" {
                count_b += 1;
            }
        }

        // Bが選ばれる確率が高いはず（統計的テスト）
        assert!(count_b > 800);

        // エラーケース
        let empty: &[(&str, f64)] = &[];
        assert!(rng.weighted_choice(empty).is_err());

        let zero_weight = [("A", 0.0), ("B", 0.0)];
        assert!(rng.weighted_choice(&zero_weight).is_err());

        Ok(())
    }

    #[test]
    fn test_distributions() -> Result<()> {
        let rng = RandomGenerator::new(Some(42));

        // 正規分布
        let normal = rng.gen_normal(0.0, 1.0)?;
        assert!(normal.is_finite());

        // 指数分布
        let exp = rng.gen_exponential(1.0)?;
        assert!(exp >= 0.0 && exp.is_finite());
        assert!(rng.gen_exponential(-1.0).is_err());

        // ポアソン分布
        let poisson = rng.gen_poisson(2.0)?;
        assert!(poisson < 1000); // 常識的な範囲
        assert!(rng.gen_poisson(-1.0).is_err());

        Ok(())
    }

    #[test]
    fn test_utils_functions() -> Result<()> {
        init_default_rng(123);

        assert!(random::utils::random().is_ok());
        assert!(random::utils::random_range(10).is_ok());
        assert!(random::utils::random_bool(0.5).is_ok());

        let items = [1, 2, 3];
        assert!(random::utils::choose(&items).is_ok());

        let mut items = vec![1, 2, 3, 4, 5];
        assert!(random::utils::shuffle(&mut items).is_ok());

        let weighted = [("A", 0.3), ("B", 0.7)];
        assert!(random::utils::weighted_choice(&weighted).is_ok());

        Ok(())
    }
}

#[cfg(test)]
mod logging_tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_log_config_variants() {
        let dev_config = LogConfig::development();
        assert!(matches!(dev_config.level, LogLevel::Debug));
        assert!(dev_config.file_output);
        assert!(dev_config.include_spans);

        let prod_config = LogConfig::production();
        assert!(matches!(prod_config.level, LogLevel::Info));
        assert!(prod_config.json_format);

        let quiet_config = LogConfig::quiet();
        assert!(matches!(quiet_config.level, LogLevel::Error));
        assert!(!quiet_config.file_output);

        let verbose_config = LogConfig::verbose();
        assert!(matches!(verbose_config.level, LogLevel::Trace));
        assert!(verbose_config.include_spans);
    }

    #[test]
    fn test_log_level_conversion() {
        use tracing::Level;
        
        assert_eq!(Level::from(LogLevel::Info), Level::INFO);
        assert_eq!(Level::from(LogLevel::Debug), Level::DEBUG);
        assert_eq!(Level::from(LogLevel::Error), Level::ERROR);
        assert_eq!(Level::from(LogLevel::Warn), Level::WARN);
        assert_eq!(Level::from(LogLevel::Trace), Level::TRACE);
    }

    #[test]
    fn test_log_rotator() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let log_path = temp_dir.path().join("test.log");
        
        // テスト用のログファイルを作成（2MB）
        std::fs::write(&log_path, "a".repeat(1024 * 1024 * 2))?;
        
        let rotator = logging::LogRotator::new(&log_path, 1, 3); // 1MB, 3ファイル
        rotator.check_and_rotate()?;
        
        // 回転されたファイルが存在することを確認
        let backup1 = temp_dir.path().join("test.log.1");
        assert!(backup1.exists());
        
        Ok(())
    }
}

#[cfg(test)]
mod traits_tests {
    use super::*;
    use ga_prisoners_dilemma::core::traits::Statistics;

    #[test]
    fn test_statistics_trait() {
        let data: Vec<Points> = vec![1, 2, 3, 4, 5];
        
        // 平均値のテスト
        assert_eq!(data.mean(), 3.0);
        
        // 最大値・最小値のテスト
        assert_eq!(data.max(), Some(5));
        assert_eq!(data.min(), Some(1));
        
        // 標準偏差のテスト
        let std_dev = data.std_deviation();
        assert!(std_dev > 0.0);
        assert!((std_dev - 1.58).abs() < 0.01); // 期待値との比較
        
        // 空のベクタのテスト
        let empty: Vec<Points> = vec![];
        assert_eq!(empty.mean(), 0.0);
        assert_eq!(empty.max(), None);
        assert_eq!(empty.min(), None);
        assert_eq!(empty.std_deviation(), 0.0);
        
        // 単一要素のテスト
        let single = vec![5];
        assert_eq!(single.mean(), 5.0);
        assert_eq!(single.max(), Some(5));
        assert_eq!(single.min(), Some(5));
        assert_eq!(single.std_deviation(), 0.0);
    }

    #[test]
    fn test_statistics_with_large_dataset() {
        // 大きなデータセットでのテスト
        let data: Vec<Points> = (1..=1000).collect();
        
        assert_eq!(data.mean(), 500.5);
        assert_eq!(data.max(), Some(1000));
        assert_eq!(data.min(), Some(1));
        
        let std_dev = data.std_deviation();
        assert!(std_dev > 288.0 && std_dev < 290.0); // 期待値: 約288.675
    }

    #[test]
    fn test_statistics_edge_cases() {
        // 同じ値のデータセット
        let same_values = vec![5, 5, 5, 5, 5];
        assert_eq!(same_values.mean(), 5.0);
        assert_eq!(same_values.std_deviation(), 0.0);
        
        // 極端な値を含むデータセット
        let extreme_values = vec![0, 1000000];
        assert_eq!(extreme_values.mean(), 500000.0);
        assert!(extreme_values.std_deviation() > 0.0);
    }
}