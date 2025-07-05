/// 実行コマンドの実装
use anyhow::{Context, Result};
use crate::cli::app::RunArgs;
use crate::config::{Config, ConfigLoader};
use crate::simulation::Simulation;
use std::path::Path;

/// 実行コマンドを処理
pub async fn handle_run_command(args: RunArgs) -> Result<()> {
    // 設定の読み込み
    let mut config = if let Some(config_path) = &args.config {
        Config::from_file(config_path)
            .context("設定ファイルの読み込みに失敗しました")?
    } else {
        let loader = ConfigLoader::new();
        loader.load(None)
            .context("設定の読み込みに失敗しました")?
    };

    // コマンドライン引数で設定を上書き
    apply_overrides(&mut config, &args)?;

    // 設定の検証
    config.validate()
        .context("設定の検証に失敗しました")?;

    if args.dry_run {
        println!("ドライラン: 設定内容を表示します");
        let toml_string = toml::to_string_pretty(&config)
            .context("設定のTOML変換に失敗しました")?;
        println!("{}", toml_string);
        return Ok(());
    }

    // シミュレーションの実行
    tracing::info!("シミュレーションを開始します");
    
    let mut simulation = Simulation::new(config, args.seed.map(|s| s as u32))
        .context("シミュレーションの初期化に失敗しました")?;

    if args.parallel {
        simulation.enable_parallel();
    }

    let result = simulation.run().await
        .context("シミュレーションの実行に失敗しました")?;

    // 結果の出力
    println!("\n{}", result.summary());

    // 結果の保存
    if let Some(save_path) = &args.save_to {
        save_results(&result, save_path)
            .context("結果の保存に失敗しました")?;
        println!("結果を保存しました: {}", save_path.display());
    }

    tracing::info!("シミュレーションが完了しました");
    Ok(())
}

/// コマンドライン引数で設定を上書き
fn apply_overrides(config: &mut Config, args: &RunArgs) -> Result<()> {
    if let Some(generations) = args.generations {
        config.genetic.generations = generations;
    }
    
    if let Some(population) = args.population {
        config.genetic.population_size = population;
    }
    
    if let Some(mutation_rate) = args.mutation_rate {
        config.genetic.mutation_rate = mutation_rate;
    }
    
    if let Some(strategy) = &args.strategy {
        config.simulation.default_strategy = strategy.clone();
    }
    
    if let Some(rounds) = args.rounds {
        config.simulation.rounds_per_match = rounds;
    }
    
    if let Some(interval) = args.report_interval {
        config.output.report_interval = interval;
    }

    Ok(())
}

/// 結果をファイルに保存
fn save_results(result: &crate::simulation::SimulationStats, path: &Path) -> Result<()> {
    let json_string = serde_json::to_string_pretty(result)
        .context("結果のJSON変換に失敗しました")?;
    
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .context("出力ディレクトリの作成に失敗しました")?;
    }
    
    std::fs::write(path, json_string)
        .context("結果ファイルの書き込みに失敗しました")?;
    
    Ok(())
}