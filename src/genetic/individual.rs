/// 個体の実装
use anyhow::{Result, ensure};
use crate::simulation::environment::Choice;

/// 遺伝的アルゴリズムの個体
#[derive(Debug, Clone)]
pub struct Individual {
    id: usize,
    dna: String,
    fitness: f64,
}

impl Individual {
    /// 新しい個体を作成
    pub fn new(id: usize, dna: String) -> Self {
        Self {
            id,
            dna,
            fitness: 0.0,
        }
    }
    
    /// 個体のIDを取得
    pub fn id(&self) -> usize {
        self.id
    }
    
    /// 個体のDNAを取得
    pub fn dna(&self) -> &str {
        &self.dna
    }
    
    /// 個体の適応度を取得
    pub fn fitness(&self) -> f64 {
        self.fitness
    }
    
    /// 個体の適応度を設定
    pub fn set_fitness(&mut self, fitness: f64) {
        self.fitness = fitness;
    }
    
    /// 指定されたラウンドでの選択を決定
    pub fn choose(&self, _history: &[Choice], round: usize) -> Result<Choice> {
        ensure!(!self.dna.is_empty(), "DNAが空です");
        
        let bit_index = round % self.dna.len();
        let bit = self.dna.chars().nth(bit_index)
            .ok_or_else(|| anyhow::anyhow!("DNAの位置{}にアクセスできません", bit_index))?;
        
        match bit {
            '1' => Ok(Choice::Cooperate),
            '0' => Ok(Choice::Defect),
            _ => anyhow::bail!("無効なDNAビット: {}", bit),
        }
    }
    
    /// 他の個体とのDNA距離を計算（ハミング距離）
    pub fn dna_distance(&self, other: &Individual) -> Result<usize> {
        ensure!(
            self.dna.len() == other.dna.len(),
            "DNA長が異なります: {} vs {}",
            self.dna.len(),
            other.dna.len()
        );
        
        let distance = self.dna.chars()
            .zip(other.dna.chars())
            .map(|(a, b)| if a != b { 1 } else { 0 })
            .sum();
        
        Ok(distance)
    }
    
    /// 個体を変異させる
    pub fn mutate(&mut self, mutation_rate: f64) -> Result<()> {
        use crate::core::random::utils;
        
        let mut new_dna = String::new();
        for c in self.dna.chars() {
            if utils::random_bool(mutation_rate)? {
                // ビットを反転
                new_dna.push(if c == '0' { '1' } else { '0' });
            } else {
                new_dna.push(c);
            }
        }
        
        self.dna = new_dna;
        Ok(())
    }
    
    /// 他の個体との交叉を行う
    pub fn crossover(&self, other: &Individual, crossover_point: usize) -> Result<(Individual, Individual)> {
        ensure!(
            self.dna.len() == other.dna.len(),
            "DNA長が異なるため交叉できません"
        );
        ensure!(
            crossover_point <= self.dna.len(),
            "交叉点が範囲外です: {} > {}",
            crossover_point,
            self.dna.len()
        );
        
        let (self_prefix, self_suffix) = self.dna.split_at(crossover_point);
        let (other_prefix, other_suffix) = other.dna.split_at(crossover_point);
        
        let child1_dna = format!("{}{}", self_prefix, other_suffix);
        let child2_dna = format!("{}{}", other_prefix, self_suffix);
        
        let child1 = Individual::new(0, child1_dna); // IDは後で設定される
        let child2 = Individual::new(0, child2_dna);
        
        Ok((child1, child2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_individual_creation() {
        let individual = Individual::new(1, "101010".to_string());
        assert_eq!(individual.id(), 1);
        assert_eq!(individual.dna(), "101010");
        assert_eq!(individual.fitness(), 0.0);
    }

    #[test]
    fn test_fitness_setting() {
        let mut individual = Individual::new(1, "101010".to_string());
        individual.set_fitness(42.5);
        assert_eq!(individual.fitness(), 42.5);
    }

    #[test]
    fn test_choice() -> Result<()> {
        let individual = Individual::new(1, "101010".to_string());
        
        // ラウンド0: DNA[0] = '1' -> Cooperate
        let choice = individual.choose(&[], 0)?;
        assert_eq!(choice, Choice::Cooperate);
        
        // ラウンド1: DNA[1] = '0' -> Defect
        let choice = individual.choose(&[], 1)?;
        assert_eq!(choice, Choice::Defect);
        
        // ラウンド6: DNA[6 % 6] = DNA[0] = '1' -> Cooperate
        let choice = individual.choose(&[], 6)?;
        assert_eq!(choice, Choice::Cooperate);

        Ok(())
    }

    #[test]
    fn test_dna_distance() -> Result<()> {
        let individual1 = Individual::new(1, "101010".to_string());
        let individual2 = Individual::new(2, "111000".to_string());
        
        let distance = individual1.dna_distance(&individual2)?;
        assert_eq!(distance, 3); // 3箇所で異なる
        
        // 同じDNAの場合
        let individual3 = Individual::new(3, "101010".to_string());
        let distance = individual1.dna_distance(&individual3)?;
        assert_eq!(distance, 0);

        Ok(())
    }

    #[test]
    fn test_dna_distance_different_length() {
        let individual1 = Individual::new(1, "1010".to_string());
        let individual2 = Individual::new(2, "101010".to_string());
        
        let result = individual1.dna_distance(&individual2);
        assert!(result.is_err()); // 異なる長さの場合はエラー
    }

    #[test]
    fn test_crossover() -> Result<()> {
        let parent1 = Individual::new(1, "111111".to_string());
        let parent2 = Individual::new(2, "000000".to_string());
        
        let (child1, child2) = parent1.crossover(&parent2, 3)?;
        
        assert_eq!(child1.dna(), "111000");
        assert_eq!(child2.dna(), "000111");

        Ok(())
    }

    #[test]
    fn test_dna_to_strategy_mapping() -> Result<()> {
        // DNAビットパターンから戦略を選択するテスト
        // 00: AlwaysDefect
        // 01: AlwaysCooperate  
        // 10: TitForTat
        // 11: Pavlov
        let individual = Individual::new(1, "00011011".to_string());
        
        // まだ実装していないのでコンパイルエラーになる
        // let strategy = individual.get_strategy_from_dna(0)?; // DNA[0:1] = "00"
        // assert_eq!(strategy.name(), "always-defect");
        
        Ok(())
    }
}