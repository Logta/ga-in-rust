/// 遺伝的アルゴリズムの実装モジュール
///
/// 個体、個体群、遺伝的操作を定義

pub mod individual;
pub mod population;
pub mod operations;
pub mod algorithm;

pub use individual::Individual;
pub use population::Population;
pub use operations::{select_individuals, crossover, mutate_population, apply_elitism};
pub use algorithm::{GeneticAlgorithm, GAStatistics};