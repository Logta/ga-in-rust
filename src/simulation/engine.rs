/// メインシミュレーションエンジン
///
/// 遺伝的アルゴリズムと囚人のジレンマゲームを統合したシミュレーション実行エンジン
use anyhow::{Context, Result};
use std::time::Instant;
use crate::config::Config;
use crate::core::random::{RandomGenerator, init_default_rng};
use crate::simulation::environment::Environment;
use crate::simulation::stats::{SimulationStats, GenerationStats, BestIndividualInfo, ConvergenceInfo};
use crate::genetic::{Individual, GeneticAlgorithm};

/// シミュレーション実行エンジン
pub struct Simulation {
    /// 設定
    config: Config,
    
    /// ゲーム環境
    environment: Environment,
    
    /// 遺伝的アルゴリズム
    genetic_algorithm: GeneticAlgorithm,
    
    /// 乱数生成器
    rng: RandomGenerator,
    
    /// 現在の世代
    current_generation: usize,
    
    /// 統計履歴
    generation_history: Vec<GenerationStats>,
    
    /// 最良個体の情報
    best_individual: Option<BestIndividualInfo>,
    
    /// 進捗コールバック
    progress_callback: Option<Box<dyn Fn(usize, &GenerationStats) + Send + Sync>>,
    
    /// 並列実行フラグ
    parallel_enabled: bool,
}

impl Simulation {
    /// 新しいシミュレーションを作成
    pub fn new(config: Config, seed: Option<u32>) -> Result<Self> {
        // 設定の妥当性を検証
        config.validate()
            .context("設定の検証に失敗しました")?;

        // 乱数生成器の初期化
        let rng = RandomGenerator::new(seed);
        if let Some(seed) = seed {
            init_default_rng(seed);
        }

        // 環境の作成
        let environment = Environment::new(
            config.simulation.payoff_matrix.clone(),
            config.simulation.rounds_per_match,
            0.0, // ノイズレベルは設定から取得するように後で拡張
            1.0, // 情報完全性も同様
        ).context("ゲーム環境の作成に失敗しました")?;

        // 遺伝的アルゴリズムの初期化（個体群も内部で作成される）
        let genetic_algorithm = GeneticAlgorithm::new(
            config.genetic.clone(),
        ).context("遺伝的アルゴリズムの初期化に失敗しました")?;

        Ok(Self {
            config,
            environment,
            genetic_algorithm,
            rng,
            current_generation: 0,
            generation_history: Vec::new(),
            best_individual: None,
            progress_callback: None,
            parallel_enabled: false,
        })
    }

    /// 進捗コールバックを設定
    pub fn set_progress_callback(
        &mut self, 
        callback: Box<dyn Fn(usize, &GenerationStats) + Send + Sync>
    ) {
        self.progress_callback = Some(callback);
    }

    /// 並列実行を有効化
    pub fn enable_parallel(&mut self) {
        self.parallel_enabled = true;
    }

    /// シミュレーションを実行
    pub async fn run(&mut self) -> Result<SimulationStats> {
        let simulation_start = Instant::now();
        
        tracing::info!(
            "シミュレーション開始: {} 世代, {} 個体",
            self.config.genetic.generations,
            self.config.genetic.population_size
        );

        // 初期個体群の評価
        self.evaluate_population()
            .context("初期個体群の評価に失敗しました")?;

        // 世代ループ
        for generation in 0..self.config.genetic.generations {
            let generation_start = Instant::now();
            self.current_generation = generation;

            // 個体群の評価
            self.evaluate_population()
                .context("個体群の評価に失敗しました")?;

            // 統計の計算
            let generation_stats = self.calculate_generation_stats(generation, generation_start.elapsed().as_millis() as u64)
                .context("世代統計の計算に失敗しました")?;

            // 最良個体の更新
            self.update_best_individual(generation, &generation_stats)
                .context("最良個体の更新に失敗しました")?;

            // 統計の記録
            self.generation_history.push(generation_stats.clone());

            // 進捗コールバックの呼び出し
            if let Some(ref callback) = self.progress_callback {
                callback(generation, &generation_stats);
            }

            // レポート出力
            if generation % self.config.output.report_interval == 0 {
                self.log_generation_report(generation, &generation_stats);
            }

            // 収束チェック
            if self.check_convergence()? {
                tracing::info!("第{}世代で収束しました", generation);
                break;
            }

            // 次世代の生成（最終世代以外）
            if generation < self.config.genetic.generations - 1 {
                self.evolve_population()
                    .context("個体群の進化に失敗しました")?;
            }
        }

        let total_elapsed = simulation_start.elapsed();
        
        // 最終統計の構築
        let simulation_stats = self.build_final_stats(total_elapsed)
            .context("最終統計の構築に失敗しました")?;

        tracing::info!(
            "シミュレーション完了: {:.2}秒, 最高適応度: {:.2}",
            total_elapsed.as_secs_f64(),
            simulation_stats.best_individual.best_fitness
        );

        Ok(simulation_stats)
    }

    /// 個体群の評価
    fn evaluate_population(&mut self) -> Result<()> {
        let population_size = self.genetic_algorithm.population().size();
        
        if self.parallel_enabled {
            #[cfg(feature = "parallel")]
            {
                use rayon::prelude::*;
                
                // 並列処理は一旦無効化（複雑になるため）
                for i in 0..population_size {
                    let individual = self.genetic_algorithm.population().get_individual(i)
                        .context("個体の取得に失敗しました")?;
                    
                    let fitness = self.calculate_individual_fitness(individual)
                        .context("個体の適応度計算に失敗しました")?;
                    
                    self.genetic_algorithm.population_mut().set_individual_fitness(i, fitness)
                        .context("個体の適応度設定に失敗しました")?;
                }
            }
        } else {
            // シーケンシャル実行
            for i in 0..population_size {
                let individual = self.genetic_algorithm.population().get_individual(i)
                    .context("個体の取得に失敗しました")?;
                
                let fitness = self.calculate_individual_fitness(individual)
                    .context("個体の適応度計算に失敗しました")?;
                
                self.genetic_algorithm.population_mut().set_individual_fitness(i, fitness)
                    .context("個体の適応度設定に失敗しました")?;
            }
        }

        Ok(())
    }

    /// 個体の適応度を計算
    fn calculate_individual_fitness(&self, individual: &Individual) -> Result<f64> {
        let mut total_score = 0.0;
        let population_size = self.genetic_algorithm.population().size();
        
        // 他の全個体と対戦
        for i in 0..population_size {
            let opponent = self.genetic_algorithm.population().get_individual(i)
                .context("対戦相手の取得に失敗しました")?;
            
            if individual.id() != opponent.id() {
                let score = self.play_match(individual, opponent)
                    .context("試合の実行に失敗しました")?;
                total_score += score;
            }
        }

        // 平均スコアを返す
        Ok(total_score / (population_size - 1) as f64)
    }

    /// 2個体間の試合を実行
    fn play_match(&self, player1: &Individual, player2: &Individual) -> Result<f64> {
        let mut total_score = 0;
        let mut history1 = Vec::new();
        let mut history2 = Vec::new();

        for round in 0..self.environment.rounds_per_match {
            // 各個体の選択を決定
            let choice1 = player1.choose(&history2, round)
                .context("プレイヤー1の選択決定に失敗しました")?;
            let choice2 = player2.choose(&history1, round)
                .context("プレイヤー2の選択決定に失敗しました")?;

            // ペイオフの計算
            let (score1, _score2) = self.environment.payoff_matrix.payoff(choice1, choice2);
            total_score += score1;

            // 履歴の更新
            history1.push(choice1);
            history2.push(choice2);
        }

        Ok(total_score as f64)
    }

    /// 個体群を進化
    fn evolve_population(&mut self) -> Result<()> {
        self.genetic_algorithm.evolve()
            .context("遺伝的アルゴリズムの実行に失敗しました")?;
        Ok(())
    }

    /// 世代統計を計算
    fn calculate_generation_stats(&self, generation: usize, elapsed_ms: u64) -> Result<GenerationStats> {
        let fitness_values: Vec<f64> = self.genetic_algorithm.population().individuals()
            .iter()
            .map(|individual| individual.fitness())
            .collect();

        let diversity = self.calculate_diversity()?;
        let elite_count = self.config.genetic.elite_count;

        GenerationStats::new(generation, &fitness_values, diversity, elite_count, elapsed_ms)
            .context("世代統計の作成に失敗しました")
    }

    /// 遺伝的多様性を計算
    fn calculate_diversity(&self) -> Result<f64> {
        let individuals = self.genetic_algorithm.population().individuals();
        let total_pairs = individuals.len() * (individuals.len() - 1) / 2;
        
        if total_pairs == 0 {
            return Ok(0.0);
        }

        let mut total_distance = 0;
        for i in 0..individuals.len() {
            for j in (i + 1)..individuals.len() {
                total_distance += individuals[i].dna_distance(&individuals[j])?;
            }
        }

        let max_distance = self.config.genetic.dna_length;
        let normalized_diversity = total_distance as f64 / (total_pairs * max_distance) as f64;
        
        Ok(normalized_diversity)
    }

    /// 最良個体の情報を更新
    fn update_best_individual(&mut self, generation: usize, stats: &GenerationStats) -> Result<()> {
        let should_update = match &self.best_individual {
            None => true,
            Some(best) => stats.max_fitness > best.best_fitness,
        };

        if should_update {
            let best_agent = self.genetic_algorithm.population().best_individual()
                .context("最良個体の取得に失敗しました")?;
            
            self.best_individual = Some(BestIndividualInfo {
                best_fitness: stats.max_fitness,
                best_generation: generation,
                best_dna: best_agent.dna().to_string(),
                description: format!("世代{}の最良個体", generation),
            });
        }

        Ok(())
    }

    /// 収束チェック
    fn check_convergence(&self) -> Result<bool> {
        if self.generation_history.len() < 10 {
            return Ok(false);
        }

        // 過去10世代の適応度の変化をチェック
        let recent_stats = &self.generation_history[self.generation_history.len() - 10..];
        let fitness_values: Vec<f64> = recent_stats.iter().map(|s| s.avg_fitness).collect();
        
        let min_fitness = fitness_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_fitness = fitness_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        
        // 適応度の変化が1%未満なら収束とみなす
        let convergence_threshold = 0.01;
        let fitness_change = if max_fitness > 0.0 {
            (max_fitness - min_fitness) / max_fitness
        } else {
            0.0
        };

        Ok(fitness_change < convergence_threshold)
    }

    /// 世代レポートをログ出力
    fn log_generation_report(&self, generation: usize, stats: &GenerationStats) {
        tracing::info!(
            "第{}世代: 平均適応度={:.2}, 最大適応度={:.2}, 多様性={:.2}%, 時間={}ms",
            generation,
            stats.avg_fitness,
            stats.max_fitness,
            stats.diversity * 100.0,
            stats.elapsed_ms
        );
    }

    /// 最終統計を構築
    fn build_final_stats(&self, _total_elapsed: std::time::Duration) -> Result<SimulationStats> {
        let best_individual = self.best_individual.clone()
            .context("最良個体の情報が見つかりません")?;

        let convergence_info = ConvergenceInfo {
            converged: self.check_convergence()?,
            convergence_generation: None, // 詳細な収束検出は後で実装
            convergence_threshold: 0.01,
            convergence_window: 10,
        };

        let thread_count = if self.parallel_enabled {
            #[cfg(feature = "parallel")]
            {
                rayon::current_num_threads()
            }
            #[cfg(not(feature = "parallel"))]
            {
                1
            }
        } else {
            1
        };

        SimulationStats::new(
            self.generation_history.clone(),
            best_individual,
            convergence_info,
            self.parallel_enabled,
            thread_count,
        ).context("シミュレーション統計の構築に失敗しました")
    }
}

