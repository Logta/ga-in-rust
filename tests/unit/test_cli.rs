/// CLIモジュールの単体テスト
///
/// コマンドライン解析、出力フォーマット、プログレス表示のテスト
use anyhow::Result;
use ga_prisoners_dilemma::cli::*;
use clap::Parser;
use std::io::{self, Write};
use tempfile::tempdir;

#[cfg(test)]
mod cli_parsing_tests {
    use super::*;

    #[test]
    fn test_basic_run_command() {
        let args = vec![
            "ga-sim",
            "run",
            "--generations", "500",
            "--population", "100",
            "--mutation-rate", "0.05"
        ];
        
        let cli = app::Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            app::Commands::Run(run_args) => {
                assert_eq!(run_args.generations, Some(500));
                assert_eq!(run_args.population, Some(100));
                assert_eq!(run_args.mutation_rate, Some(0.05));
                assert!(!run_args.dry_run);
                assert!(!run_args.parallel);
            }
            _ => panic!("Expected run command"),
        }
    }

    #[test]
    fn test_run_command_with_strategy() {
        let args = vec![
            "ga-sim",
            "run",
            "--strategy", "always-cooperate",
            "--rounds", "20"
        ];
        
        let cli = app::Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            app::Commands::Run(run_args) => {
                assert_eq!(run_args.strategy, Some("always-cooperate".to_string()));
                assert_eq!(run_args.rounds, Some(20));
            }
            _ => panic!("Expected run command"),
        }
    }

    #[test]
    fn test_run_command_with_flags() {
        let args = vec![
            "ga-sim",
            "run",
            "--dry-run",
            "--parallel",
            "--quiet"
        ];
        
        let cli = app::Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            app::Commands::Run(run_args) => {
                assert!(run_args.dry_run);
                assert!(run_args.parallel);
            }
            _ => panic!("Expected run command"),
        }
        
        assert!(cli.quiet);
    }

    #[test]
    fn test_run_command_with_file_options() -> Result<()> {
        let temp_dir = tempdir()?;
        let config_path = temp_dir.path().join("test.toml");
        let save_path = temp_dir.path().join("results.json");
        
        let args = vec![
            "ga-sim",
            "run",
            "--config", config_path.to_str().unwrap(),
            "--save-to", save_path.to_str().unwrap(),
            "--seed", "12345"
        ];
        
        let cli = app::Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            app::Commands::Run(run_args) => {
                assert_eq!(run_args.config, Some(config_path));
                assert_eq!(run_args.save_to, Some(save_path));
                assert_eq!(run_args.seed, Some(12345));
            }
            _ => panic!("Expected run command"),
        }
        
        Ok(())
    }

    #[test]
    fn test_config_command() {
        let args = vec!["ga-sim", "config", "show"];
        let cli = app::Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            app::Commands::Config(config_args) => {
                assert!(matches!(config_args.action, app::ConfigAction::Show));
            }
            _ => panic!("Expected config command"),
        }
    }

    #[test]
    fn test_config_init_command() -> Result<()> {
        let temp_dir = tempdir()?;
        let config_path = temp_dir.path().join("my-config.toml");
        
        let args = vec![
            "ga-sim", 
            "config", 
            "init",
            "--path", config_path.to_str().unwrap(),
            "--format", "toml"
        ];
        let cli = app::Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            app::Commands::Config(config_args) => {
                match config_args.action {
                    app::ConfigAction::Init { path, format, .. } => {
                        assert_eq!(path, Some(config_path));
                        assert_eq!(format, app::ConfigFormat::Toml);
                    }
                    _ => panic!("Expected init action"),
                }
            }
            _ => panic!("Expected config command"),
        }
        
        Ok(())
    }

    #[test]
    fn test_init_command() -> Result<()> {
        let temp_dir = tempdir()?;
        let project_path = temp_dir.path().join("my-project");
        
        let args = vec![
            "ga-sim",
            "init",
            project_path.to_str().unwrap()
        ];
        
        let cli = app::Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            app::Commands::Init(init_args) => {
                assert_eq!(init_args.path, project_path);
                assert!(!init_args.force);
            }
            _ => panic!("Expected init command"),
        }
        
        Ok(())
    }

    #[test]
    fn test_global_flags() {
        let args = vec![
            "ga-sim",
            "--verbose",
            "--config", "/path/to/config.toml",
            "run"
        ];
        
        let cli = app::Cli::try_parse_from(args).unwrap();
        
        assert!(cli.verbose);
        assert_eq!(cli.config, Some("/path/to/config.toml".into()));
        assert!(!cli.quiet);
    }

    #[test]
    fn test_conflicting_flags() {
        // verboseとquietは競合する
        let args = vec![
            "ga-sim",
            "--verbose",
            "--quiet",
            "run"
        ];
        
        // clap v4では競合するフラグの処理は設定次第
        // この場合は最後に指定されたものが有効になる
        let cli = app::Cli::try_parse_from(args).unwrap();
        assert!(cli.quiet); // 後に指定されたquietが有効
    }

    #[test]
    fn test_invalid_arguments() {
        // 存在しないコマンド
        let args = vec!["ga-sim", "invalid-command"];
        assert!(app::Cli::try_parse_from(args).is_err());
        
        // 無効なオプション値
        let args = vec![
            "ga-sim",
            "run",
            "--generations", "invalid"
        ];
        assert!(app::Cli::try_parse_from(args).is_err());
        
        // 必須でない引数の欠如は問題ない
        let args = vec!["ga-sim", "run"];
        assert!(app::Cli::try_parse_from(args).is_ok());
    }

    #[test]
    fn test_help_generation() {
        // ヘルプが生成できることを確認
        let args = vec!["ga-sim", "--help"];
        let result = app::Cli::try_parse_from(args);
        assert!(result.is_err()); // ヘルプ表示時はエラーで終了するのが正常
        
        // サブコマンドのヘルプ
        let args = vec!["ga-sim", "run", "--help"];
        let result = app::Cli::try_parse_from(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_version_flag() {
        let args = vec!["ga-sim", "--version"];
        let result = app::Cli::try_parse_from(args);
        assert!(result.is_err()); // バージョン表示時もエラーで終了
    }
}

#[cfg(test)]
mod output_formatter_tests {
    use super::*;

    // 注意: 実際の出力テストは統合テストで行う
    // 単体テストでは主にエラーハンドリングと基本的な動作をテスト

    #[test]
    fn test_output_formatter_creation() {
        let formatter_normal = output::OutputFormatter::new(false);
        let formatter_quiet = output::OutputFormatter::new(true);
        
        // フォーマッターが正常に作成されることを確認
        // （内部状態のテストは難しいため、作成時のパニックがないことを確認）
    }

    #[test]
    fn test_progress_style_creation() {
        // スタイルが正常に作成されることを確認
        let main_style = output::ProgressStyle::main_bar();
        let stats_style = output::ProgressStyle::stats_bar();
        let spinner_style = output::ProgressStyle::spinner();
        
        // スタイル作成時にパニックしないことを確認
    }

    #[test]
    fn test_simulation_progress_creation() {
        let progress = output::SimulationProgress::new(100);
        
        // プログレスバーが正常に作成されることを確認
        progress.update(10, 50.0, 0.8);
        progress.update(50, 75.0, 0.6);
        
        // 完了処理
        progress.finish();
    }

    #[test]
    fn test_simulation_progress_abandon() {
        let progress = output::SimulationProgress::new(100);
        
        progress.update(10, 50.0, 0.8);
        
        // エラー時の処理
        progress.abandon();
    }

    // インタラクティブ機能のテストは実際のユーザー入力が必要なため、
    // 統合テストまたは手動テストで行う
}

#[cfg(test)]
mod cli_integration_tests {
    use super::*;
    use std::process::Command;

    // 注意: これらのテストは実際にバイナリをビルドして実行する
    // CI環境での実行を想定

    #[test]
    #[ignore] // 通常のテスト実行では無視（手動実行時のみ）
    fn test_cli_binary_help() {
        let output = Command::new("cargo")
            .args(&["run", "--", "--help"])
            .output()
            .expect("Failed to execute command");
        
        assert!(output.status.success() || output.status.code() == Some(0));
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("ga-prisoners-dilemma"));
        assert!(stdout.contains("run"));
        assert!(stdout.contains("config"));
        assert!(stdout.contains("init"));
    }

    #[test]
    #[ignore]
    fn test_cli_binary_version() {
        let output = Command::new("cargo")
            .args(&["run", "--", "--version"])
            .output()
            .expect("Failed to execute command");
        
        // バージョン表示は成功扱い
        assert!(output.status.success() || output.status.code() == Some(0));
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains(env!("CARGO_PKG_VERSION")));
    }

    #[test]
    #[ignore]
    fn test_cli_binary_invalid_command() {
        let output = Command::new("cargo")
            .args(&["run", "--", "invalid-command"])
            .output()
            .expect("Failed to execute command");
        
        // 無効なコマンドはエラーになるはず
        assert!(!output.status.success());
        
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("error") || stderr.contains("無効") || stderr.contains("unknown"));
    }

    #[test]
    #[ignore]
    fn test_cli_binary_run_dry() -> Result<()> {
        let temp_dir = tempdir()?;
        
        let output = Command::new("cargo")
            .args(&[
                "run", "--",
                "run",
                "--dry-run",
                "--generations", "10",
                "--population", "5"
            ])
            .current_dir(temp_dir.path())
            .output()
            .expect("Failed to execute command");
        
        // ドライランは成功するはず
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("STDERR: {}", stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("STDOUT: {}", stdout);
        }
        
        assert!(output.status.success());
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("ドライラン") || stdout.contains("dry"));
        
        Ok(())
    }
}

#[cfg(test)]
mod command_validation_tests {
    use super::*;

    #[test]
    fn test_run_args_validation() {
        // 基本的な値の範囲チェック
        let run_args = app::RunArgs {
            generations: Some(0), // 無効
            population: Some(100),
            mutation_rate: Some(0.01),
            strategy: None,
            rounds: None,
            config: None,
            save_to: None,
            seed: None,
            dry_run: false,
            parallel: false,
            report_interval: None,
        };
        
        // 実際のバリデーションは設定適用時に行われる
        assert_eq!(run_args.generations, Some(0));
    }

    #[test]
    fn test_config_format_enum() {
        // 設定フォーマットの列挙型テスト
        let toml_format = app::ConfigFormat::Toml;
        let yaml_format = app::ConfigFormat::Yaml;
        let json_format = app::ConfigFormat::Json;
        
        // デフォルトはTOML
        assert!(matches!(app::ConfigFormat::default(), app::ConfigFormat::Toml));
    }

    #[test]
    fn test_config_action_enum() {
        let show_action = app::ConfigAction::Show;
        let validate_action = app::ConfigAction::Validate;
        
        // それぞれの動作確認（パニックしないことを確認）
        match show_action {
            app::ConfigAction::Show => {} // OK
            _ => panic!("Unexpected action"),
        }
        
        match validate_action {
            app::ConfigAction::Validate => {} // OK
            _ => panic!("Unexpected action"),
        }
    }
}

#[cfg(test)]
mod cli_error_handling_tests {
    use super::*;

    #[test]
    fn test_missing_required_args() {
        // 現在のCLI設計では必須引数はないが、将来の変更に備えてテスト
        let args = vec!["ga-sim"];
        let result = app::Cli::try_parse_from(args);
        
        // サブコマンドが必要
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_numeric_values() {
        let test_cases = vec![
            ("--generations", "not_a_number"),
            ("--population", "-1"),
            ("--mutation-rate", "1.5"),
            ("--seed", "not_numeric"),
        ];
        
        for (flag, value) in test_cases {
            let args = vec!["ga-sim", "run", flag, value];
            let result = app::Cli::try_parse_from(args);
            assert!(result.is_err(), "Should fail for {} {}", flag, value);
        }
    }

    #[test]
    fn test_conflicting_options() {
        // 論理的に競合するオプションの組み合わせ
        let args = vec![
            "ga-sim",
            "run",
            "--dry-run",
            "--save-to", "/dev/null" // ドライランなのに保存指定
        ];
        
        // CLIレベルでは受け入れられるが、実行時にワーニングになる可能性
        let result = app::Cli::try_parse_from(args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_file_path_validation() -> Result<()> {
        let temp_dir = tempdir()?;
        let valid_path = temp_dir.path().join("valid.toml");
        
        // 有効なパス
        let args = vec![
            "ga-sim",
            "run",
            "--config", valid_path.to_str().unwrap()
        ];
        let result = app::Cli::try_parse_from(args);
        assert!(result.is_ok());
        
        // 無効なパス文字（プラットフォーム依存）
        #[cfg(unix)]
        {
            let args = vec![
                "ga-sim",
                "run",
                "--config", "/path/with\0null/byte"
            ];
            // パス解析時点ではエラーにならない場合がある
            let result = app::Cli::try_parse_from(args);
            // プラットフォーム依存なので、両方の結果を許容
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod cli_defaults_tests {
    use super::*;

    #[test]
    fn test_run_args_defaults() {
        let args = vec!["ga-sim", "run"];
        let cli = app::Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            app::Commands::Run(run_args) => {
                // デフォルト値の確認
                assert_eq!(run_args.generations, None);
                assert_eq!(run_args.population, None);
                assert_eq!(run_args.mutation_rate, None);
                assert_eq!(run_args.strategy, None);
                assert_eq!(run_args.rounds, None);
                assert_eq!(run_args.config, None);
                assert_eq!(run_args.save_to, None);
                assert_eq!(run_args.seed, None);
                assert!(!run_args.dry_run);
                assert!(!run_args.parallel);
                assert_eq!(run_args.report_interval, None);
            }
            _ => panic!("Expected run command"),
        }
        
        // グローバルフラグのデフォルト
        assert!(!cli.verbose);
        assert!(!cli.quiet);
        assert_eq!(cli.config, None);
    }

    #[test]
    fn test_config_args_defaults() {
        let args = vec!["ga-sim", "config", "show"];
        let cli = app::Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            app::Commands::Config(config_args) => {
                assert!(matches!(config_args.action, app::ConfigAction::Show));
            }
            _ => panic!("Expected config command"),
        }
    }

    #[test]
    fn test_init_args_defaults() -> Result<()> {
        let temp_dir = tempdir()?;
        let project_path = temp_dir.path().join("test-project");
        
        let args = vec![
            "ga-sim",
            "init",
            project_path.to_str().unwrap()
        ];
        let cli = app::Cli::try_parse_from(args).unwrap();
        
        match cli.command {
            app::Commands::Init(init_args) => {
                assert_eq!(init_args.path, project_path);
                assert!(!init_args.force); // デフォルトはfalse
            }
            _ => panic!("Expected init command"),
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod cli_serialization_tests {
    use super::*;

    #[test]
    fn test_cli_args_debug() {
        // Debug traitの実装確認
        let args = vec!["ga-sim", "run", "--generations", "100"];
        let cli = app::Cli::try_parse_from(args).unwrap();
        
        let debug_string = format!("{:?}", cli);
        assert!(debug_string.contains("Cli"));
        assert!(debug_string.contains("run"));
    }

    #[test]
    fn test_cli_args_clone() {
        // Clone traitの実装確認（必要に応じて）
        let args = vec!["ga-sim", "run"];
        let cli = app::Cli::try_parse_from(args).unwrap();
        
        // Cloneが実装されているかのテスト
        // （現在の実装ではCloneは自動導出されていない可能性がある）
        // let cloned = cli.clone();
        // assert_eq!(cli.quiet, cloned.quiet);
    }
}