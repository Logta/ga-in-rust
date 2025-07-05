/// 個体の実装
use anyhow::{Result, ensure};
use crate::simulation::environment::Choice;
use crate::strategies::{Strategy, BasicStrategy};

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
    /// history: 相手の選択の履歴
    pub fn choose(&self, history: &[Choice], round: usize) -> Result<Choice> {
        ensure!(!self.dna.is_empty(), "DNAが空です");
        ensure!(self.dna.len() >= 4, "DNAは最低4ビット必要です");
        
        // DNAの最初の4ビットから戦略を選択
        let strategy = self.get_strategy_from_dna(0)?;
        
        // 履歴を(自分の選択, 相手の選択)のタプル形式に変換
        // 簡略化のため、相手の履歴のみから推測
        let history_pairs: Vec<(Choice, Choice)> = history.iter()
            .map(|&opponent_choice| {
                // 戦略の履歴形式に合わせるため、ダミーの自分の選択を生成
                // 実際のゲームでは両方の履歴が保持される
                (Choice::Cooperate, opponent_choice)
            })
            .collect();
        
        // 戦略に基づいて選択を決定
        Ok(strategy.decide(&history_pairs, round))
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
    
    /// DNAの指定位置から戦略を取得
    pub fn get_strategy_from_dna(&self, position: usize) -> Result<Box<dyn Strategy>> {
        ensure!(position + 3 < self.dna.len(), "DNAの位置が範囲外です");
        
        let bits = &self.dna[position..position + 4];
        let strategy: Box<dyn Strategy> = match bits {
            "0000" => Box::new(BasicStrategy::AlwaysDefect),
            "0001" => Box::new(BasicStrategy::AlwaysCooperate),
            "0010" => Box::new(BasicStrategy::TitForTat),
            "0011" => Box::new(BasicStrategy::Pavlov),
            "0100" => Box::new(BasicStrategy::GeneralizedReciprocity),
            "0101" => Box::new(BasicStrategy::Random),
            "0110" => Box::new(BasicStrategy::ThirdPartyInfluence),
            // 予備（0111-1111は将来の拡張用、今はデフォルト戦略を使用）
            "0111" => Box::new(BasicStrategy::AlwaysCooperate), // デフォルト
            "1000" => Box::new(BasicStrategy::TitForTat), // デフォルト
            "1001" => Box::new(BasicStrategy::Pavlov), // デフォルト
            "1010" => Box::new(BasicStrategy::Random), // デフォルト
            "1011" => Box::new(BasicStrategy::GeneralizedReciprocity), // デフォルト
            "1100" => Box::new(BasicStrategy::ThirdPartyInfluence), // デフォルト
            "1101" => Box::new(BasicStrategy::AlwaysCooperate), // デフォルト
            "1110" => Box::new(BasicStrategy::TitForTat), // デフォルト
            "1111" => Box::new(BasicStrategy::AlwaysDefect), // デフォルト
            _ => anyhow::bail!("無効なDNAパターン: {}", bits),
        };
        
        Ok(strategy)
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
        // 新しい仕様: 最初の4ビットが戦略を決定
        // "0010..." -> TitForTat戦略
        let individual = Individual::new(1, "00101010".to_string());
        
        // TitForTat: 初回は協力
        let choice = individual.choose(&[], 0)?;
        assert_eq!(choice, Choice::Cooperate);
        
        // TitForTat: 相手が裏切ったので裏切る
        let choice = individual.choose(&[Choice::Defect], 1)?;
        assert_eq!(choice, Choice::Defect);
        
        // TitForTat: 相手が協力したので協力
        let choice = individual.choose(&[Choice::Defect, Choice::Cooperate], 2)?;
        assert_eq!(choice, Choice::Cooperate);

        Ok(())
    }

    #[test]
    fn test_dna_distance() -> Result<()> {
        let individual1 = Individual::new(1, "101010".to_string());
        let individual2 = Individual::new(2, "111000".to_string());
        
        let distance = individual1.dna_distance(&individual2)?;
        assert_eq!(distance, 2); // 2箇所で異なる (位置1と位置4)
        
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
        // DNAビットパターンから戦略を選択するテスト（4ビット版）
        // 0000: AlwaysDefect
        // 0001: AlwaysCooperate  
        // 0010: TitForTat
        // 0011: Pavlov
        // 0100: GeneralizedReciprocity
        // 0101: Random
        // 0110: ThirdPartyInfluence
        let individual = Individual::new(1, "000000010010001101000101011000000000".to_string());
        
        let strategy = individual.get_strategy_from_dna(0)?; // DNA[0:3] = "0000"
        assert_eq!(strategy.name(), "always-defect");
        
        let strategy = individual.get_strategy_from_dna(4)?; // DNA[4:7] = "0001"
        assert_eq!(strategy.name(), "always-cooperate");
        
        let strategy = individual.get_strategy_from_dna(8)?; // DNA[8:11] = "0010"
        assert_eq!(strategy.name(), "tit-for-tat");
        
        let strategy = individual.get_strategy_from_dna(12)?; // DNA[12:15] = "0011"
        assert_eq!(strategy.name(), "pavlov");
        
        let strategy = individual.get_strategy_from_dna(16)?; // DNA[16:19] = "0100"
        assert_eq!(strategy.name(), "generalized-reciprocity");
        
        let strategy = individual.get_strategy_from_dna(20)?; // DNA[20:23] = "0101"
        assert_eq!(strategy.name(), "random");
        
        let strategy = individual.get_strategy_from_dna(24)?; // DNA[24:27] = "0110"
        assert_eq!(strategy.name(), "third-party-influence");
        
        Ok(())
    }

    #[test]
    fn test_choice_with_strategy() -> Result<()> {
        // 戦略を使った選択のテスト
        // DNA: "00011010" - 最初の4ビット"0001"はAlwaysCooperate
        let individual = Individual::new(1, "00011010".to_string());
        
        // AlwaysCooperateなので常にCooperateを返すはず
        assert_eq!(individual.choose(&[], 0)?, Choice::Cooperate);
        assert_eq!(individual.choose(&[], 1)?, Choice::Cooperate);
        assert_eq!(individual.choose(&[Choice::Defect], 2)?, Choice::Cooperate);
        
        Ok(())
    }

    #[test]
    fn test_choice_with_different_strategies() -> Result<()> {
        // AlwaysDefect (0000)
        let individual = Individual::new(1, "000010000".to_string());
        assert_eq!(individual.choose(&[], 0)?, Choice::Defect);
        assert_eq!(individual.choose(&[Choice::Cooperate], 1)?, Choice::Defect);
        
        // TitForTat (0010) - 初回は協力、その後は相手の前回の行動を真似る
        let individual = Individual::new(2, "001010000".to_string());
        assert_eq!(individual.choose(&[], 0)?, Choice::Cooperate);
        assert_eq!(individual.choose(&[Choice::Defect], 1)?, Choice::Defect);
        assert_eq!(individual.choose(&[Choice::Defect, Choice::Cooperate], 2)?, Choice::Cooperate);
        
        Ok(())
    }
}