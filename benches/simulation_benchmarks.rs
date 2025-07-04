/// シミュレーションのベンチマーク
///
/// パフォーマンス測定と最適化の指標
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use ga_prisoners_dilemma::*;
use std::collections::HashMap;
use tokio::runtime::Runtime;

/// 基本的なシミュレーション実行のベンチマーク
fn bench_basic_simulation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    let config = config::Config {
        simulation: config::SimulationConfig {
            default_strategy: "tit-for-tat".to_string(),
            rounds_per_match: 10,
            payoff_matrix: simulation::PayoffMatrix::standard(),
            tournament_type: config::TournamentType::RoundRobin,
        },
        genetic: config::GeneticConfig {
            population_size: 50,
            generations: 20,
            mutation_rate: 0.01,
            elite_count: 5,
            dna_length: 8,
            crossover_type: config::CrossoverType::SinglePoint,
            selection_method: config::SelectionMethod::Tournament(3),
        },
        output: config::OutputConfig::default(),
        performance: config::PerformanceConfig::default(),
        strategies: HashMap::new(),
    };

    c.bench_function("basic_simulation", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut simulation = simulation::Simulation::new(
                    black_box(config.clone()), 
                    Some(42)
                ).unwrap();
                let result = simulation.run().await.unwrap();
                black_box(result)
            })
        })
    });
}

/// 個体数による性能スケーリングのベンチマーク
fn bench_population_scaling(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("population_scaling");
    
    for population_size in [20, 50, 100, 200].iter() {
        let config = config::Config {
            simulation: config::SimulationConfig {
                default_strategy: "tit-for-tat".to_string(),
                rounds_per_match: 5,
                payoff_matrix: simulation::PayoffMatrix::standard(),
                tournament_type: config::TournamentType::RoundRobin,
            },
            genetic: config::GeneticConfig {
                population_size: *population_size,
                generations: 10,
                mutation_rate: 0.01,
                elite_count: (*population_size / 10).max(1),
                dna_length: 8,
                crossover_type: config::CrossoverType::SinglePoint,
                selection_method: config::SelectionMethod::Tournament(2),
            },
            output: config::OutputConfig::default(),
            performance: config::PerformanceConfig::default(),
            strategies: HashMap::new(),
        };

        group.bench_with_input(
            BenchmarkId::new("population", population_size),
            population_size,
            |b, &_size| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut simulation = simulation::Simulation::new(
                            black_box(config.clone()),
                            Some(42)
                        ).unwrap();
                        let result = simulation.run().await.unwrap();
                        black_box(result)
                    })
                })
            },
        );
    }
    group.finish();
}

/// 世代数による性能スケーリングのベンチマーク
fn bench_generation_scaling(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("generation_scaling");
    
    for generations in [10, 25, 50, 100].iter() {
        let config = config::Config {
            simulation: config::SimulationConfig {
                default_strategy: "tit-for-tat".to_string(),
                rounds_per_match: 5,
                payoff_matrix: simulation::PayoffMatrix::standard(),
                tournament_type: config::TournamentType::RoundRobin,
            },
            genetic: config::GeneticConfig {
                population_size: 30,
                generations: *generations,
                mutation_rate: 0.01,
                elite_count: 3,
                dna_length: 8,
                crossover_type: config::CrossoverType::SinglePoint,
                selection_method: config::SelectionMethod::Tournament(2),
            },
            output: config::OutputConfig::default(),
            performance: config::PerformanceConfig::default(),
            strategies: HashMap::new(),
        };

        group.bench_with_input(
            BenchmarkId::new("generations", generations),
            generations,
            |b, &_gens| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut simulation = simulation::Simulation::new(
                            black_box(config.clone()),
                            Some(42)
                        ).unwrap();
                        let result = simulation.run().await.unwrap();
                        black_box(result)
                    })
                })
            },
        );
    }
    group.finish();
}

/// DNA長による性能への影響のベンチマーク
fn bench_dna_length_impact(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("dna_length_impact");
    
    for dna_length in [4, 8, 16, 32].iter() {
        let config = config::Config {
            simulation: config::SimulationConfig {
                default_strategy: "tit-for-tat".to_string(),
                rounds_per_match: 5,
                payoff_matrix: simulation::PayoffMatrix::standard(),
                tournament_type: config::TournamentType::RoundRobin,
            },
            genetic: config::GeneticConfig {
                population_size: 30,
                generations: 15,
                mutation_rate: 0.01,
                elite_count: 3,
                dna_length: *dna_length,
                crossover_type: config::CrossoverType::SinglePoint,
                selection_method: config::SelectionMethod::Tournament(2),
            },
            output: config::OutputConfig::default(),
            performance: config::PerformanceConfig::default(),
            strategies: HashMap::new(),
        };

        group.bench_with_input(
            BenchmarkId::new("dna_length", dna_length),
            dna_length,
            |b, &_length| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut simulation = simulation::Simulation::new(
                            black_box(config.clone()),
                            Some(42)
                        ).unwrap();
                        let result = simulation.run().await.unwrap();
                        black_box(result)
                    })
                })
            },
        );
    }
    group.finish();
}

/// 異なる戦略の性能比較ベンチマーク
fn bench_strategy_comparison(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("strategy_comparison");
    
    let strategies = ["always-cooperate", "always-defect", "tit-for-tat", "random"];
    
    for strategy in strategies.iter() {
        let config = config::Config {
            simulation: config::SimulationConfig {
                default_strategy: strategy.to_string(),
                rounds_per_match: 8,
                payoff_matrix: simulation::PayoffMatrix::standard(),
                tournament_type: config::TournamentType::RoundRobin,
            },
            genetic: config::GeneticConfig {
                population_size: 40,
                generations: 15,
                mutation_rate: 0.01,
                elite_count: 4,
                dna_length: 8,
                crossover_type: config::CrossoverType::SinglePoint,
                selection_method: config::SelectionMethod::Tournament(2),
            },
            output: config::OutputConfig::default(),
            performance: config::PerformanceConfig::default(),
            strategies: HashMap::new(),
        };

        group.bench_with_input(
            BenchmarkId::new("strategy", strategy),
            strategy,
            |b, &_strat| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut simulation = simulation::Simulation::new(
                            black_box(config.clone()),
                            Some(42)
                        ).unwrap();
                        let result = simulation.run().await.unwrap();
                        black_box(result)
                    })
                })
            },
        );
    }
    group.finish();
}

/// 交叉方式の性能比較ベンチマーク
fn bench_crossover_methods(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("crossover_methods");
    
    let crossover_types = [
        ("single_point", config::CrossoverType::SinglePoint),
        ("two_point", config::CrossoverType::TwoPoint),
        ("uniform", config::CrossoverType::Uniform(0.5)),
    ];
    
    for (name, crossover_type) in crossover_types.iter() {
        let config = config::Config {
            simulation: config::SimulationConfig {
                default_strategy: "tit-for-tat".to_string(),
                rounds_per_match: 5,
                payoff_matrix: simulation::PayoffMatrix::standard(),
                tournament_type: config::TournamentType::RoundRobin,
            },
            genetic: config::GeneticConfig {
                population_size: 30,
                generations: 15,
                mutation_rate: 0.01,
                elite_count: 3,
                dna_length: 12,
                crossover_type: crossover_type.clone(),
                selection_method: config::SelectionMethod::Tournament(2),
            },
            output: config::OutputConfig::default(),
            performance: config::PerformanceConfig::default(),
            strategies: HashMap::new(),
        };

        group.bench_with_input(
            BenchmarkId::new("crossover", name),
            name,
            |b, &_method| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut simulation = simulation::Simulation::new(
                            black_box(config.clone()),
                            Some(42)
                        ).unwrap();
                        let result = simulation.run().await.unwrap();
                        black_box(result)
                    })
                })
            },
        );
    }
    group.finish();
}

/// 選択方式の性能比較ベンチマーク
fn bench_selection_methods(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("selection_methods");
    
    let selection_methods = [
        ("roulette", config::SelectionMethod::Roulette),
        ("tournament_2", config::SelectionMethod::Tournament(2)),
        ("tournament_3", config::SelectionMethod::Tournament(3)),
        ("rank", config::SelectionMethod::Rank),
        ("elite", config::SelectionMethod::Elite),
    ];
    
    for (name, selection_method) in selection_methods.iter() {
        let config = config::Config {
            simulation: config::SimulationConfig {
                default_strategy: "tit-for-tat".to_string(),
                rounds_per_match: 5,
                payoff_matrix: simulation::PayoffMatrix::standard(),
                tournament_type: config::TournamentType::RoundRobin,
            },
            genetic: config::GeneticConfig {
                population_size: 40,
                generations: 15,
                mutation_rate: 0.01,
                elite_count: 4,
                dna_length: 8,
                crossover_type: config::CrossoverType::SinglePoint,
                selection_method: selection_method.clone(),
            },
            output: config::OutputConfig::default(),
            performance: config::PerformanceConfig::default(),
            strategies: HashMap::new(),
        };

        group.bench_with_input(
            BenchmarkId::new("selection", name),
            name,
            |b, &_method| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut simulation = simulation::Simulation::new(
                            black_box(config.clone()),
                            Some(42)
                        ).unwrap();
                        let result = simulation.run().await.unwrap();
                        black_box(result)
                    })
                })
            },
        );
    }
    group.finish();
}

/// 乱数生成器の性能ベンチマーク
fn bench_random_generation(c: &mut Criterion) {
    use ga_prisoners_dilemma::core::random::*;
    
    let mut group = c.benchmark_group("random_generation");
    
    // SFMT乱数生成器の性能測定
    group.bench_function("sfmt_f64", |b| {
        let rng = RandomGenerator::new(Some(42));
        b.iter(|| {
            for _ in 0..1000 {
                black_box(rng.gen_f64().unwrap());
            }
        })
    });
    
    group.bench_function("sfmt_range", |b| {
        let rng = RandomGenerator::new(Some(42));
        b.iter(|| {
            for _ in 0..1000 {
                black_box(rng.gen_range(100).unwrap());
            }
        })
    });
    
    group.bench_function("sfmt_bool", |b| {
        let rng = RandomGenerator::new(Some(42));
        b.iter(|| {
            for _ in 0..1000 {
                black_box(rng.gen_bool(0.5).unwrap());
            }
        })
    });
    
    group.bench_function("sfmt_normal", |b| {
        let rng = RandomGenerator::new(Some(42));
        b.iter(|| {
            for _ in 0..1000 {
                black_box(rng.gen_normal(0.0, 1.0).unwrap());
            }
        })
    });
    
    group.finish();
}

/// 設定の読み込み・解析性能ベンチマーク
fn bench_config_operations(c: &mut Criterion) {
    use tempfile::NamedTempFile;
    
    let mut group = c.benchmark_group("config_operations");
    
    let config = config::Config {
        simulation: config::SimulationConfig {
            default_strategy: "tit-for-tat".to_string(),
            rounds_per_match: 10,
            payoff_matrix: simulation::PayoffMatrix::standard(),
            tournament_type: config::TournamentType::RoundRobin,
        },
        genetic: config::GeneticConfig {
            population_size: 100,
            generations: 1000,
            mutation_rate: 0.01,
            elite_count: 10,
            dna_length: 16,
            crossover_type: config::CrossoverType::SinglePoint,
            selection_method: config::SelectionMethod::Tournament(3),
        },
        output: config::OutputConfig::default(),
        performance: config::PerformanceConfig::default(),
        strategies: HashMap::new(),
    };
    
    // TOML形式の性能測定
    group.bench_function("config_toml_save", |b| {
        b.iter(|| {
            let temp_file = NamedTempFile::with_suffix(".toml").unwrap();
            config::ConfigLoader::save(black_box(&config), temp_file.path()).unwrap();
            black_box(temp_file)
        })
    });
    
    group.bench_function("config_toml_load", |b| {
        let temp_file = NamedTempFile::with_suffix(".toml").unwrap();
        config::ConfigLoader::save(&config, temp_file.path()).unwrap();
        
        b.iter(|| {
            let loaded = config::Config::from_file(black_box(temp_file.path())).unwrap();
            black_box(loaded)
        })
    });
    
    // JSON形式の性能測定
    group.bench_function("config_json_save", |b| {
        b.iter(|| {
            let temp_file = NamedTempFile::with_suffix(".json").unwrap();
            config::ConfigLoader::save(black_box(&config), temp_file.path()).unwrap();
            black_box(temp_file)
        })
    });
    
    group.bench_function("config_json_load", |b| {
        let temp_file = NamedTempFile::with_suffix(".json").unwrap();
        config::ConfigLoader::save(&config, temp_file.path()).unwrap();
        
        b.iter(|| {
            let loaded = config::Config::from_file(black_box(temp_file.path())).unwrap();
            black_box(loaded)
        })
    });
    
    // バリデーション性能
    group.bench_function("config_validation", |b| {
        b.iter(|| {
            black_box(config.validate().unwrap())
        })
    });
    
    group.finish();
}

/// 統計計算の性能ベンチマーク
fn bench_statistics_calculation(c: &mut Criterion) {
    use ga_prisoners_dilemma::core::traits::Statistics;
    
    let mut group = c.benchmark_group("statistics_calculation");
    
    // 大きなデータセットでの統計計算
    let large_dataset: Vec<u64> = (1..=10000).collect();
    
    group.bench_function("stats_mean_large", |b| {
        b.iter(|| {
            black_box(large_dataset.mean())
        })
    });
    
    group.bench_function("stats_std_dev_large", |b| {
        b.iter(|| {
            black_box(large_dataset.std_deviation())
        })
    });
    
    group.bench_function("stats_max_min_large", |b| {
        b.iter(|| {
            let max = large_dataset.max();
            let min = large_dataset.min();
            black_box((max, min))
        })
    });
    
    // 小さなデータセットでの統計計算
    let small_dataset: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    group.bench_function("stats_all_small", |b| {
        b.iter(|| {
            let mean = small_dataset.mean();
            let std_dev = small_dataset.std_deviation();
            let max = small_dataset.max();
            let min = small_dataset.min();
            black_box((mean, std_dev, max, min))
        })
    });
    
    group.finish();
}

/// 並列実行のベンチマーク（フィーチャーが有効な場合）
#[cfg(feature = "parallel")]
fn bench_parallel_execution(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("parallel_execution");
    
    let base_config = config::Config {
        simulation: config::SimulationConfig {
            default_strategy: "tit-for-tat".to_string(),
            rounds_per_match: 5,
            payoff_matrix: simulation::PayoffMatrix::standard(),
            tournament_type: config::TournamentType::RoundRobin,
        },
        genetic: config::GeneticConfig {
            population_size: 100,
            generations: 10,
            mutation_rate: 0.01,
            elite_count: 10,
            dna_length: 8,
            crossover_type: config::CrossoverType::SinglePoint,
            selection_method: config::SelectionMethod::Tournament(2),
        },
        output: config::OutputConfig::default(),
        performance: config::PerformanceConfig::default(),
        strategies: HashMap::new(),
    };
    
    // シーケンシャル実行
    group.bench_function("sequential", |b| {
        let config = config::Config {
            performance: config::PerformanceConfig {
                parallel: false,
                ..base_config.performance.clone()
            },
            ..base_config.clone()
        };
        
        b.iter(|| {
            rt.block_on(async {
                let mut simulation = simulation::Simulation::new(
                    black_box(config.clone()),
                    Some(42)
                ).unwrap();
                let result = simulation.run().await.unwrap();
                black_box(result)
            })
        })
    });
    
    // 並列実行
    group.bench_function("parallel", |b| {
        let config = config::Config {
            performance: config::PerformanceConfig {
                parallel: true,
                num_threads: 4,
                ..base_config.performance.clone()
            },
            ..base_config.clone()
        };
        
        b.iter(|| {
            rt.block_on(async {
                let mut simulation = simulation::Simulation::new(
                    black_box(config.clone()),
                    Some(42)
                ).unwrap();
                simulation.enable_parallel();
                let result = simulation.run().await.unwrap();
                black_box(result)
            })
        })
    });
    
    group.finish();
}

// ベンチマークグループの定義
criterion_group!(
    benches,
    bench_basic_simulation,
    bench_population_scaling,
    bench_generation_scaling,
    bench_dna_length_impact,
    bench_strategy_comparison,
    bench_crossover_methods,
    bench_selection_methods,
    bench_random_generation,
    bench_config_operations,
    bench_statistics_calculation,
);

// 並列実行ベンチマークはフィーチャーが有効な場合のみ追加
#[cfg(feature = "parallel")]
criterion_group!(
    parallel_benches,
    bench_parallel_execution,
);

// メインベンチマーク実行
#[cfg(not(feature = "parallel"))]
criterion_main!(benches);

#[cfg(feature = "parallel")]
criterion_main!(benches, parallel_benches);