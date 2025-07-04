/// シミュレーション実行ランナー
///
/// 複数のシミュレーション実行と結果の管理を行う
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;
use crate::config::Config;
use crate::simulation::engine::Simulation;
use crate::simulation::stats::SimulationStats;

/// シミュレーション実行ランナー
pub struct SimulationRunner {
    /// ベース設定
    base_config: Config,
    
    /// 実行結果のストレージ
    results: HashMap<String, SimulationStats>,
    
    /// 現在実行中のシミュレーション名
    current_simulation: Option<String>,
}

impl SimulationRunner {
    /// 新しいランナーを作成
    pub fn new(base_config: Config) -> Result<Self> {
        base_config.validate()
            .context("ベース設定の検証に失敗しました")?;

        Ok(Self {
            base_config,
            results: HashMap::new(),
            current_simulation: None,
        })
    }

    /// 単一のシミュレーションを実行
    pub async fn run_single(
        &mut self,
        name: String,
        config_override: Option<Config>,
        seed: Option<u32>,
    ) -> Result<SimulationStats> {
        let config = config_override.unwrap_or_else(|| self.base_config.clone());
        
        tracing::info!("シミュレーション '{}' を開始します", name);
        self.current_simulation = Some(name.clone());

        let mut simulation = Simulation::new(config, seed)
            .context("シミュレーションの作成に失敗しました")?;

        let stats = simulation.run().await
            .context("シミュレーションの実行に失敗しました")?;

        self.results.insert(name.clone(), stats.clone());
        self.current_simulation = None;

        tracing::info!("シミュレーション '{}' が完了しました", name);
        
        Ok(stats)
    }

    /// パラメータスイープを実行
    pub async fn run_parameter_sweep(
        &mut self,
        base_name: &str,
        parameter_variations: Vec<(&str, Vec<ParameterValue>)>,
        seed: Option<u32>,
    ) -> Result<Vec<(String, SimulationStats)>> {
        tracing::info!(
            "パラメータスイープを開始します: {} のバリエーション",
            parameter_variations.len()
        );

        let mut results = Vec::new();
        let variations = self.generate_parameter_combinations(&parameter_variations)?;

        let base_config = self.base_config.clone();
        
        for (i, variation) in variations.iter().enumerate() {
            let simulation_name = format!("{}_{:03}", base_name, i);
            let config = Self::apply_parameter_variation_static(&base_config, variation)?;
            
            let stats = self.run_single(simulation_name.clone(), Some(config), seed).await
                .context("パラメータスイープ中のシミュレーション実行に失敗しました")?;
            
            results.push((simulation_name, stats));
            
            tracing::info!(
                "パラメータスイープ進捗: {}/{} 完了",
                i + 1,
                variations.len()
            );
        }

        tracing::info!("パラメータスイープが完了しました");
        Ok(results)
    }

    /// 複数回実行（統計的信頼性向上）
    pub async fn run_multiple(
        &mut self,
        base_name: &str,
        runs: usize,
        config_override: Option<Config>,
    ) -> Result<Vec<(String, SimulationStats)>> {
        tracing::info!("{}回の反復実行を開始します", runs);

        let mut results = Vec::new();
        for i in 0..runs {
            let simulation_name = format!("{}_{:03}", base_name, i);
            let seed = Some((i as u32) * 1000); // 異なるシードを使用
            
            let stats = self.run_single(simulation_name.clone(), config_override.clone(), seed).await
                .context("反復実行中のシミュレーション実行に失敗しました")?;
            
            results.push((simulation_name, stats));
            
            if i % 10 == 9 {
                tracing::info!("反復実行進捗: {}/{} 完了", i + 1, runs);
            }
        }

        tracing::info!("反復実行が完了しました");
        Ok(results)
    }

    /// 結果の比較分析
    pub fn compare_results(&self, simulation_names: &[String]) -> Result<ComparisonResult> {
        let mut stats_list = Vec::new();
        
        for name in simulation_names {
            let stats = self.results.get(name)
                .context(format!("シミュレーション '{}' の結果が見つかりません", name))?;
            stats_list.push((name.clone(), stats));
        }

        if stats_list.len() < 2 {
            anyhow::bail!("比較には少なくとも2つの結果が必要です");
        }

        ComparisonResult::new(stats_list)
    }

    /// 結果を保存
    pub async fn save_results(&self, output_dir: &Path) -> Result<()> {
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir)
                .context("出力ディレクトリの作成に失敗しました")?;
        }

        for (name, stats) in &self.results {
            let file_path = output_dir.join(format!("{}.json", name));
            
            let json = serde_json::to_string_pretty(stats)
                .context("結果のJSON変換に失敗しました")?;
            
            tokio::fs::write(&file_path, json).await
                .context(format!("結果の保存に失敗しました: {}", file_path.display()))?;
        }

        // サマリーレポートの生成
        let summary_path = output_dir.join("summary.txt");
        let summary = self.generate_summary_report()?;
        
        tokio::fs::write(&summary_path, summary).await
            .context("サマリーレポートの保存に失敗しました")?;

        tracing::info!("結果を {} に保存しました", output_dir.display());
        Ok(())
    }

    /// 結果を読み込み
    pub async fn load_results(&mut self, input_dir: &Path) -> Result<()> {
        if !input_dir.exists() {
            anyhow::bail!("入力ディレクトリが存在しません: {}", input_dir.display());
        }

        let mut entries = tokio::fs::read_dir(input_dir).await
            .context("ディレクトリの読み取りに失敗しました")?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "json") {
                let name = path.file_stem()
                    .and_then(|stem| stem.to_str())
                    .context("ファイル名の解析に失敗しました")?
                    .to_string();

                let content = tokio::fs::read_to_string(&path).await
                    .context(format!("ファイルの読み取りに失敗しました: {}", path.display()))?;

                let stats: SimulationStats = serde_json::from_str(&content)
                    .context(format!("JSONの解析に失敗しました: {}", path.display()))?;

                self.results.insert(name, stats);
            }
        }

        tracing::info!("結果を {} から読み込みました", input_dir.display());
        Ok(())
    }

    /// 現在の結果一覧を取得
    pub fn list_results(&self) -> Vec<&String> {
        self.results.keys().collect()
    }

    /// 特定の結果を取得
    pub fn get_result(&self, name: &str) -> Option<&SimulationStats> {
        self.results.get(name)
    }

    /// 結果を削除
    pub fn remove_result(&mut self, name: &str) -> Option<SimulationStats> {
        self.results.remove(name)
    }

    /// 全ての結果をクリア
    pub fn clear_results(&mut self) {
        self.results.clear();
    }

    /// パラメータの組み合わせを生成
    fn generate_parameter_combinations(
        &self,
        variations: &[(&str, Vec<ParameterValue>)],
    ) -> Result<Vec<Vec<(String, ParameterValue)>>> {
        if variations.is_empty() {
            return Ok(vec![vec![]]);
        }

        let mut combinations = vec![vec![]];
        
        for (param_name, values) in variations {
            let mut new_combinations = Vec::new();
            
            for combination in &combinations {
                for value in values {
                    let mut new_combination = combination.clone();
                    new_combination.push((param_name.to_string(), value.clone()));
                    new_combinations.push(new_combination);
                }
            }
            
            combinations = new_combinations;
        }

        Ok(combinations)
    }

    /// パラメータバリエーションを設定に適用
    fn apply_parameter_variation_static(
        base_config: &Config,
        variation: &[(String, ParameterValue)],
    ) -> Result<Config> {
        let mut config = base_config.clone();

        for (param_name, value) in variation {
            match param_name.as_str() {
                "population_size" => {
                    if let ParameterValue::Integer(val) = value {
                        config.genetic.population_size = *val as usize;
                    }
                }
                "generations" => {
                    if let ParameterValue::Integer(val) = value {
                        config.genetic.generations = *val as usize;
                    }
                }
                "mutation_rate" => {
                    if let ParameterValue::Float(val) = value {
                        config.genetic.mutation_rate = *val;
                    }
                }
                "rounds_per_match" => {
                    if let ParameterValue::Integer(val) = value {
                        config.simulation.rounds_per_match = *val as usize;
                    }
                }
                _ => {
                    tracing::warn!("未知のパラメータです: {}", param_name);
                }
            }
        }

        config.validate()
            .context("パラメータ適用後の設定検証に失敗しました")?;

        Ok(config)
    }

    /// サマリーレポートを生成
    fn generate_summary_report(&self) -> Result<String> {
        let mut report = String::new();
        report.push_str("# シミュレーション結果サマリー\n\n");
        
        report.push_str(&format!("総実行数: {}\n\n", self.results.len()));

        if self.results.is_empty() {
            report.push_str("実行結果がありません。\n");
            return Ok(report);
        }

        // 統計情報の集計
        let mut best_fitness_values = Vec::new();
        let mut final_avg_fitness_values = Vec::new();
        let mut total_generations = Vec::new();

        for (name, stats) in &self.results {
            report.push_str(&format!("## {}\n", name));
            report.push_str(&format!("{}\n\n", stats.summary()));
            
            best_fitness_values.push(stats.best_individual.best_fitness);
            final_avg_fitness_values.push(stats.final_stats.avg_fitness);
            total_generations.push(stats.total_generations);
        }

        // 全体統計
        report.push_str("## 全体統計\n");
        report.push_str(&format!(
            "最高適応度: 平均={:.2}, 最大={:.2}, 最小={:.2}\n",
            best_fitness_values.iter().sum::<f64>() / best_fitness_values.len() as f64,
            best_fitness_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)),
            best_fitness_values.iter().fold(f64::INFINITY, |a, &b| a.min(b))
        ));
        
        report.push_str(&format!(
            "最終平均適応度: 平均={:.2}, 最大={:.2}, 最小={:.2}\n",
            final_avg_fitness_values.iter().sum::<f64>() / final_avg_fitness_values.len() as f64,
            final_avg_fitness_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)),
            final_avg_fitness_values.iter().fold(f64::INFINITY, |a, &b| a.min(b))
        ));

        Ok(report)
    }
}

/// パラメータ値
#[derive(Debug, Clone)]
pub enum ParameterValue {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

/// 比較結果
pub struct ComparisonResult {
    simulations: Vec<(String, f64, f64, usize)>, // name, best_fitness, avg_fitness, generations
}

impl ComparisonResult {
    fn new(stats_list: Vec<(String, &SimulationStats)>) -> Result<Self> {
        let simulations = stats_list
            .into_iter()
            .map(|(name, stats)| {
                (
                    name,
                    stats.best_individual.best_fitness,
                    stats.final_stats.avg_fitness,
                    stats.total_generations,
                )
            })
            .collect();

        Ok(Self { simulations })
    }

    /// 比較レポートを生成
    pub fn report(&self) -> String {
        let mut report = String::new();
        report.push_str("# シミュレーション比較結果\n\n");

        // ソートして表示
        let mut sorted_sims = self.simulations.clone();
        sorted_sims.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); // 最高適応度でソート

        report.push_str("| 順位 | シミュレーション名 | 最高適応度 | 最終平均適応度 | 世代数 |\n");
        report.push_str("|------|-------------------|-----------|---------------|-------|\n");

        for (rank, (name, best, avg, gens)) in sorted_sims.iter().enumerate() {
            report.push_str(&format!(
                "| {:4} | {:17} | {:9.2} | {:13.2} | {:5} |\n",
                rank + 1, name, best, avg, gens
            ));
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::schema::*;

    fn create_test_config() -> Config {
        Config {
            simulation: SimulationConfig {
                default_strategy: "tit-for-tat".to_string(),
                rounds_per_match: 10,
                payoff_matrix: PayoffMatrix::default(),
                tournament_type: TournamentType::RoundRobin,
            },
            genetic: GeneticConfig {
                population_size: 50,
                generations: 100,
                mutation_rate: 0.01,
                elite_count: 2,
                dna_length: 8,
                crossover_type: CrossoverType::SinglePoint,
                selection_method: SelectionMethod::Tournament(2),
            },
            output: OutputConfig::default(),
            performance: PerformanceConfig::default(),
            strategies: std::collections::HashMap::new(),
        }
    }

    #[test]
    fn test_runner_creation() {
        let config = create_test_config();
        let runner = SimulationRunner::new(config);
        assert!(runner.is_ok());
    }

    #[test]
    fn test_parameter_combinations() {
        let config = create_test_config();
        let runner = SimulationRunner::new(config).unwrap();
        
        let variations = vec![
            ("population_size", vec![ParameterValue::Integer(50), ParameterValue::Integer(100)]),
            ("mutation_rate", vec![ParameterValue::Float(0.01), ParameterValue::Float(0.05)]),
        ];
        
        let combinations = runner.generate_parameter_combinations(&variations).unwrap();
        assert_eq!(combinations.len(), 4); // 2 × 2 = 4 combinations
    }

    #[test]
    fn test_parameter_application() {
        let config = create_test_config();
        let runner = SimulationRunner::new(config).unwrap();
        
        let variation = vec![
            ("population_size".to_string(), ParameterValue::Integer(200)),
            ("mutation_rate".to_string(), ParameterValue::Float(0.05)),
        ];
        
        let new_config = runner.apply_parameter_variation(&variation).unwrap();
        assert_eq!(new_config.genetic.population_size, 200);
        assert_eq!(new_config.genetic.mutation_rate, 0.05);
    }

    #[test]
    fn test_comparison_result() {
        use crate::simulation::stats::*;
        
        let stats1 = SimulationStats {
            total_generations: 100,
            final_stats: GenerationStats::new(99, &[10.0, 15.0, 12.0], 0.8, 2, 100).unwrap(),
            generation_history: vec![],
            best_individual: BestIndividualInfo {
                best_fitness: 15.0,
                best_generation: 50,
                best_dna: "101010".to_string(),
                description: "Test".to_string(),
            },
            convergence_info: ConvergenceInfo {
                converged: true,
                convergence_generation: Some(80),
                convergence_threshold: 0.01,
                convergence_window: 10,
            },
            performance_info: PerformanceInfo {
                total_elapsed_ms: 5000,
                avg_generation_time_ms: 50.0,
                fastest_generation_ms: 30,
                slowest_generation_ms: 80,
                parallel_enabled: false,
                thread_count: 1,
            },
        };
        
        let stats2 = SimulationStats {
            best_individual: BestIndividualInfo {
                best_fitness: 20.0,
                best_generation: 30,
                best_dna: "111000".to_string(),
                description: "Test2".to_string(),
            },
            ..stats1.clone()
        };
        
        let comparison = ComparisonResult::new(vec![
            ("sim1".to_string(), &stats1),
            ("sim2".to_string(), &stats2),
        ]).unwrap();
        
        let report = comparison.report();
        assert!(report.contains("sim1"));
        assert!(report.contains("sim2"));
        assert!(report.contains("15.00"));
        assert!(report.contains("20.00"));
    }
}