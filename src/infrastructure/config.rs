use crate::core::types::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Config {
    pub generations: usize,
    pub population: usize,
    pub mutation_rate: f64,
    pub rounds_per_generation: usize,
    pub dna_length: usize,
    pub report_interval: usize,
    pub elite_size: usize,
}

impl Config {
    pub fn new() -> Self {
        Self {
            generations: DEFAULT_GENERATIONS,
            population: DEFAULT_POPULATION,
            mutation_rate: DEFAULT_MUTATION_RATE,
            rounds_per_generation: 1,
            dna_length: DEFAULT_DNA_LENGTH,
            report_interval: DEFAULT_REPORT_INTERVAL,
            elite_size: DEFAULT_ELITE_SIZE,
        }
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.population == 0 {
            return Err(ConfigError::InvalidPopulation);
        }
        if self.mutation_rate < 0.0 || self.mutation_rate > 1.0 {
            return Err(ConfigError::InvalidMutationRate);
        }
        if self.dna_length == 0 {
            return Err(ConfigError::InvalidDnaLength);
        }
        if self.elite_size >= self.population {
            return Err(ConfigError::InvalidEliteSize);
        }
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum ConfigError {
    InvalidPopulation,
    InvalidMutationRate,
    InvalidDnaLength,
    InvalidEliteSize,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InvalidPopulation => write!(f, "Population must be greater than 0"),
            ConfigError::InvalidMutationRate => {
                write!(f, "Mutation rate must be between 0.0 and 1.0")
            }
            ConfigError::InvalidDnaLength => write!(f, "DNA length must be greater than 0"),
            ConfigError::InvalidEliteSize => {
                write!(f, "Elite size must be less than population size")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: Config::new(),
        }
    }

    pub fn generations(mut self, generations: usize) -> Self {
        self.config.generations = generations;
        self
    }

    pub fn population(mut self, population: usize) -> Self {
        self.config.population = population;
        self
    }

    pub fn mutation_rate(mut self, rate: f64) -> Self {
        self.config.mutation_rate = rate;
        self
    }

    pub fn dna_length(mut self, length: usize) -> Self {
        self.config.dna_length = length;
        self
    }

    pub fn report_interval(mut self, interval: usize) -> Self {
        self.config.report_interval = interval;
        self
    }

    pub fn elite_size(mut self, size: usize) -> Self {
        self.config.elite_size = size;
        self
    }

    pub fn build(self) -> Result<Config, ConfigError> {
        self.config.validate()?;
        Ok(self.config)
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
