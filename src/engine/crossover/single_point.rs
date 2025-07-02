use crate::core::{errors::*, traits::*, types::*};
use rand::{thread_rng, Rng};

/// Single-point crossover operation
#[derive(Debug, Clone)]
pub struct SinglePointCrossover;

impl SinglePointCrossover {
    pub fn new() -> Self {
        Self
    }

    pub fn crossover<T: Agent>(&self, parent1: &T, parent2: &T) -> GAResult<(T, T)> {
        let dna1 = parent1.dna_binary();
        let dna2 = parent2.dna_binary();

        if dna1.len() != dna2.len() {
            return Err(GAError::InvalidDna(
                "Parent DNA lengths must be equal".to_string(),
            ));
        }

        if dna1.is_empty() {
            return Err(GAError::InvalidDnaLength(0));
        }

        let mut rng = thread_rng();
        let crossover_point = rng.gen_range(1..dna1.len()); // 1 to len-1

        self.crossover_at_point(parent1, parent2, crossover_point)
    }

    pub fn crossover_at_point<T: Agent>(
        &self,
        parent1: &T,
        parent2: &T,
        point: CrossoverPoint,
    ) -> GAResult<(T, T)> {
        let dna1 = parent1.dna_binary();
        let dna2 = parent2.dna_binary();

        if dna1.len() != dna2.len() {
            return Err(GAError::InvalidDna(
                "Parent DNA lengths must be equal".to_string(),
            ));
        }

        crate::core::errors::validation::validate_crossover_point(point, dna1.len())?;

        let offspring1_dna = format!("{}{}", &dna1[..point], &dna2[point..]);
        let offspring2_dna = format!("{}{}", &dna2[..point], &dna1[point..]);

        crate::core::errors::validation::validate_dna(&offspring1_dna)?;
        crate::core::errors::validation::validate_dna(&offspring2_dna)?;

        let offspring1 = parent1.crossover(parent2, point);
        let offspring2 = parent2.crossover(parent1, point);

        Ok((offspring1, offspring2))
    }

    pub fn multiple_crossover<T: Agent>(&self, parents: &[(T, T)]) -> GAResult<Vec<(T, T)>> {
        let mut offspring = Vec::with_capacity(parents.len());

        for (parent1, parent2) in parents {
            let (child1, child2) = self.crossover(parent1, parent2)?;
            offspring.push((child1, child2));
        }

        Ok(offspring)
    }
}

impl Default for SinglePointCrossover {
    fn default() -> Self {
        Self::new()
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
        fn crossover(&self, other: &Self, point: CrossoverPoint) -> Self {
            let dna1 = self.dna();
            let dna2 = other.dna();
            let cut_point = point.min(dna1.len());

            let new_dna = format!("{}{}", &dna1[..cut_point], &dna2[cut_point..]);

            Self {
                id: self.id,
                points: 0,
                dna: new_dna,
            }
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
    fn test_single_point_crossover_creation() {
        let crossover = SinglePointCrossover::new();
        let default_crossover = SinglePointCrossover::default();

        // Both should be equivalent
        assert_eq!(
            std::mem::discriminant(&crossover),
            std::mem::discriminant(&default_crossover)
        );
    }

    #[test]
    fn test_crossover_at_point() {
        let parent1 = TestAgent {
            id: 1,
            points: 10,
            dna: "111000".to_string(),
        };
        let parent2 = TestAgent {
            id: 2,
            points: 20,
            dna: "000111".to_string(),
        };

        let crossover = SinglePointCrossover::new();
        let result = crossover.crossover_at_point(&parent1, &parent2, 3);

        assert!(result.is_ok());
        let (child1, child2) = result.unwrap();

        assert_eq!(child1.dna(), "111111"); // 111 + 111
        assert_eq!(child2.dna(), "000000"); // 000 + 000
    }

    #[test]
    fn test_crossover_random_point() {
        let parent1 = TestAgent {
            id: 1,
            points: 10,
            dna: "111111".to_string(),
        };
        let parent2 = TestAgent {
            id: 2,
            points: 20,
            dna: "000000".to_string(),
        };

        let crossover = SinglePointCrossover::new();
        let result = crossover.crossover(&parent1, &parent2);

        assert!(result.is_ok());
        let (child1, child2) = result.unwrap();

        // Children should have valid DNA
        assert_eq!(child1.dna_length(), 6);
        assert_eq!(child2.dna_length(), 6);

        // Children DNA should contain parts from both parents
        assert!(child1.dna().contains('0') || child1.dna().contains('1'));
        assert!(child2.dna().contains('0') || child2.dna().contains('1'));
    }

    #[test]
    fn test_crossover_different_lengths() {
        let parent1 = TestAgent {
            id: 1,
            points: 10,
            dna: "1111".to_string(),
        };
        let parent2 = TestAgent {
            id: 2,
            points: 20,
            dna: "000000".to_string(),
        };

        let crossover = SinglePointCrossover::new();
        let result = crossover.crossover(&parent1, &parent2);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GAError::InvalidDna(_)));
    }

    #[test]
    fn test_crossover_empty_dna() {
        let parent1 = TestAgent {
            id: 1,
            points: 10,
            dna: "".to_string(),
        };
        let parent2 = TestAgent {
            id: 2,
            points: 20,
            dna: "".to_string(),
        };

        let crossover = SinglePointCrossover::new();
        let result = crossover.crossover(&parent1, &parent2);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GAError::InvalidDnaLength(0)));
    }

    #[test]
    fn test_crossover_invalid_point() {
        let parent1 = TestAgent {
            id: 1,
            points: 10,
            dna: "111000".to_string(),
        };
        let parent2 = TestAgent {
            id: 2,
            points: 20,
            dna: "000111".to_string(),
        };

        let crossover = SinglePointCrossover::new();
        let result = crossover.crossover_at_point(&parent1, &parent2, 10); // Point > DNA length

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            GAError::InvalidCrossoverPoint(_)
        ));
    }

    #[test]
    fn test_multiple_crossover() {
        let parents = vec![
            (
                TestAgent {
                    id: 1,
                    points: 10,
                    dna: "111000".to_string(),
                },
                TestAgent {
                    id: 2,
                    points: 20,
                    dna: "000111".to_string(),
                },
            ),
            (
                TestAgent {
                    id: 3,
                    points: 30,
                    dna: "101010".to_string(),
                },
                TestAgent {
                    id: 4,
                    points: 40,
                    dna: "010101".to_string(),
                },
            ),
        ];

        let crossover = SinglePointCrossover::new();
        let result = crossover.multiple_crossover(&parents);

        assert!(result.is_ok());
        let offspring = result.unwrap();
        assert_eq!(offspring.len(), 2);

        for (child1, child2) in offspring {
            assert_eq!(child1.dna_length(), 6);
            assert_eq!(child2.dna_length(), 6);
        }
    }

    #[test]
    fn test_crossover_preserves_dna_length() {
        let parent1 = TestAgent {
            id: 1,
            points: 10,
            dna: "10101010".to_string(),
        };
        let parent2 = TestAgent {
            id: 2,
            points: 20,
            dna: "01010101".to_string(),
        };

        let crossover = SinglePointCrossover::new();

        for point in 1..parent1.dna_length() {
            let result = crossover.crossover_at_point(&parent1, &parent2, point);
            assert!(result.is_ok());

            let (child1, child2) = result.unwrap();
            assert_eq!(child1.dna_length(), parent1.dna_length());
            assert_eq!(child2.dna_length(), parent2.dna_length());
        }
    }

    #[test]
    fn test_crossover_boundary_points() {
        let parent1 = TestAgent {
            id: 1,
            points: 10,
            dna: "111111".to_string(),
        };
        let parent2 = TestAgent {
            id: 2,
            points: 20,
            dna: "000000".to_string(),
        };

        let crossover = SinglePointCrossover::new();

        // Test at first valid point
        let result = crossover.crossover_at_point(&parent1, &parent2, 1);
        assert!(result.is_ok());

        // Test at last valid point
        let result = crossover.crossover_at_point(&parent1, &parent2, parent1.dna_length() - 1);
        assert!(result.is_ok());
    }
}
