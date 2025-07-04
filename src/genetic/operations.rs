/// 遺伝的アルゴリズムの操作（選択、交叉、突然変異）
use anyhow::{Result, Context};
use crate::genetic::{individual::Individual, population::Population};
use crate::config::schema::{CrossoverType, SelectionMethod};
use crate::core::random::utils;

/// 選択操作を実行
pub fn select_individuals(
    population: &Population,
    method: &SelectionMethod,
    count: usize,
) -> Result<Vec<Individual>> {
    let mut selected = Vec::with_capacity(count);
    
    for _ in 0..count {
        let individual = match method {
            SelectionMethod::Tournament(size) => {
                population.tournament_selection(*size)?.clone()
            }
            SelectionMethod::Roulette => {
                population.roulette_selection()?.clone()
            }
            SelectionMethod::Rank => {
                rank_selection(population)?
            }
            SelectionMethod::Elite => {
                elite_selection(population)?
            }
        };
        
        selected.push(individual);
    }
    
    Ok(selected)
}

/// ランク選択
fn rank_selection(population: &Population) -> Result<Individual> {
    let individuals = population.individuals();
    let mut ranked: Vec<_> = individuals.iter().enumerate().collect();
    
    // 適応度でソート
    ranked.sort_by(|(_, a), (_, b)| 
        b.fitness().partial_cmp(&a.fitness()).unwrap_or(std::cmp::Ordering::Equal)
    );
    
    // ランクに基づく重みを計算
    let n = ranked.len();
    let weights: Vec<_> = (0..n)
        .map(|i| (n - i) as f64)
        .collect();
    
    // 重み付きランダム選択
    let total_weight: f64 = weights.iter().sum();
    let target = utils::random()? * total_weight;
    let mut current_sum = 0.0;
    
    for (i, &weight) in weights.iter().enumerate() {
        current_sum += weight;
        if current_sum >= target {
            return Ok(ranked[i].1.clone());
        }
    }
    
    // フォールバック
    Ok(ranked.last().unwrap().1.clone())
}

/// エリート選択
fn elite_selection(population: &Population) -> Result<Individual> {
    let best = population.best_individual()?;
    Ok(best.clone())
}

/// 交叉操作を実行
pub fn crossover(
    parent1: &Individual,
    parent2: &Individual,
    crossover_type: &CrossoverType,
) -> Result<(Individual, Individual)> {
    match crossover_type {
        CrossoverType::SinglePoint => single_point_crossover(parent1, parent2),
        CrossoverType::TwoPoint => two_point_crossover(parent1, parent2),
        CrossoverType::Uniform(probability) => uniform_crossover(parent1, parent2, *probability),
    }
}

/// 一点交叉
fn single_point_crossover(
    parent1: &Individual,
    parent2: &Individual,
) -> Result<(Individual, Individual)> {
    let dna_length = parent1.dna().len();
    if dna_length != parent2.dna().len() {
        anyhow::bail!("親のDNA長が異なります");
    }
    
    let crossover_point = utils::random_range(dna_length + 1)?;
    parent1.crossover(parent2, crossover_point)
}

/// 二点交叉
fn two_point_crossover(
    parent1: &Individual,
    parent2: &Individual,
) -> Result<(Individual, Individual)> {
    let dna_length = parent1.dna().len();
    if dna_length != parent2.dna().len() {
        anyhow::bail!("親のDNA長が異なります");
    }
    
    let point1 = utils::random_range(dna_length)?;
    let point2 = utils::random_range(dna_length)?;
    let (start, end) = if point1 <= point2 { (point1, point2) } else { (point2, point1) };
    
    let p1_dna: Vec<char> = parent1.dna().chars().collect();
    let p2_dna: Vec<char> = parent2.dna().chars().collect();
    
    let mut child1_dna = p1_dna.clone();
    let mut child2_dna = p2_dna.clone();
    
    // 指定範囲を交換
    for i in start..end {
        child1_dna[i] = p2_dna[i];
        child2_dna[i] = p1_dna[i];
    }
    
    let child1 = Individual::new(0, child1_dna.into_iter().collect());
    let child2 = Individual::new(0, child2_dna.into_iter().collect());
    
    Ok((child1, child2))
}

/// 一様交叉
fn uniform_crossover(
    parent1: &Individual,
    parent2: &Individual,
    probability: f64,
) -> Result<(Individual, Individual)> {
    let dna_length = parent1.dna().len();
    if dna_length != parent2.dna().len() {
        anyhow::bail!("親のDNA長が異なります");
    }
    
    let p1_dna: Vec<char> = parent1.dna().chars().collect();
    let p2_dna: Vec<char> = parent2.dna().chars().collect();
    
    let mut child1_dna = Vec::with_capacity(dna_length);
    let mut child2_dna = Vec::with_capacity(dna_length);
    
    for i in 0..dna_length {
        if utils::random_bool(probability)? {
            // 交換
            child1_dna.push(p2_dna[i]);
            child2_dna.push(p1_dna[i]);
        } else {
            // そのまま
            child1_dna.push(p1_dna[i]);
            child2_dna.push(p2_dna[i]);
        }
    }
    
    let child1 = Individual::new(0, child1_dna.into_iter().collect());
    let child2 = Individual::new(0, child2_dna.into_iter().collect());
    
    Ok((child1, child2))
}

/// 突然変異操作を実行
pub fn mutate_population(
    population: &mut Population,
    mutation_rate: f64,
) -> Result<()> {
    for individual in population.individuals_mut() {
        mutate_individual(individual, mutation_rate)?;
    }
    Ok(())
}

/// 個体の突然変異
fn mutate_individual(individual: &mut Individual, mutation_rate: f64) -> Result<()> {
    individual.mutate(mutation_rate)
}

/// エリート保存戦略
pub fn apply_elitism(
    old_population: &Population,
    new_population: &mut Population,
    elite_count: usize,
) -> Result<()> {
    if elite_count == 0 {
        return Ok(());
    }
    
    let elite = old_population.get_elite(elite_count);
    let new_individuals = new_population.individuals_mut();
    
    // 新しい個体群の最初の部分をエリートで置き換え
    for (i, elite_individual) in elite.into_iter().enumerate() {
        if i < new_individuals.len() {
            new_individuals[i] = elite_individual;
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genetic::population::Population;

    #[test]
    fn test_selection_methods() -> Result<()> {
        let mut population = Population::new(5, 4)?;
        
        // 適応度を設定
        for i in 0..5 {
            population.set_individual_fitness(i, (i + 1) as f64)?;
        }
        
        // トーナメント選択
        let selected = select_individuals(&population, &SelectionMethod::Tournament(3), 2)?;
        assert_eq!(selected.len(), 2);
        
        // ルーレット選択
        let selected = select_individuals(&population, &SelectionMethod::Roulette, 2)?;
        assert_eq!(selected.len(), 2);
        
        // ランク選択
        let selected = select_individuals(&population, &SelectionMethod::Rank, 2)?;
        assert_eq!(selected.len(), 2);
        
        // エリート選択
        let selected = select_individuals(&population, &SelectionMethod::Elite, 2)?;
        assert_eq!(selected.len(), 2);

        Ok(())
    }

    #[test]
    fn test_crossover_methods() -> Result<()> {
        let parent1 = Individual::new(1, "111111".to_string());
        let parent2 = Individual::new(2, "000000".to_string());
        
        // 一点交叉
        let (child1, child2) = crossover(&parent1, &parent2, &CrossoverType::SinglePoint)?;
        assert_eq!(child1.dna().len(), 6);
        assert_eq!(child2.dna().len(), 6);
        
        // 二点交叉
        let (child1, child2) = crossover(&parent1, &parent2, &CrossoverType::TwoPoint)?;
        assert_eq!(child1.dna().len(), 6);
        assert_eq!(child2.dna().len(), 6);
        
        // 一様交叉
        let (child1, child2) = crossover(&parent1, &parent2, &CrossoverType::Uniform(0.5))?;
        assert_eq!(child1.dna().len(), 6);
        assert_eq!(child2.dna().len(), 6);

        Ok(())
    }

    #[test]
    fn test_mutation() -> Result<()> {
        let mut population = Population::new(5, 10)?;
        
        // 突然変異前のDNAを記録
        let original_dnas: Vec<String> = population.individuals()
            .iter()
            .map(|ind| ind.dna().to_string())
            .collect();
        
        // 高い突然変異率で実行
        mutate_population(&mut population, 0.5)?;
        
        // 一部のDNAが変化していることを確認
        let mut changed_count = 0;
        for (i, individual) in population.individuals().iter().enumerate() {
            if individual.dna() != original_dnas[i] {
                changed_count += 1;
            }
        }
        
        // 高い突然変異率なので、いくつかは変化しているはず
        assert!(changed_count > 0);

        Ok(())
    }

    #[test]
    fn test_elitism() -> Result<()> {
        let mut old_population = Population::new(5, 4)?;
        let mut new_population = Population::new(5, 4)?;
        
        // 旧個体群に適応度を設定
        for i in 0..5 {
            old_population.set_individual_fitness(i, (i + 1) as f64)?;
        }
        
        // 新個体群の適応度は低く設定
        for i in 0..5 {
            new_population.set_individual_fitness(i, 0.1)?;
        }
        
        // エリート保存を適用
        apply_elitism(&old_population, &mut new_population, 2)?;
        
        // 上位2個体の適応度が高いことを確認
        let top_individuals = new_population.get_elite(2);
        assert_eq!(top_individuals.len(), 2);
        assert!(top_individuals[0].fitness() > 1.0);
        assert!(top_individuals[1].fitness() > 1.0);

        Ok(())
    }
}