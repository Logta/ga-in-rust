/// 遺伝的アルゴリズムのメインエンジン
use anyhow::Result;
use crate::genetic::{individual::Individual, population::Population, operations};
use crate::config::schema::{GeneticConfig, SelectionMethod};

/// 遺伝的アルゴリズムの実装
pub struct GeneticAlgorithm {
    config: GeneticConfig,
    current_generation: usize,
    population: Population,
}

impl GeneticAlgorithm {
    /// 新しい遺伝的アルゴリズムを作成
    pub fn new(config: GeneticConfig) -> Result<Self> {
        // 設定の検証
        config.validate()?;
        
        // 初期個体群を生成
        let population = Population::new(config.population_size, config.dna_length)?;
        
        Ok(Self {
            config,
            current_generation: 0,
            population,
        })
    }
    
    /// 現在の世代数を取得
    pub fn current_generation(&self) -> usize {
        self.current_generation
    }
    
    /// 現在の個体群を取得
    pub fn population(&self) -> &Population {
        &self.population
    }
    
    /// 現在の個体群への可変参照を取得
    pub fn population_mut(&mut self) -> &mut Population {
        &mut self.population
    }
    
    /// 個体の適応度を設定
    pub fn set_fitness(&mut self, individual_index: usize, fitness: f64) -> Result<()> {
        self.population.set_individual_fitness(individual_index, fitness)
    }
    
    /// すべての個体の適応度を設定
    pub fn set_all_fitness(&mut self, fitness_values: &[f64]) -> Result<()> {
        if fitness_values.len() != self.population.size() {
            anyhow::bail!(
                "適応度配列のサイズ{}が個体数{}と一致しません",
                fitness_values.len(),
                self.population.size()
            );
        }
        
        for (i, &fitness) in fitness_values.iter().enumerate() {
            self.set_fitness(i, fitness)?;
        }
        
        Ok(())
    }
    
    /// 次世代を生成
    pub fn evolve(&mut self) -> Result<()> {
        let mut new_individuals = Vec::new();
        
        // エリート保存のための古い個体群をコピー
        let old_population = self.population.clone();
        
        // 新しい個体群を生成（エリート分を除く）
        let offspring_count = self.config.population_size - self.config.elite_count;
        
        while new_individuals.len() < offspring_count {
            // 親選択
            let parent1 = self.select_parent()?;
            let parent2 = self.select_parent()?;
            
            // 交叉
            let (mut child1, mut child2) = operations::crossover(
                &parent1,
                &parent2,
                &self.config.crossover_type,
            )?;
            
            // 突然変異
            child1.mutate(self.config.mutation_rate)?;
            child2.mutate(self.config.mutation_rate)?;
            
            // IDを設定
            let next_id = new_individuals.len();
            child1 = Individual::new(next_id, child1.dna().to_string());
            
            new_individuals.push(child1);
            
            if new_individuals.len() < offspring_count {
                let next_id = new_individuals.len();
                child2 = Individual::new(next_id, child2.dna().to_string());
                new_individuals.push(child2);
            }
        }
        
        // エリート個体を取得
        let elite_individuals = old_population.get_elite(self.config.elite_count);
        
        // エリート + 子個体で新しい個体群を構成
        let mut all_individuals = elite_individuals;
        all_individuals.extend(new_individuals);
        
        // IDを再設定
        for (i, individual) in all_individuals.iter_mut().enumerate() {
            *individual = Individual::new(i, individual.dna().to_string());
        }
        
        // 新しい個体群を作成
        let new_population = Population::from_individuals(all_individuals)?;
        
        // 個体群を置き換え
        self.population = new_population;
        self.current_generation += 1;
        
        Ok(())
    }
    
    /// 親個体を選択
    fn select_parent(&self) -> Result<Individual> {
        match &self.config.selection_method {
            SelectionMethod::Tournament(size) => {
                Ok(self.population.tournament_selection(*size)?.clone())
            }
            SelectionMethod::Roulette => {
                Ok(self.population.roulette_selection()?.clone())
            }
            SelectionMethod::Rank => {
                let selected = operations::select_individuals(
                    &self.population,
                    &SelectionMethod::Rank,
                    1,
                )?;
                Ok(selected[0].clone())
            }
            SelectionMethod::Elite => {
                Ok(self.population.best_individual()?.clone())
            }
        }
    }
    
    /// 最良個体を取得
    pub fn best_individual(&self) -> Result<&Individual> {
        self.population.best_individual()
    }
    
    /// 統計情報を取得
    pub fn statistics(&self) -> Result<GAStatistics> {
        let best = self.best_individual()?;
        let fitness_values: Vec<f64> = self.population.individuals()
            .iter()
            .map(|ind| ind.fitness())
            .collect();
        
        let avg_fitness = self.population.average_fitness();
        let std_deviation = self.population.fitness_std_deviation();
        let diversity = self.population.diversity()?;
        
        Ok(GAStatistics {
            generation: self.current_generation,
            best_fitness: best.fitness(),
            avg_fitness,
            std_deviation,
            diversity,
            fitness_values,
        })
    }
    
    /// 収束判定
    pub fn is_converged(&self, threshold: f64) -> Result<bool> {
        let std_dev = self.population.fitness_std_deviation();
        Ok(std_dev < threshold)
    }
    
    /// 世代を指定回数進める
    pub fn evolve_generations(&mut self, generations: usize) -> Result<Vec<GAStatistics>> {
        let mut stats_history = Vec::new();
        
        for _ in 0..generations {
            let stats = self.statistics()?;
            stats_history.push(stats);
            
            self.evolve()?;
        }
        
        // 最終世代の統計も追加
        let final_stats = self.statistics()?;
        stats_history.push(final_stats);
        
        Ok(stats_history)
    }
}

/// GA統計情報
#[derive(Debug, Clone)]
pub struct GAStatistics {
    pub generation: usize,
    pub best_fitness: f64,
    pub avg_fitness: f64,
    pub std_deviation: f64,
    pub diversity: f64,
    pub fitness_values: Vec<f64>,
}

impl GAStatistics {
    /// 収束度を計算（0.0-1.0）
    pub fn convergence_rate(&self) -> f64 {
        if self.best_fitness == 0.0 {
            return 0.0;
        }
        
        // 標準偏差の逆数を使った収束度
        let normalized_std_dev = self.std_deviation / self.best_fitness.abs();
        (1.0 - normalized_std_dev.min(1.0)).max(0.0)
    }
    
    /// 改善率を計算
    pub fn improvement_rate(&self, previous: &GAStatistics) -> f64 {
        if previous.best_fitness == 0.0 {
            return 0.0;
        }
        
        (self.best_fitness - previous.best_fitness) / previous.best_fitness.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::schema::CrossoverType;

    fn create_test_config() -> GeneticConfig {
        GeneticConfig {
            population_size: 10,
            generations: 5,
            mutation_rate: 0.01,
            elite_count: 2,
            dna_length: 8,
            crossover_type: CrossoverType::SinglePoint,
            selection_method: SelectionMethod::Tournament(3),
        }
    }

    #[test]
    fn test_ga_creation() -> Result<()> {
        let config = create_test_config();
        let ga = GeneticAlgorithm::new(config)?;
        
        assert_eq!(ga.current_generation(), 0);
        assert_eq!(ga.population().size(), 10);
        
        Ok(())
    }

    #[test]
    fn test_fitness_setting() -> Result<()> {
        let config = create_test_config();
        let mut ga = GeneticAlgorithm::new(config)?;
        
        let fitness_values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        ga.set_all_fitness(&fitness_values)?;
        
        let best = ga.best_individual()?;
        assert_eq!(best.fitness(), 10.0);
        
        Ok(())
    }

    #[test]
    fn test_evolution() -> Result<()> {
        let config = create_test_config();
        let mut ga = GeneticAlgorithm::new(config)?;
        
        // 初期適応度を設定
        let fitness_values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        ga.set_all_fitness(&fitness_values)?;
        
        let initial_generation = ga.current_generation();
        ga.evolve()?;
        
        assert_eq!(ga.current_generation(), initial_generation + 1);
        assert_eq!(ga.population().size(), 10);
        
        Ok(())
    }

    #[test]
    fn test_statistics() -> Result<()> {
        let config = create_test_config();
        let mut ga = GeneticAlgorithm::new(config)?;
        
        let fitness_values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        ga.set_all_fitness(&fitness_values)?;
        
        let stats = ga.statistics()?;
        
        assert_eq!(stats.generation, 0);
        assert_eq!(stats.best_fitness, 10.0);
        assert_eq!(stats.avg_fitness, 5.5);
        assert!(stats.std_deviation > 0.0);
        assert!(stats.diversity >= 0.0 && stats.diversity <= 1.0);
        
        Ok(())
    }

    #[test]
    fn test_convergence() -> Result<()> {
        let config = create_test_config();
        let mut ga = GeneticAlgorithm::new(config)?;
        
        // 全個体に同じ適応度を設定（収束状態）
        let fitness_values = vec![5.0; 10];
        ga.set_all_fitness(&fitness_values)?;
        
        assert!(ga.is_converged(0.1)?);
        
        Ok(())
    }

    #[test]
    fn test_multi_generation_evolution() -> Result<()> {
        let config = create_test_config();
        let mut ga = GeneticAlgorithm::new(config)?;
        
        let fitness_values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        ga.set_all_fitness(&fitness_values)?;
        
        let stats_history = ga.evolve_generations(3)?;
        
        assert_eq!(stats_history.len(), 4); // 初期 + 3世代
        assert_eq!(ga.current_generation(), 3);
        
        Ok(())
    }
}