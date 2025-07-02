use crate::core::types::*;

/// Base trait for all genetic algorithm entities
pub trait BaseEntity: Clone + Send + Sync {
    fn id(&self) -> AgentId;
}

/// Trait for genetic operations on agents
pub trait GeneticOperations: BaseEntity {
    fn crossover(&self, other: &Self, point: CrossoverPoint) -> Self;
    fn mutate(&self, rate: MutationRate) -> Self;
    fn fitness(&self) -> Fitness;
}

/// Trait for DNA operations
pub trait DnaOperations {
    fn dna(&self) -> &Dna;
    fn dna_length(&self) -> usize;
    fn dna_sum(&self) -> u64;
    fn dna_binary(&self) -> &str;
}

/// Trait for agent behavior in games
pub trait Agent: GeneticOperations + DnaOperations {
    fn points(&self) -> Points;
    fn with_points(&self, points: Points) -> Self;
    fn is_active(&self) -> bool;
    fn activate(&mut self);
    fn deactivate(&mut self);
}

/// Trait for selection strategies
pub trait SelectionStrategy<T: Agent> {
    fn select_parents(&self, population: &[T]) -> (T, T);
    fn select_survivors(&self, population: &[T], count: usize) -> Vec<T>;
}

/// Trait for game strategies
pub trait GameStrategy<T: Agent> {
    fn play_match(&self, agent1: &T, agent2: &T) -> (T, T);
}

/// Trait for genetic algorithm operations
pub trait GeneticAlgorithm<T: Agent> {
    fn population(&self) -> &[T];
    fn generation(&self) -> Generation;
    fn evolve(&mut self) -> Result<(), crate::core::errors::GAError>;
    fn best_agent(&self) -> Option<&T>;
    fn average_fitness(&self) -> f64;
}

/// Trait for statistical operations
pub trait Statistics {
    fn mean(&self) -> f64;
    fn max(&self) -> Option<Points>;
    fn min(&self) -> Option<Points>;
    fn std_deviation(&self) -> f64;
}

impl Statistics for Vec<Points> {
    fn mean(&self) -> f64 {
        if self.is_empty() {
            0.0
        } else {
            self.iter().sum::<u64>() as f64 / self.len() as f64
        }
    }

    fn max(&self) -> Option<Points> {
        self.iter().max().copied()
    }

    fn min(&self) -> Option<Points> {
        self.iter().min().copied()
    }

    fn std_deviation(&self) -> f64 {
        if self.len() <= 1 {
            return 0.0;
        }

        let mean = self.mean();
        let variance = self
            .iter()
            .map(|&x| {
                let diff = x as f64 - mean;
                diff * diff
            })
            .sum::<f64>()
            / (self.len() - 1) as f64;

        variance.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statistics_mean() {
        let data = vec![1, 2, 3, 4, 5];
        assert_eq!(data.mean(), 3.0);

        let empty: Vec<Points> = vec![];
        assert_eq!(empty.mean(), 0.0);
    }

    #[test]
    fn test_statistics_max_min() {
        let data = vec![1, 5, 3, 2, 4];
        assert_eq!(data.iter().max(), Some(&5));
        assert_eq!(data.iter().min(), Some(&1));

        let empty: Vec<Points> = vec![];
        assert_eq!(empty.iter().max(), None);
        assert_eq!(empty.iter().min(), None);
    }

    #[test]
    fn test_statistics_std_deviation() {
        let data = vec![2, 4, 4, 4, 5, 5, 7, 9];
        let std_dev = data.std_deviation();
        assert!((std_dev - 2.138).abs() < 0.01);

        let single = vec![5];
        assert_eq!(single.std_deviation(), 0.0);
    }
}
