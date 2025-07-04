/// 遺伝的アルゴリズム囚人のジレンマシミュレーション
///
/// 遺伝的アルゴリズムで囚人のジレンマゲームの最適戦略を進化させるシミュレーション
///
/// 使用例: `cargo run -- run --generations 1000 --population 50 --mutation-rate 0.02`
use anyhow::{Context, Result};
use clap::Parser;
use ga_prisoners_dilemma::cli::app::Cli;
use ga_prisoners_dilemma::cli::commands::*;
use ga_prisoners_dilemma::core::{init_logging, LogConfig};
use ga_prisoners_dilemma::config::ConfigLoader;
use std::process;
use tokio;

/// アプリケーションのエントリーポイント
#[tokio::main]
async fn main() {
    // 基本的なロギングを設定（エラー処理前）
    let _ = init_logging(&LogConfig::default());
    
    if let Err(e) = run().await {
        eprintln!("❌ エラー: {}", e);
        
        // エラーチェーンを表示
        let mut source = e.source();
        while let Some(err) = source {
            eprintln!("   原因: {}", err);
            source = err.source();
        }
        
        process::exit(1);
    }
}

/// メイン実行ロジック
async fn run() -> Result<()> {
    // コマンドライン引数を解析
    let cli = Cli::parse();

    // ロギングシステムの初期化
    let log_config = if cli.quiet {
        LogConfig::default()
    } else {
        LogConfig::development()
    };
    
    init_logging(&log_config)
        .context("ロギングシステムの初期化に失敗しました")?;

    tracing::info!("ga-prisoners-dilemma v{} を開始します", env!("CARGO_PKG_VERSION"));
    
    // 設定を読み込み
    let config_loader = ConfigLoader::new();
    let config = config_loader.load(cli.config.as_deref())
        .context("設定の読み込みに失敗しました")?;
    
    tracing::debug!("設定が読み込まれました: {:?}", config);

    // コマンドに応じて処理を分岐
    match &cli.command {
        ga_prisoners_dilemma::cli::app::Commands::Run(args) => {
            handle_run_command(args.clone()).await
                .context("runコマンドの実行に失敗しました")?;
        }
        ga_prisoners_dilemma::cli::app::Commands::Config { action } => {
            config::execute_config(action.clone()).await
                .context("configコマンドの実行に失敗しました")?;
        }
        ga_prisoners_dilemma::cli::app::Commands::Init { output, format } => {
            println!("初期化コマンドが実行されました（未実装）: {:?}, {:?}", output, format);
        }
        _ => {
            println!("このコマンドはまだ実装されていません");
        }
    }

    tracing::info!("アプリケーションが正常に終了しました");
    Ok(())
}
