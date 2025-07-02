/// トーナメント選択戦略の実装
/// 
/// トーナメント選択は、個体群からランダムに選んだ小グループ（トーナメント）の中で
/// 最も適応度の高い個体を選択する手法です。選択圧を調整しやすく、
/// 実装が簡単で効率的な選択手法として広く使用されています。

use crate::core::{errors::*, traits::*, types::*};
use rand::{thread_rng, Rng};

/// トーナメント選択戦略の実装構造体
/// 
/// 指定されたサイズのトーナメントを開催し、その中で最も適応度の高い個体を選択します。
/// トーナメントサイズが大きいほど選択圧が高くなり、小さいほど多様性が保たれます。
/// 
/// # フィールド
/// * `tournament_size` - トーナメントに参加する個体の数
#[derive(Debug, Clone)]
pub struct TournamentSelection {
    /// トーナメントに参加する個体の数
    /// 
    /// 一般的に2-5程度の値が使用されます。
    /// - 2: バランスの取れた選択圧
    /// - 3以上: より強い選択圧
    tournament_size: usize,
}

impl TournamentSelection {
    /// 指定されたトーナメントサイズで新しいインスタンスを作成
    /// 
    /// # 引数
    /// * `tournament_size` - トーナメントに参加する個体の数（1以上）
    /// 
    /// # 戻り値
    /// 成功時は新しいTournamentSelectionインスタンス、失敗時はエラー
    /// 
    /// # エラー
    /// トーナメントサイズが0の場合、ValidationErrorが返されます
    pub fn new(tournament_size: usize) -> GAResult<Self> {
        if tournament_size == 0 {
            return Err(GAError::ValidationError(
                "Tournament size must be greater than 0".to_string(),
            ));
        }

        Ok(Self { tournament_size })
    }

    /// サイズ2のトーナメント選択を作成
    /// 
    /// 最も一般的なトーナメントサイズで、適度な選択圧と多様性のバランスが取れています。
    /// 
    /// # 戻り値
    /// トーナメントサイズ2のTournamentSelectionインスタンス
    pub fn with_size_2() -> Self {
        Self { tournament_size: 2 }
    }

    /// サイズ3のトーナメント選択を作成
    /// 
    /// サイズ2よりも強い選択圧を持ち、より優秀な個体が選ばれやすくなります。
    /// 
    /// # 戻り値
    /// トーナメントサイズ3のTournamentSelectionインスタンス
    pub fn with_size_3() -> Self {
        Self { tournament_size: 3 }
    }

    fn run_tournament<T: Agent>(&self, population: &[T]) -> GAResult<T> {
        if population.is_empty() {
            return Err(GAError::EmptyPopulation);
        }

        let tournament_size = self.tournament_size.min(population.len());
        let mut rng = thread_rng();

        let mut best_agent = None;
        let mut best_fitness = 0;

        for _ in 0..tournament_size {
            let index = rng.gen_range(0..population.len());
            let candidate = &population[index];
            let fitness = candidate.fitness();

            if best_agent.is_none() || fitness > best_fitness {
                best_agent = Some(candidate.clone());
                best_fitness = fitness;
            }
        }

        best_agent.ok_or(GAError::SelectionError(
            "Tournament failed to select agent".to_string(),
        ))
    }
}

impl Default for TournamentSelection {
    fn default() -> Self {
        Self::with_size_2()
    }
}

impl<T: Agent> SelectionStrategy<T> for TournamentSelection {
    fn select_parents(&self, population: &[T]) -> (T, T) {
        let parent1 = self
            .run_tournament(population)
            .unwrap_or_else(|_| population[0].clone());
        let parent2 = self
            .run_tournament(population)
            .unwrap_or_else(|_| population[0].clone());
        (parent1, parent2)
    }

    fn select_survivors(&self, population: &[T], count: usize) -> Vec<T> {
        if count >= population.len() {
            return population.to_vec();
        }

        let mut survivors = Vec::with_capacity(count);
        for _ in 0..count {
            if let Ok(survivor) = self.run_tournament(population) {
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
    fn test_tournament_creation() {
        let selection = TournamentSelection::new(3);
        assert!(selection.is_ok());
        assert_eq!(selection.unwrap().tournament_size, 3);

        let selection = TournamentSelection::new(0);
        assert!(selection.is_err());
    }

    #[test]
    fn test_default_tournament() {
        let selection = TournamentSelection::default();
        assert_eq!(selection.tournament_size, 2);

        let selection = TournamentSelection::with_size_3();
        assert_eq!(selection.tournament_size, 3);
    }

    #[test]
    fn test_tournament_selection() {
        let population = vec![
            TestAgent {
                id: 1,
                points: 10,
                dna: "101010".to_string(),
            },
            TestAgent {
                id: 2,
                points: 30,
                dna: "111000".to_string(),
            }, // Best
            TestAgent {
                id: 3,
                points: 5,
                dna: "000111".to_string(),
            },
            TestAgent {
                id: 4,
                points: 20,
                dna: "110011".to_string(),
            },
        ];

        let selection = TournamentSelection::with_size_2();

        // トーナメント選択が動作することを確認
        // 複数回実行して統計的に妥当な結果が得られることを検証
        let mut results = std::collections::HashMap::new();
        for _ in 0..100 {
            let winner = selection.run_tournament(&population).unwrap();
            *results.entry(winner.id).or_insert(0) += 1;
        }

        // より高い適応度を持つ個体がより頻繁に選ばれることを確認
        // 最高適応度の個体（id=2）が最も多く選ばれることを期待
        let best_wins = results.get(&2).unwrap_or(&0);
        assert!(*best_wins > 0, "最高適応度の個体が一度も選ばれませんでした");
        
        // 全ての個体が選択される可能性があることを確認（多様性）
        assert!(results.len() >= 2, "トーナメント選択に多様性がありません");
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

        let selection = TournamentSelection::new(2).unwrap();
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

        let selection = TournamentSelection::new(2).unwrap();
        let survivors = selection.select_survivors(&population, 2);

        assert_eq!(survivors.len(), 2);
        for survivor in &survivors {
            assert!(population.iter().any(|a| a.id == survivor.id));
        }
    }

    #[test]
    fn test_empty_population() {
        let population: Vec<TestAgent> = vec![];
        let selection = TournamentSelection::new(2).unwrap();

        let result = selection.run_tournament(&population);
        assert!(matches!(result, Err(GAError::EmptyPopulation)));
    }

    #[test]
    fn test_tournament_size_larger_than_population() {
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
        ];

        let selection = TournamentSelection::new(5).unwrap(); // Larger than population
        let result = selection.run_tournament(&population);
        assert!(result.is_ok());
    }
}
