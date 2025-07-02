#[cfg(test)]
mod tests {
    use ga_prisoners_dilemma::engine::{Population, RouletteSelection, TournamentSelection, RankSelection};
    use ga_prisoners_dilemma::core::{traits::*, types::*};

    #[derive(Debug, Clone)]
    struct MockAgent {
        id: AgentId,
        points: Points,
        dna: String,
        active: bool,
    }

    impl BaseEntity for MockAgent {
        fn id(&self) -> AgentId {
            self.id
        }
    }

    impl GeneticOperations for MockAgent {
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

        fn mutate(&self, _rate: MutationRate) -> Self {
            self.clone()
        }

        fn fitness(&self) -> Fitness {
            self.points
        }
    }

    impl DnaOperations for MockAgent {
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

    impl Agent for MockAgent {
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
    fn test_population_creation_and_basic_operations() {
        let agents = vec![
            MockAgent { id: 1, points: 10, dna: "101010".to_string(), active: true },
            MockAgent { id: 2, points: 20, dna: "111000".to_string(), active: true },
            MockAgent { id: 3, points: 15, dna: "000111".to_string(), active: true },
        ];

        let population = Population::new(agents, 1).unwrap();
        
        assert_eq!(population.size(), 3);
        assert_eq!(population.generation(), 0);
        assert_eq!(population.elite_size(), 1);
        assert_eq!(population.average_fitness(), 15.0);
        
        let best = population.best_agent().unwrap();
        assert_eq!(best.id, 2);
        
        let worst = population.worst_agent().unwrap();
        assert_eq!(worst.id, 1);
    }

    #[test]
    fn test_random_population_generation() {
        let population = Population::random(5, 6, 1, |id, dna| MockAgent {
            id,
            points: 0,
            dna,
            active: true,
        }).unwrap();

        assert_eq!(population.size(), 5);
        assert_eq!(population.elite_size(), 1);
        
        for agent in population.agents() {
            assert_eq!(agent.dna_length(), 6);
            assert!(agent.dna().chars().all(|c| c == '0' || c == '1'));
        }
    }

    #[test]
    fn test_selection_strategies() {
        let population = vec![
            MockAgent { id: 1, points: 10, dna: "101010".to_string(), active: true },
            MockAgent { id: 2, points: 30, dna: "111000".to_string(), active: true },
            MockAgent { id: 3, points: 20, dna: "000111".to_string(), active: true },
        ];

        // Test RouletteSelection
        let roulette = RouletteSelection::new();
        let (p1, p2) = roulette.select_parents(&population);
        assert!(population.iter().any(|a| a.id == p1.id));
        assert!(population.iter().any(|a| a.id == p2.id));

        // Test TournamentSelection
        let tournament = TournamentSelection::new(2).unwrap();
        let (p1, p2) = tournament.select_parents(&population);
        assert!(population.iter().any(|a| a.id == p1.id));
        assert!(population.iter().any(|a| a.id == p2.id));

        // Test RankSelection
        let rank = RankSelection::linear();
        let (p1, p2) = rank.select_parents(&population);
        assert!(population.iter().any(|a| a.id == p1.id));
        assert!(population.iter().any(|a| a.id == p2.id));
    }

    #[test]
    fn test_elite_preservation() {
        let agents = vec![
            MockAgent { id: 1, points: 10, dna: "101010".to_string(), active: true },
            MockAgent { id: 2, points: 30, dna: "111000".to_string(), active: true },
            MockAgent { id: 3, points: 20, dna: "000111".to_string(), active: true },
        ];

        let population = Population::new(agents, 2).unwrap();
        let elite = population.get_elite();
        
        assert_eq!(elite.len(), 2);
        assert_eq!(elite[0].id, 2); // Best agent
        assert_eq!(elite[1].id, 3); // Second best
    }

    #[test]
    fn test_diversity_metric() {
        let agents = vec![
            MockAgent { id: 1, points: 10, dna: "000000".to_string(), active: true },
            MockAgent { id: 2, points: 20, dna: "111111".to_string(), active: true },
            MockAgent { id: 3, points: 15, dna: "101010".to_string(), active: true },
        ];

        let population = Population::new(agents, 1).unwrap();
        let diversity = population.diversity_metric();
        
        assert!(diversity > 0.0);
        assert!(diversity <= 1.0);
    }

    #[test]
    fn test_fitness_statistics() {
        let agents = vec![
            MockAgent { id: 1, points: 10, dna: "101010".to_string(), active: true },
            MockAgent { id: 2, points: 20, dna: "111000".to_string(), active: true },
            MockAgent { id: 3, points: 30, dna: "000111".to_string(), active: true },
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
}