/// Genetic algorithm implementations
pub mod genetic;

/// Selection strategies
pub mod selection;

/// Crossover operations
pub mod crossover;

// Re-export commonly used items
pub use genetic::{GeneticAlgorithmEngine, Population};
pub use selection::{RankSelection, RouletteSelection, TournamentSelection};
