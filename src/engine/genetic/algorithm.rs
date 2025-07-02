use crate::core::traits::*;

/// Placeholder for future genetic algorithm engine
#[derive(Debug)]
pub struct GeneticAlgorithmEngine<T: Agent> {
    population: crate::engine::genetic::Population<T>,
}

impl<T: Agent> GeneticAlgorithmEngine<T> {
    pub fn new(population: crate::engine::genetic::Population<T>) -> Self {
        Self { population }
    }
}
