use crate::core::{errors::*, traits::*, types::*};
use rand::{thread_rng, Rng};

/// Rank-based selection strategy
#[derive(Debug, Clone)]
pub struct RankSelection {
    pressure: f64, // Selection pressure (1.0 = no pressure, 2.0 = max pressure)
}

impl RankSelection {
    pub fn new(pressure: f64) -> GAResult<Self> {
        if pressure < 1.0 || pressure > 2.0 {
            return Err(GAError::ValidationError(
                "Selection pressure must be between 1.0 and 2.0".to_string(),
            ));
        }

        Ok(Self { pressure })
    }

    pub fn linear() -> Self {
        Self { pressure: 1.5 }
    }

    pub fn uniform() -> Self {
        Self { pressure: 1.0 }
    }

    pub fn high_pressure() -> Self {
        Self { pressure: 2.0 }
    }

    fn calculate_rank_probabilities(&self, population_size: usize) -> Vec<f64> {
        let mut probabilities = Vec::with_capacity(population_size);
        let n = population_size as f64;

        for rank in 0..population_size {
            let r = (rank + 1) as f64; // Rank 1-based
            let prob = (2.0 - self.pressure + 2.0 * (self.pressure - 1.0) * (r - 1.0) / (n - 1.0)) / n;
            probabilities.push(prob);
        }

        probabilities
    }

    fn select_by_rank<T: Agent>(&self, sorted_population: &[T]) -> GAResult<T> {
        if sorted_population.is_empty() {
            return Err(GAError::EmptyPopulation);
        }

        let probabilities = self.calculate_rank_probabilities(sorted_population.len());
        let mut rng = thread_rng();
        let mut cumulative_prob = 0.0;
        let random_value: f64 = rng.gen();

        for (i, &prob) in probabilities.iter().enumerate() {
            cumulative_prob += prob;
            if random_value <= cumulative_prob {
                return Ok(sorted_population[i].clone());
            }
        }

        // Fallback to last (best) agent
        Ok(sorted_population[sorted_population.len() - 1].clone())
    }

    fn sort_population_by_fitness<T: Agent>(&self, population: &[T]) -> Vec<T> {
        let mut sorted = population.to_vec();
        sorted.sort_by_key(|agent| agent.fitness());
        sorted
    }
}

impl Default for RankSelection {
    fn default() -> Self {
        Self::linear()
    }
}

impl<T: Agent> SelectionStrategy<T> for RankSelection {
    fn select_parents(&self, population: &[T]) -> (T, T) {
        let sorted_population = self.sort_population_by_fitness(population);

        let parent1 = self
            .select_by_rank(&sorted_population)
            .unwrap_or_else(|_| population[0].clone());
        let parent2 = self
            .select_by_rank(&sorted_population)
            .unwrap_or_else(|_| population[0].clone());

        (parent1, parent2)
    }

    fn select_survivors(&self, population: &[T], count: usize) -> Vec<T> {
        if count >= population.len() {
            return population.to_vec();
        }

        let sorted_population = self.sort_population_by_fitness(population);
        let mut survivors = Vec::with_capacity(count);

        for _ in 0..count {
            if let Ok(survivor) = self.select_by_rank(&sorted_population) {
                survivors.push(survivor);
            }
        }

        // Fill remaining slots if needed
        while survivors.len() < count && !population.is_empty() {
            survivors.push(sorted_population[sorted_population.len() - 1].clone());
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
    fn test_rank_selection_creation() {
        let selection = RankSelection::new(1.5);
        assert!(selection.is_ok());
        assert_eq!(selection.unwrap().pressure, 1.5);

        let selection = RankSelection::new(0.5);
        assert!(selection.is_err());

        let selection = RankSelection::new(2.5);
        assert!(selection.is_err());
    }

    #[test]
    fn test_predefined_selections() {
        let linear = RankSelection::linear();
        assert_eq!(linear.pressure, 1.5);

        let uniform = RankSelection::uniform();
        assert_eq!(uniform.pressure, 1.0);

        let high = RankSelection::high_pressure();
        assert_eq!(high.pressure, 2.0);
    }

    #[test]
    fn test_rank_probabilities() {
        let selection = RankSelection::linear();
        let probs = selection.calculate_rank_probabilities(4);

        assert_eq!(probs.len(), 4);

        // Check that probabilities sum to approximately 1.0
        let sum: f64 = probs.iter().sum();
        assert!((sum - 1.0).abs() < 0.001);

        // Higher ranks should have higher probabilities
        assert!(probs[3] > probs[0]); // Best > Worst
    }

    #[test]
    fn test_population_sorting() {
        let population = vec![
            TestAgent {
                id: 1,
                points: 30,
                dna: "101010".to_string(),
            },
            TestAgent {
                id: 2,
                points: 10,
                dna: "111000".to_string(),
            },
            TestAgent {
                id: 3,
                points: 20,
                dna: "000111".to_string(),
            },
        ];

        let selection = RankSelection::linear();
        let sorted = selection.sort_population_by_fitness(&population);

        assert_eq!(sorted[0].points, 10); // Lowest first
        assert_eq!(sorted[1].points, 20);
        assert_eq!(sorted[2].points, 30); // Highest last
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

        let selection = RankSelection::linear();
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

        let selection = RankSelection::linear();
        let survivors = selection.select_survivors(&population, 2);

        assert_eq!(survivors.len(), 2);
        for survivor in &survivors {
            assert!(population.iter().any(|a| a.id == survivor.id));
        }
    }

    #[test]
    fn test_empty_population() {
        let population: Vec<TestAgent> = vec![];
        let selection = RankSelection::linear();

        let result = selection.select_by_rank(&population);
        assert!(matches!(result, Err(GAError::EmptyPopulation)));
    }

    #[test]
    fn test_uniform_selection() {
        let selection = RankSelection::uniform();
        let probs = selection.calculate_rank_probabilities(4);

        // With pressure = 1.0, all probabilities should be equal
        let expected_prob = 1.0 / 4.0;
        for prob in probs {
            assert!((prob - expected_prob).abs() < 0.001);
        }
    }
}
