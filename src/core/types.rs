/// Core type definitions for the genetic algorithm
pub type AgentId = u64;
pub type Points = u64;
pub type Dna = String;
pub type Generation = usize;
pub type Population = usize;
pub type MutationRate = f64;
pub type CrossoverPoint = usize;
pub type Fitness = u64;

/// Configuration constants
pub const DEFAULT_POPULATION: Population = 20;
pub const DEFAULT_GENERATIONS: Generation = 50_000;
pub const DEFAULT_MUTATION_RATE: MutationRate = 0.01;
pub const DEFAULT_DNA_LENGTH: usize = 6;
pub const DEFAULT_REPORT_INTERVAL: usize = 5_000;
pub const DEFAULT_ELITE_SIZE: usize = 2;

/// Game-specific constants
pub const COOPERATE_COOPERATE_REWARD: Points = 3;
pub const DEFECT_COOPERATE_REWARD: Points = 5;
pub const COOPERATE_DEFECT_REWARD: Points = 0;
pub const DEFECT_DEFECT_REWARD: Points = 1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_sizes() {
        assert_eq!(std::mem::size_of::<AgentId>(), 8);
        assert_eq!(std::mem::size_of::<Points>(), 8);
        assert_eq!(
            std::mem::size_of::<Generation>(),
            std::mem::size_of::<usize>()
        );
    }

    #[test]
    fn test_constants() {
        assert!(DEFAULT_MUTATION_RATE > 0.0 && DEFAULT_MUTATION_RATE < 1.0);
        assert!(DEFAULT_POPULATION > 0);
        assert!(DEFAULT_GENERATIONS > 0);
        assert!(DEFAULT_ELITE_SIZE < DEFAULT_POPULATION);
    }
}
