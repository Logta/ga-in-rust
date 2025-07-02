use crate::core::traits::*;

/// Placeholder for future genetic algorithm engine
#[derive(Debug)]
pub struct GeneticAlgorithmEngine<T: Agent> {
    _population: crate::engine::genetic::Population<T>,
}

impl<T: Agent> GeneticAlgorithmEngine<T> {
    pub fn new(population: crate::engine::genetic::Population<T>) -> Self {
        Self {
            _population: population,
        }
    }
}
