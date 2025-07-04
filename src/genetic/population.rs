/// 個体群の実装
use anyhow::{Result, Context};
use crate::genetic::individual::Individual;
use crate::core::random::utils;

/// 遺伝的アルゴリズムの個体群
#[derive(Debug, Clone)]
pub struct Population {
    individuals: Vec<Individual>,
}

impl Population {
    /// 新しい個体群を作成
    pub fn new(size: usize, dna_length: usize) -> Result<Self> {
        let mut individuals = Vec::with_capacity(size);
        
        for i in 0..size {
            let dna = generate_random_dna(dna_length)?;
            individuals.push(Individual::new(i, dna));
        }
        
        Ok(Self { individuals })
    }
    
    /// 個体群のサイズを取得
    pub fn size(&self) -> usize {
        self.individuals.len()
    }
    
    /// すべての個体への参照を取得
    pub fn individuals(&self) -> &[Individual] {
        &self.individuals
    }
    
    /// すべての個体への可変参照を取得
    pub fn individuals_mut(&mut self) -> &mut [Individual] {
        &mut self.individuals
    }
    
    /// 指定されたインデックスの個体を取得
    pub fn get_individual(&self, index: usize) -> Result<&Individual> {
        self.individuals
            .get(index)
            .with_context(|| format!("個体インデックス{}が範囲外です（サイズ: {}）", index, self.size()))
    }
    
    /// 指定されたインデックスの個体の可変参照を取得
    pub fn get_individual_mut(&mut self, index: usize) -> Result<&mut Individual> {
        let size = self.size();
        self.individuals
            .get_mut(index)
            .with_context(|| format!("個体インデックス{}が範囲外です（サイズ: {}）", index, size))
    }
    
    /// 指定されたインデックスの個体の適応度を設定
    pub fn set_individual_fitness(&mut self, index: usize, fitness: f64) -> Result<()> {
        let individual = self.get_individual_mut(index)?;
        individual.set_fitness(fitness);
        Ok(())
    }
    
    /// 最も適応度の高い個体を取得
    pub fn best_individual(&self) -> Result<&Individual> {
        self.individuals
            .iter()
            .max_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap_or(std::cmp::Ordering::Equal))
            .context("個体群が空です")
    }
    
    /// 平均適応度を計算
    pub fn average_fitness(&self) -> f64 {
        if self.individuals.is_empty() {
            return 0.0;
        }
        
        let total: f64 = self.individuals.iter().map(|ind| ind.fitness()).sum();
        total / self.individuals.len() as f64
    }
    
    /// 適応度の標準偏差を計算
    pub fn fitness_std_deviation(&self) -> f64 {
        if self.individuals.len() <= 1 {
            return 0.0;
        }
        
        let mean = self.average_fitness();
        let variance: f64 = self.individuals
            .iter()
            .map(|ind| {
                let diff = ind.fitness() - mean;
                diff * diff
            })
            .sum::<f64>() / (self.individuals.len() - 1) as f64;
        
        variance.sqrt()
    }
    
    /// 多様性を計算（DNA距離の平均）
    pub fn diversity(&self) -> Result<f64> {
        if self.individuals.len() <= 1 {
            return Ok(0.0);
        }
        
        let mut total_distance = 0;
        let mut pair_count = 0;
        
        for i in 0..self.individuals.len() {
            for j in (i + 1)..self.individuals.len() {
                let distance = self.individuals[i].dna_distance(&self.individuals[j])?;
                total_distance += distance;
                pair_count += 1;
            }
        }
        
        if pair_count == 0 {
            return Ok(0.0);
        }
        
        // DNA長さで正規化
        let dna_length = self.individuals[0].dna().len() as f64;
        Ok((total_distance as f64 / pair_count as f64) / dna_length)
    }
    
    /// 個体群を適応度でソート（降順）
    pub fn sort_by_fitness(&mut self) {
        self.individuals.sort_by(|a, b| 
            b.fitness().partial_cmp(&a.fitness()).unwrap_or(std::cmp::Ordering::Equal)
        );
    }
    
    /// エリート個体を取得
    pub fn get_elite(&self, count: usize) -> Vec<Individual> {
        let mut sorted = self.individuals.clone();
        sorted.sort_by(|a, b| 
            b.fitness().partial_cmp(&a.fitness()).unwrap_or(std::cmp::Ordering::Equal)
        );
        
        sorted.into_iter().take(count).collect()
    }
    
    /// トーナメント選択を実行
    pub fn tournament_selection(&self, tournament_size: usize) -> Result<&Individual> {
        if tournament_size > self.individuals.len() {
            anyhow::bail!(
                "トーナメントサイズ{}が個体数{}より大きいです",
                tournament_size,
                self.individuals.len()
            );
        }
        
        let mut best: Option<&Individual> = None;
        
        for _ in 0..tournament_size {
            let random_index = utils::random_range(self.individuals.len())?;
            let candidate = &self.individuals[random_index];
            
            match best {
                None => best = Some(candidate),
                Some(current_best) => {
                    if candidate.fitness() > current_best.fitness() {
                        best = Some(candidate);
                    }
                }
            }
        }
        
        best.context("トーナメント選択で個体を選択できませんでした")
    }
    
    /// ルーレット選択を実行
    pub fn roulette_selection(&self) -> Result<&Individual> {
        let total_fitness: f64 = self.individuals.iter().map(|ind| ind.fitness()).sum();
        
        if total_fitness <= 0.0 {
            // 適応度がすべて0以下の場合はランダム選択
            let random_index = utils::random_range(self.individuals.len())?;
            return Ok(&self.individuals[random_index]);
        }
        
        let target = utils::random()? * total_fitness;
        let mut current_sum = 0.0;
        
        for individual in &self.individuals {
            current_sum += individual.fitness();
            if current_sum >= target {
                return Ok(individual);
            }
        }
        
        // 浮動小数点の誤差により最後の個体を返す
        Ok(self.individuals.last().unwrap())
    }
    
    /// 個体群を次世代に置き換える
    pub fn replace_with(&mut self, new_individuals: Vec<Individual>) -> Result<()> {
        if new_individuals.len() != self.individuals.len() {
            anyhow::bail!(
                "新しい個体群のサイズ{}が現在のサイズ{}と異なります",
                new_individuals.len(),
                self.individuals.len()
            );
        }
        
        self.individuals = new_individuals;
        Ok(())
    }
}

/// ランダムなDNA文字列を生成
fn generate_random_dna(length: usize) -> Result<String> {
    let mut dna = String::with_capacity(length);
    
    for _ in 0..length {
        let bit = if utils::random_bool(0.5)? { '1' } else { '0' };
        dna.push(bit);
    }
    
    Ok(dna)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_population_creation() -> Result<()> {
        let population = Population::new(5, 4)?;
        
        assert_eq!(population.size(), 5);
        assert_eq!(population.individuals().len(), 5);
        
        // 各個体のDNA長が正しいことを確認
        for individual in population.individuals() {
            assert_eq!(individual.dna().len(), 4);
            // DNAが0と1のみで構成されていることを確認
            for c in individual.dna().chars() {
                assert!(c == '0' || c == '1');
            }
        }

        Ok(())
    }

    #[test]
    fn test_individual_access() -> Result<()> {
        let mut population = Population::new(3, 4)?;
        
        // 個体の取得
        let individual = population.get_individual(0)?;
        assert_eq!(individual.id(), 0);
        
        // 適応度の設定
        population.set_individual_fitness(0, 42.5)?;
        let updated_individual = population.get_individual(0)?;
        assert_eq!(updated_individual.fitness(), 42.5);

        Ok(())
    }

    #[test]
    fn test_best_individual() -> Result<()> {
        let mut population = Population::new(3, 4)?;
        
        // 適応度を設定
        population.set_individual_fitness(0, 10.0)?;
        population.set_individual_fitness(1, 20.0)?;
        population.set_individual_fitness(2, 15.0)?;
        
        let best = population.best_individual()?;
        assert_eq!(best.fitness(), 20.0);
        assert_eq!(best.id(), 1);

        Ok(())
    }

    #[test]
    fn test_invalid_access() {
        let population = Population::new(3, 4).unwrap();
        
        // 範囲外アクセス
        assert!(population.get_individual(3).is_err());
        assert!(population.get_individual(10).is_err());
    }

    #[test]
    fn test_statistics() -> Result<()> {
        let mut population = Population::new(3, 4)?;
        
        population.set_individual_fitness(0, 10.0)?;
        population.set_individual_fitness(1, 20.0)?;
        population.set_individual_fitness(2, 15.0)?;
        
        let avg = population.average_fitness();
        assert!((avg - 15.0).abs() < 0.001);
        
        let std_dev = population.fitness_std_deviation();
        assert!(std_dev > 0.0);
        
        let diversity = population.diversity()?;
        assert!(diversity >= 0.0 && diversity <= 1.0);

        Ok(())
    }

    #[test]
    fn test_selection_methods() -> Result<()> {
        let mut population = Population::new(5, 4)?;
        
        // 適応度を設定
        for i in 0..5 {
            population.set_individual_fitness(i, (i + 1) as f64)?;
        }
        
        // トーナメント選択
        let selected = population.tournament_selection(3)?;
        assert!(selected.fitness() > 0.0);
        
        // ルーレット選択
        let selected = population.roulette_selection()?;
        assert!(selected.fitness() > 0.0);

        Ok(())
    }
}