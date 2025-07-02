/// 遺伝的アルゴリズム囚人のジレンマシミュレーション
///
/// 遺伝的アルゴリズムで囚人のジレンマゲームの最適戦略を進化させるシミュレーション
///
/// 使用例: `cargo run -- --generations 1000 --population 50 --mutation-rate 0.02`
use ga_prisoners_dilemma::core::errors::GAResult;
use ga_prisoners_dilemma::domain::simulation::Simulation;
use ga_prisoners_dilemma::interface::cli::CliArgs;
use std::process;

/// アプリケーションのエントリーポイント
fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

/// メイン実行ロジック
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
