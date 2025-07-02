/// コマンドライン インターフェース（CLI）モジュール
/// 
/// このモジュールでは、遺伝的アルゴリズムシミュレーションのコマンドライン
/// インターフェースを提供します。ユーザーがコマンドライン引数を通じて
/// シミュレーションパラメータを指定できる機能を実装しています。

use crate::core::errors::{GAError, GAResult};
use crate::infrastructure::config::ConfigBuilder;
use std::env;

/// コマンドライン引数を表現する構造体
/// 
/// シミュレーションの設定パラメータをコマンドライン引数から受け取るための
/// 構造体です。全てのフィールドはオプショナルで、指定されない場合は
/// デフォルト値が使用されます。
/// 
/// # フィールド
/// * `generations` - 実行する世代数
/// * `population` - 個体数
/// * `mutation_rate` - 突然変異率
/// * `dna_length` - DNA長
/// * `report_interval` - レポート間隔
/// * `elite_size` - エリートサイズ
/// * `help` - ヘルプ表示フラグ
pub struct CliArgs {
    /// 実行する世代数（--generations）
    pub generations: Option<usize>,
    /// 個体数（--population）
    pub population: Option<usize>,
    /// 突然変異率（--mutation-rate）
    pub mutation_rate: Option<f64>,
    /// DNA長（--dna-length）
    pub dna_length: Option<usize>,
    /// レポート間隔（--report-interval）
    pub report_interval: Option<usize>,
    /// エリートサイズ（--elite-size）
    pub elite_size: Option<usize>,
    /// ヘルプ表示フラグ（--help or -h）
    pub help: bool,
}

impl CliArgs {
    /// コマンドライン引数を解析してCliArgsを作成
    /// 
    /// std::env::args()からコマンドライン引数を取得し、
    /// 各オプションを解析してCliArgsインスタンスを作成します。
    /// 
    /// # 戻り値
    /// 成功時は解析されたCliArgsインスタンス、失敗時はエラー
    /// 
    /// # エラー
    /// * 無効な引数形式の場合
    /// * 数値の解析に失敗した場合
    /// * 必要な値が不足している場合
    pub fn parse() -> GAResult<Self> {
        let args: Vec<String> = env::args().collect();
        let mut cli_args = CliArgs {
            generations: None,
            population: None,
            mutation_rate: None,
            dna_length: None,
            report_interval: None,
            elite_size: None,
            help: false,
        };

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-h" | "--help" => {
                    cli_args.help = true;
                }
                "-g" | "--generations" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(GAError::ValidationError(
                            "Missing value for generations".to_string(),
                        ));
                    }
                    cli_args.generations = Some(args[i].parse().map_err(|_| {
                        GAError::ValidationError("Invalid generations value".to_string())
                    })?);
                }
                "-p" | "--population" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(GAError::ValidationError(
                            "Missing value for population".to_string(),
                        ));
                    }
                    cli_args.population = Some(args[i].parse().map_err(|_| {
                        GAError::ValidationError("Invalid population value".to_string())
                    })?);
                }
                "-m" | "--mutation-rate" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(GAError::ValidationError(
                            "Missing value for mutation rate".to_string(),
                        ));
                    }
                    cli_args.mutation_rate = Some(args[i].parse().map_err(|_| {
                        GAError::ValidationError("Invalid mutation rate value".to_string())
                    })?);
                }
                "-d" | "--dna-length" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(GAError::ValidationError(
                            "Missing value for DNA length".to_string(),
                        ));
                    }
                    cli_args.dna_length = Some(args[i].parse().map_err(|_| {
                        GAError::ValidationError("Invalid DNA length value".to_string())
                    })?);
                }
                "-r" | "--report-interval" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(GAError::ValidationError(
                            "Missing value for report interval".to_string(),
                        ));
                    }
                    cli_args.report_interval = Some(args[i].parse().map_err(|_| {
                        GAError::ValidationError("Invalid report interval value".to_string())
                    })?);
                }
                "-e" | "--elite-size" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(GAError::ValidationError(
                            "Missing value for elite size".to_string(),
                        ));
                    }
                    cli_args.elite_size = Some(args[i].parse().map_err(|_| {
                        GAError::ValidationError("Invalid elite size value".to_string())
                    })?);
                }
                _ => {
                    return Err(GAError::ValidationError(format!(
                        "Unknown argument: {}",
                        args[i]
                    )));
                }
            }
            i += 1;
        }

        Ok(cli_args)
    }

    pub fn to_config_builder(self) -> ConfigBuilder {
        let mut builder = ConfigBuilder::new();

        if let Some(generations) = self.generations {
            builder = builder.generations(generations);
        }
        if let Some(population) = self.population {
            builder = builder.population(population);
        }
        if let Some(mutation_rate) = self.mutation_rate {
            builder = builder.mutation_rate(mutation_rate);
        }
        if let Some(dna_length) = self.dna_length {
            builder = builder.dna_length(dna_length);
        }
        if let Some(report_interval) = self.report_interval {
            builder = builder.report_interval(report_interval);
        }
        if let Some(elite_size) = self.elite_size {
            builder = builder.elite_size(elite_size);
        }

        builder
    }

    pub fn print_help() {
        println!("GA Prisoner's Dilemma - Genetic Algorithm Simulation");
        println!();
        println!("USAGE:");
        println!("    ga_prisoners_dilemma [OPTIONS]");
        println!();
        println!("OPTIONS:");
        println!("    -g, --generations <NUM>      Number of generations to run [default: 50000]");
        println!("    -p, --population <NUM>       Population size [default: 20]");
        println!("    -m, --mutation-rate <RATE>   Mutation rate (0.0-1.0) [default: 0.01]");
        println!("    -d, --dna-length <NUM>       DNA string length [default: 6]");
        println!("    -r, --report-interval <NUM>  Report every N generations [default: 5000]");
        println!("    -e, --elite-size <NUM>       Number of elite individuals [default: 2]");
        println!("    -h, --help                   Print this help message");
        println!();
        println!("EXAMPLES:");
        println!("    ga_prisoners_dilemma");
        println!("    ga_prisoners_dilemma -g 10000 -p 50 -m 0.05");
        println!("    ga_prisoners_dilemma --population 100 --mutation-rate 0.02");
    }
}
