use crate::core::{errors::*, traits::*, types::*};
use rand::{thread_rng, Rng};

/// Population management for genetic algorithms
#[derive(Debug, Clone)]
pub struct Population<T: Agent> {
    agents: Vec<T>,
    generation: Generation,
    elite_size: usize,
}

impl<T: Agent> Population<T> {
    pub fn new(agents: Vec<T>, elite_size: usize) -> GAResult<Self> {
        if agents.is_empty() {
            return Err(GAError::EmptyPopulation);
        }

        crate::core::errors::validation::validate_elite_size(elite_size, agents.len())?;

        Ok(Self {
            agents,
            generation: 0,
            elite_size,
        })
    }

    pub fn random<F>(
        size: usize,
        dna_length: usize,
        elite_size: usize,
        agent_factory: F,
    ) -> GAResult<Self>
    where
        F: Fn(AgentId, Dna) -> T,
    {
        crate::core::errors::validation::validate_population_size(size)?;
        crate::core::errors::validation::validate_elite_size(elite_size, size)?;

        let mut agents = Vec::with_capacity(size);
        let mut rng = thread_rng();

        for id in 0..size {
            let dna = Self::generate_random_dna(dna_length, &mut rng);
            agents.push(agent_factory(id as AgentId, dna));
        }

        Ok(Self {
            agents,
            generation: 0,
            elite_size,
        })
    }

    fn generate_random_dna<R: Rng>(length: usize, rng: &mut R) -> Dna {
        (0..length)
            .map(|_| if rng.gen_bool(0.5) { '1' } else { '0' })
            .collect()
    }

    pub fn agents(&self) -> &[T] {
        &self.agents
    }

    pub fn agents_mut(&mut self) -> &mut [T] {
        &mut self.agents
    }

    pub fn size(&self) -> usize {
        self.agents.len()
    }

    pub fn generation(&self) -> Generation {
        self.generation
    }

    pub fn increment_generation(&mut self) {
        self.generation += 1;
    }

    pub fn elite_size(&self) -> usize {
        self.elite_size
    }

    pub fn set_elite_size(&mut self, size: usize) -> GAResult<()> {
        crate::core::errors::validation::validate_elite_size(size, self.size())?;
        self.elite_size = size;
        Ok(())
    }

    pub fn best_agent(&self) -> Option<&T> {
        self.agents.iter().max_by_key(|agent| agent.fitness())
    }

    pub fn worst_agent(&self) -> Option<&T> {
        self.agents.iter().min_by_key(|agent| agent.fitness())
    }

    pub fn average_fitness(&self) -> f64 {
        if self.agents.is_empty() {
            0.0
        } else {
            let total: Fitness = self.agents.iter().map(|agent| agent.fitness()).sum();
            total as f64 / self.agents.len() as f64
        }
    }

    pub fn fitness_statistics(&self) -> GAResult<PopulationStats> {
        if self.agents.is_empty() {
            return Err(GAError::EmptyPopulation);
        }

        let fitness_values: Vec<Points> = self.agents.iter().map(|agent| agent.points()).collect();

        Ok(PopulationStats {
            generation: self.generation,
            size: self.size(),
            mean_fitness: fitness_values.mean(),
            max_fitness: fitness_values.iter().max().copied().unwrap_or(0),
            min_fitness: fitness_values.iter().min().copied().unwrap_or(0),
            std_deviation: fitness_values.std_deviation(),
            elite_size: self.elite_size,
        })
    }

    pub fn get_elite(&self) -> Vec<T> {
        let mut sorted_agents = self.agents.clone();
        sorted_agents.sort_by_key(|agent| std::cmp::Reverse(agent.fitness()));
        sorted_agents.into_iter().take(self.elite_size).collect()
    }

    pub fn replace_agents(&mut self, new_agents: Vec<T>) -> GAResult<()> {
        if new_agents.len() != self.agents.len() {
            return Err(GAError::InvalidPopulationSize(new_agents.len()));
        }

        self.agents = new_agents;
        Ok(())
    }

    pub fn apply_elitism(&mut self, offspring: &mut Vec<T>) -> GAResult<()> {
        if offspring.len() < self.elite_size {
            return Err(GAError::ValidationError(
                "Offspring size is smaller than elite size".to_string(),
            ));
        }

        let elite = self.get_elite();

        // Sort offspring by fitness (worst first)
        offspring.sort_by_key(|agent| agent.fitness());

        // Replace worst offspring with elite
        for (i, elite_agent) in elite.into_iter().enumerate() {
            offspring[i] = elite_agent;
        }

        Ok(())
    }

    pub fn diversity_metric(&self) -> f64 {
        if self.agents.len() < 2 {
            return 0.0;
        }

        let mut total_distance = 0.0;
        let mut comparisons = 0;

        for i in 0..self.agents.len() {
            for j in (i + 1)..self.agents.len() {
                let distance = self.hamming_distance(&self.agents[i], &self.agents[j]);
                total_distance += distance;
                comparisons += 1;
            }
        }

        if comparisons > 0 {
            total_distance / comparisons as f64
        } else {
            0.0
        }
    }

    fn hamming_distance(&self, agent1: &T, agent2: &T) -> f64 {
        let dna1 = agent1.dna_binary();
        let dna2 = agent2.dna_binary();

        if dna1.len() != dna2.len() {
            return 0.0;
        }

        let differences = dna1
            .chars()
            .zip(dna2.chars())
            .filter(|(a, b)| a != b)
            .count();

        differences as f64 / dna1.len() as f64
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PopulationStats {
    pub generation: Generation,
    pub size: usize,
    pub mean_fitness: f64,
    pub max_fitness: Points,
    pub min_fitness: Points,
    pub std_deviation: f64,
    pub elite_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestAgent {
        id: AgentId,
        points: Points,
        dna: String,
        active: bool,
    }

    impl BaseEntity for TestAgent {
        fn id(&self) -> AgentId {
            self.id
        }
    }

    impl GeneticOperations for TestAgent {
        fn crossover(&self, other: &Self, point: CrossoverPoint) -> Self {
            let dna1 = self.dna();
            let dna2 = other.dna();
            let cut_point = point.min(dna1.len());

            let new_dna = format!("{}{}", &dna1[..cut_point], &dna2[cut_point..]);

            Self {
                id: self.id,
                points: 0,
                dna: new_dna,
                active: true,
            }
        }

        fn mutate(&self, rate: MutationRate) -> Self {
            let mut rng = thread_rng();
            let new_dna: String = self
                .dna
                .chars()
                .map(|c| {
                    if rng.gen::<f64>() < rate {
                        if c == '0' {
                            '1'
                        } else {
                            '0'
                        }
                    } else {
                        c
                    }
                })
                .collect();

            Self {
                id: self.id,
                points: self.points,
                dna: new_dna,
                active: self.active,
            }
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
            self.active
        }

        fn activate(&mut self) {
            self.active = true;
        }

        fn deactivate(&mut self) {
            self.active = false;
        }
    }

    #[test]
    fn test_population_creation() {
        let agents = vec![
            TestAgent {
                id: 1,
                points: 10,
                dna: "101010".to_string(),
                active: true,
            },
            TestAgent {
                id: 2,
                points: 20,
                dna: "111000".to_string(),
                active: true,
            },
        ];

        let population = Population::new(agents, 1);
        assert!(population.is_ok());

        let pop = population.unwrap();
        assert_eq!(pop.size(), 2);
        assert_eq!(pop.generation(), 0);
        assert_eq!(pop.elite_size(), 1);
    }

    #[test]
    fn test_empty_population_error() {
        let agents: Vec<TestAgent> = vec![];
        let result = Population::new(agents, 0);
        assert!(matches!(result, Err(GAError::EmptyPopulation)));
    }

    #[test]
    fn test_invalid_elite_size() {
        let agents = vec![TestAgent {
            id: 1,
            points: 10,
            dna: "101010".to_string(),
            active: true,
        }];

        let result = Population::new(agents, 2); // Elite size > population size
        assert!(matches!(result, Err(GAError::InvalidEliteSize(_))));
    }

    #[test]
    fn test_random_population() {
        let population = Population::random(5, 6, 1, |id, dna| TestAgent {
            id,
            points: 0,
            dna,
            active: true,
        });

        assert!(population.is_ok());
        let pop = population.unwrap();
        assert_eq!(pop.size(), 5);

        for agent in pop.agents() {
            assert_eq!(agent.dna_length(), 6);
            assert!(agent.dna().chars().all(|c| c == '0' || c == '1'));
        }
    }

    #[test]
    fn test_best_worst_agents() {
        let agents = vec![
            TestAgent {
                id: 1,
                points: 10,
                dna: "101010".to_string(),
                active: true,
            },
            TestAgent {
                id: 2,
                points: 30,
                dna: "111000".to_string(),
                active: true,
            },
            TestAgent {
                id: 3,
                points: 5,
                dna: "000111".to_string(),
                active: true,
            },
        ];

        let population = Population::new(agents, 1).unwrap();

        assert_eq!(population.best_agent().unwrap().id, 2);
        assert_eq!(population.worst_agent().unwrap().id, 3);
    }

    #[test]
    fn test_average_fitness() {
        let agents = vec![
            TestAgent {
                id: 1,
                points: 10,
                dna: "101010".to_string(),
                active: true,
            },
            TestAgent {
                id: 2,
                points: 20,
                dna: "111000".to_string(),
                active: true,
            },
            TestAgent {
                id: 3,
                points: 30,
                dna: "000111".to_string(),
                active: true,
            },
        ];

        let population = Population::new(agents, 1).unwrap();
        assert_eq!(population.average_fitness(), 20.0);
    }

    #[test]
    fn test_fitness_statistics() {
        let agents = vec![
            TestAgent {
                id: 1,
                points: 10,
                dna: "101010".to_string(),
                active: true,
            },
            TestAgent {
                id: 2,
                points: 20,
                dna: "111000".to_string(),
                active: true,
            },
            TestAgent {
                id: 3,
                points: 30,
                dna: "000111".to_string(),
                active: true,
            },
        ];

        let population = Population::new(agents, 1).unwrap();
        let stats = population.fitness_statistics().unwrap();

        assert_eq!(stats.generation, 0);
        assert_eq!(stats.size, 3);
        assert_eq!(stats.mean_fitness, 20.0);
        assert_eq!(stats.max_fitness, 30);
        assert_eq!(stats.min_fitness, 10);
        assert!(stats.std_deviation > 0.0);
    }

    #[test]
    fn test_get_elite() {
        let agents = vec![
            TestAgent {
                id: 1,
                points: 10,
                dna: "101010".to_string(),
                active: true,
            },
            TestAgent {
                id: 2,
                points: 30,
                dna: "111000".to_string(),
                active: true,
            },
            TestAgent {
                id: 3,
                points: 20,
                dna: "000111".to_string(),
                active: true,
            },
        ];

        let population = Population::new(agents, 2).unwrap();
        let elite = population.get_elite();

        assert_eq!(elite.len(), 2);
        assert_eq!(elite[0].id, 2); // Best agent
        assert_eq!(elite[1].id, 3); // Second best
    }

    #[test]
    fn test_increment_generation() {
        let agents = vec![TestAgent {
            id: 1,
            points: 10,
            dna: "101010".to_string(),
            active: true,
        }];

        let mut population = Population::new(agents, 0).unwrap();
        assert_eq!(population.generation(), 0);

        population.increment_generation();
        assert_eq!(population.generation(), 1);
    }

    #[test]
    fn test_diversity_metric() {
        let agents = vec![
            TestAgent {
                id: 1,
                points: 10,
                dna: "000000".to_string(),
                active: true,
            },
            TestAgent {
                id: 2,
                points: 20,
                dna: "111111".to_string(),
                active: true,
            },
            TestAgent {
                id: 3,
                points: 30,
                dna: "101010".to_string(),
                active: true,
            },
        ];

        let population = Population::new(agents, 1).unwrap();
        let diversity = population.diversity_metric();

        // Should be > 0 since agents have different DNA
        assert!(diversity > 0.0);
        assert!(diversity <= 1.0);
    }

    #[test]
    fn test_hamming_distance() {
        let agent1 = TestAgent {
            id: 1,
            points: 10,
            dna: "000000".to_string(),
            active: true,
        };
        let agent2 = TestAgent {
            id: 2,
            points: 20,
            dna: "111111".to_string(),
            active: true,
        };
        let agent3 = TestAgent {
            id: 3,
            points: 30,
            dna: "000000".to_string(),
            active: true,
        };

        let population = Population::new(vec![agent1.clone()], 0).unwrap();

        // Completely different DNA
        assert_eq!(population.hamming_distance(&agent1, &agent2), 1.0);

        // Identical DNA
        assert_eq!(population.hamming_distance(&agent1, &agent3), 0.0);
    }
}
