use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// 遺伝的アルゴリズムによる囚人のジレンマシミュレーション
/// 
/// 複数の戦略を用いて囚人のジレンマゲームをシミュレートし、
/// 遺伝的アルゴリズムを通じて最適な戦略を進化させます。
#[derive(Parser, Debug)]
#[command(
    name = "ga-prisoners-dilemma",
    version,
    author,
    about,
    long_about = None,
    arg_required_else_help = true
)]
pub struct Cli {
    /// サブコマンド
    #[command(subcommand)]
    pub command: Commands,
    
    /// 設定ファイルのパス
    #[arg(short, long, global = true, value_name = "FILE")]
    pub config: Option<PathBuf>,
    
    /// ログレベル (trace, debug, info, warn, error)
    #[arg(
        short = 'v',
        long,
        global = true,
        default_value = "info",
        value_name = "LEVEL"
    )]
    pub log_level: String,
    
    /// 出力を抑制する（進捗バーのみ表示）
    #[arg(short, long, global = true)]
    pub quiet: bool,
    
    /// 出力形式
    #[arg(
        short = 'o',
        long,
        global = true,
        value_enum,
        default_value = "human",
        value_name = "FORMAT"
    )]
    pub output: OutputFormat,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// シミュレーションを実行
    Run(RunArgs),
    
    /// 設定を管理
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    
    /// インタラクティブに設定を初期化
    Init {
        /// 出力ファイルパス
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
        
        /// 設定ファイル形式
        #[arg(short, long, value_enum, default_value = "toml")]
        format: ConfigFormat,
    },
    
    /// シミュレーション結果を可視化
    #[cfg(feature = "visualization")]
    Visualize {
        /// 結果ファイルのパス
        #[arg(value_name = "FILE")]
        input: PathBuf,
        
        /// 出力形式
        #[arg(short, long, value_enum, default_value = "png")]
        format: VisualizationFormat,
    },
    
    /// 利用可能な戦略をリスト表示
    Strategies {
        /// 詳細情報を表示
        #[arg(short, long)]
        detailed: bool,
    },
    
    /// ベンチマークを実行
    Benchmark {
        /// ベンチマーク設定
        #[command(flatten)]
        args: BenchmarkArgs,
    },
}

/// シミュレーション実行時の引数
#[derive(Parser, Debug, Clone)]
pub struct RunArgs {
    /// 世代数
    #[arg(short = 'g', long, value_name = "NUM")]
    pub generations: Option<usize>,
    
    /// 個体数
    #[arg(short = 'p', long, value_name = "NUM")]
    pub population: Option<usize>,
    
    /// 突然変異率 (0.0-1.0)
    #[arg(short = 'm', long, value_name = "RATE")]
    pub mutation_rate: Option<f64>,
    
    /// 使用する戦略
    #[arg(short = 's', long, value_name = "NAME")]
    pub strategy: Option<String>,
    
    /// 設定ファイルのパス
    #[arg(short = 'c', long, value_name = "FILE")]
    pub config: Option<PathBuf>,
    
    /// 並列実行を有効化
    #[arg(long)]
    pub parallel: bool,
    
    /// 結果を保存するファイル
    #[arg(long, value_name = "FILE")]
    pub save_to: Option<PathBuf>,
    
    /// レポート間隔（世代数）
    #[arg(short = 'r', long, value_name = "NUM")]
    pub report_interval: Option<usize>,
    
    /// 乱数シード（再現性のため）
    #[arg(long, value_name = "SEED")]
    pub seed: Option<u64>,
    
    /// 1試合あたりのラウンド数
    #[arg(long, value_name = "NUM")]
    pub rounds: Option<usize>,
    
    /// ドライラン（実際には実行しない）
    #[arg(long)]
    pub dry_run: bool,
}

/// 設定管理のサブコマンド
#[derive(Subcommand, Debug, Clone)]
pub enum ConfigAction {
    /// 現在の設定を表示
    Show {
        /// 設定キーのパス（例: genetic.population_size）
        #[arg(value_name = "KEY")]
        key: Option<String>,
    },
    
    /// 設定値を変更
    Set {
        /// 設定キー
        #[arg(value_name = "KEY")]
        key: String,
        
        /// 新しい値
        #[arg(value_name = "VALUE")]
        value: String,
    },
    
    /// 設定をリセット
    Reset {
        /// 確認をスキップ
        #[arg(short = 'y', long)]
        yes: bool,
    },
    
    /// 設定を検証
    Validate,
    
    /// 設定ファイルの場所を表示
    Path,
    
    /// 新しい設定ファイルを初期化
    Init {
        /// 出力パス
        #[arg(short, long, value_name = "PATH")]
        path: Option<std::path::PathBuf>,
        
        /// ファイル形式
        #[arg(short, long, default_value = "toml")]
        format: ConfigFormat,
        
        /// 既存ファイルを上書き
        #[arg(short = 'f', long)]
        force: bool,
    },
}

/// ベンチマーク実行時の引数
#[derive(Parser, Debug, Clone)]
pub struct BenchmarkArgs {
    /// ベンチマークする戦略（複数指定可）
    #[arg(short, long, value_name = "STRATEGY")]
    pub strategies: Vec<String>,
    
    /// 各戦略の実行回数
    #[arg(short, long, default_value = "5", value_name = "NUM")]
    pub iterations: usize,
    
    /// ベンチマーク用の世代数
    #[arg(short, long, default_value = "1000", value_name = "NUM")]
    pub generations: usize,
    
    /// 結果をCSVで出力
    #[arg(long)]
    pub csv: bool,
}

/// 出力形式
#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFormat {
    /// 人間が読みやすい形式
    Human,
    /// JSON形式
    Json,
    /// CSV形式
    Csv,
    /// 最小限の出力
    Minimal,
}

/// 設定ファイル形式
#[derive(Clone, Debug, ValueEnum)]
pub enum ConfigFormat {
    /// TOML形式
    Toml,
    /// YAML形式
    Yaml,
    /// JSON形式
    Json,
}

/// 可視化形式
#[cfg(feature = "visualization")]
#[derive(Clone, Debug, ValueEnum)]
pub enum VisualizationFormat {
    /// PNG画像
    Png,
    /// SVG画像
    Svg,
    /// インタラクティブHTML
    Html,
}