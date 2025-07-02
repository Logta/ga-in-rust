/// 遺伝的アルゴリズム囚人のジレンマシミュレーション
/// 
/// このプログラムは、遺伝的アルゴリズムを使用して囚人のジレンマゲームの
/// 最適戦略を進化させるシミュレーションを実行します。
/// 
/// ## 使用方法
/// 
/// ```bash
/// cargo run --bin ga-sim -- --generations 1000 --population 50 --mutation-rate 0.02
/// ```
/// 
/// ## パラメータ
/// 
/// * `--generations`: 実行する世代数
/// * `--population`: 個体数
/// * `--mutation-rate`: 突然変異率（0.0-1.0）
/// * `--dna-length`: DNA（戦略）の長さ
/// * `--elite-size`: エリート保存個体数
/// * `--report-interval`: 進捗報告間隔
/// * `--help`, `-h`: ヘルプを表示

use ga_prisoners_dilemma::core::errors::GAResult;
use ga_prisoners_dilemma::domain::simulation::Simulation;
use ga_prisoners_dilemma::interface::cli::CliArgs;
use std::process;

/// アプリケーションのエントリーポイント
/// 
/// コマンドライン引数を解析し、シミュレーションを実行します。
/// エラーが発生した場合は、エラーメッセージを表示して異常終了します。
fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

/// メインの実行ロジック
/// 
/// 1. コマンドライン引数を解析
/// 2. ヘルプが要求された場合は表示して終了
/// 3. 設定を構築
/// 4. シミュレーションを作成・実行
/// 
/// # 戻り値
/// 成功時はOk(())、エラー時はエラー詳細
/// 
/// # エラー
/// * コマンドライン引数の解析に失敗した場合
/// * 設定の構築に失敗した場合
/// * シミュレーションの作成・実行に失敗した場合
fn run() -> GAResult<()> {
    // コマンドライン引数を解析
    let args = CliArgs::parse()?;

    // ヘルプが要求された場合は表示して終了
    if args.help {
        CliArgs::print_help();
        return Ok(());
    }

    // 設定を構築
    let config = args.to_config_builder().build()?;
    
    // シミュレーションを作成・実行
    let simulation = Simulation::new(config)?;
    let _result = simulation.run()?;

    Ok(())
}
