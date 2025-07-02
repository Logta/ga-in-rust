/// ルーレット選択戦略の実装
/// 
/// ルーレット選択は、各個体の適応度に比例した確率で個体を選択する手法です。
/// 適応度が高い個体ほど選択される確率が高くなりますが、適応度の低い個体にも
/// 選択される可能性を残すことで、多様性を保持します。

use crate::core::{errors::*, traits::*, types::*};
use rand::{thread_rng, Rng};

/// ルーレット選択戦略の実装構造体
/// 
/// ルーレット選択では、各個体の適応度に応じた選択確率を計算し、
/// ランダムに個体を選択します。適応度の計算方法を選択可能です。
/// 
/// # フィールド
/// * `use_squared_fitness` - 適応度を二乗するかどうか（選択圧の調整）
#[derive(Debug, Clone)]
pub struct RouletteSelection {
    /// 適応度を二乗して計算するかどうか
    /// 
    /// trueの場合、適応度の差がより顕著になり選択圧が高まります。
    /// falseの場合、線形な適応度を使用し選択圧が穏やかになります。
    use_squared_fitness: bool,
}

impl RouletteSelection {
    /// デフォルトのルーレット選択を作成
    /// 
    /// デフォルトでは二乗適応度を使用し、選択圧を高めに設定します。
    /// 
    /// # 戻り値
    /// 新しいRouletteSelectionインスタンス
    pub fn new() -> Self {
        Self {
            use_squared_fitness: true,
        }
    }

    /// 線形適応度を使用するルーレット選択を作成
    /// 
    /// 適応度を二乗せず、元の値をそのまま使用します。
    /// これにより、選択圧が穏やかになり多様性が保たれやすくなります。
    /// 
    /// # 戻り値
    /// 線形適応度を使用するRouletteSelectionインスタンス
    pub fn with_linear_fitness() -> Self {
        Self {
            use_squared_fitness: false,
        }
    }

    /// 適応度を計算
    /// 
    /// 設定に応じて、ポイントをそのまま使用するか二乗して使用するかを決定します。
    /// 
    /// # 引数
    /// * `points` - 個体の獲得ポイント
    /// 
    /// # 戻り値
    /// 計算された適応度値
    fn calculate_fitness(&self, points: Points) -> Fitness {
        if self.use_squared_fitness {
            // 適応度を二乗することで選択圧を高める
            points * points
        } else {
            // 線形な適応度を使用
            points
        }
    }

    fn select_single<T: Agent>(&self, population: &[T]) -> GAResult<T> {
        if population.is_empty() {
            return Err(GAError::EmptyPopulation);
        }

        let total_fitness: Fitness = population
            .iter()
            .map(|agent| self.calculate_fitness(agent.points()))
            .sum();

        if total_fitness == 0 {
            // If all fitness is 0, select randomly
            let mut rng = thread_rng();
            let index = rng.gen_range(0..population.len());
            return Ok(population[index].clone());
        }

        let mut rng = thread_rng();
        let mut selection_point = rng.gen_range(0..total_fitness) as i64;

        for agent in population {
            let fitness = self.calculate_fitness(agent.points());
            selection_point -= fitness as i64;
            if selection_point <= 0 {
                return Ok(agent.clone());
            }
        }

        // Fallback to first agent (should never reach here)
        Ok(population[0].clone())
    }
}

impl Default for RouletteSelection {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Agent> SelectionStrategy<T> for RouletteSelection {
    fn select_parents(&self, population: &[T]) -> (T, T) {
        let parent1 = self
            .select_single(population)
            .unwrap_or_else(|_| population[0].clone());
        let parent2 = self
            .select_single(population)
            .unwrap_or_else(|_| population[0].clone());
        (parent1, parent2)
    }

    fn select_survivors(&self, population: &[T], count: usize) -> Vec<T> {
        if count >= population.len() {
            return population.to_vec();
        }

        let mut survivors = Vec::with_capacity(count);
        for _ in 0..count {
            if let Ok(survivor) = self.select_single(population) {
                survivors.push(survivor);
            }
        }

        // Fill remaining slots if needed
        while survivors.len() < count && !population.is_empty() {
            survivors.push(population[0].clone());
        }

        survivors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestAgent {
        id: AgentId,
        points: Points,
        dna: String,
    }

    impl BaseEntity for TestAgent {
        fn id(&self) -> AgentId {
            self.id
        }
    }

    impl GeneticOperations for TestAgent {
        fn crossover(&self, _other: &Self, _point: CrossoverPoint) -> Self {
            self.clone()
        }

        fn mutate(&self, _rate: MutationRate) -> Self {
            self.clone()
        }

        fn fitness(&self) -> Fitness {
            self.points
        }
    }

    impl DnaOperations for TestAgent {
        fn dna(&self) -> &Dna {
            &self.dna
        }

        fn dna_length(&self) -> usize {
            self.dna.len()
        }

        fn dna_sum(&self) -> u64 {
            self.dna.chars().filter(|&c| c == '1').count() as u64
        }

        fn dna_binary(&self) -> &str {
            &self.dna
        }
    }

    impl Agent for TestAgent {
        fn points(&self) -> Points {
            self.points
        }

        fn with_points(&self, points: Points) -> Self {
            Self {
                points,
                ..self.clone()
            }
        }

        fn is_active(&self) -> bool {
            true
        }

        fn activate(&mut self) {}

        fn deactivate(&mut self) {}
    }

    #[test]
    fn test_roulette_selection_creation() {
        let selection = RouletteSelection::new();
        assert!(selection.use_squared_fitness);

        let selection = RouletteSelection::with_linear_fitness();
        assert!(!selection.use_squared_fitness);
    }

    #[test]
    fn test_select_parents() {
        let population = vec![
            TestAgent {
                id: 1,
                points: 10,
                dna: "101010".to_string(),
            },
            TestAgent {
                id: 2,
                points: 20,
                dna: "111000".to_string(),
            },
            TestAgent {
                id: 3,
                points: 5,
                dna: "000111".to_string(),
            },
        ];

        let selection = RouletteSelection::new();
        let (parent1, parent2) = selection.select_parents(&population);

        assert!(population.iter().any(|a| a.id == parent1.id));
        assert!(population.iter().any(|a| a.id == parent2.id));
    }

    #[test]
    fn test_select_survivors() {
        let population = vec![
            TestAgent {
                id: 1,
                points: 10,
                dna: "101010".to_string(),
            },
            TestAgent {
                id: 2,
                points: 20,
                dna: "111000".to_string(),
            },
            TestAgent {
                id: 3,
                points: 5,
                dna: "000111".to_string(),
            },
        ];

        let selection = RouletteSelection::new();
        let survivors = selection.select_survivors(&population, 2);

        assert_eq!(survivors.len(), 2);
        for survivor in &survivors {
            assert!(population.iter().any(|a| a.id == survivor.id));
        }
    }

    #[test]
    fn test_empty_population() {
        let population: Vec<TestAgent> = vec![];
        let selection = RouletteSelection::new();

        let result = selection.select_single(&population);
        assert!(matches!(result, Err(GAError::EmptyPopulation)));
    }

    #[test]
    fn test_zero_fitness_population() {
        let population = vec![
            TestAgent {
                id: 1,
                points: 0,
                dna: "101010".to_string(),
            },
            TestAgent {
                id: 2,
                points: 0,
                dna: "111000".to_string(),
            },
        ];

        let selection = RouletteSelection::new();
        let result = selection.select_single(&population);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fitness_calculation() {
        let selection_squared = RouletteSelection::new();
        let selection_linear = RouletteSelection::with_linear_fitness();

        assert_eq!(selection_squared.calculate_fitness(5), 25);
        assert_eq!(selection_linear.calculate_fitness(5), 5);
    }
}
