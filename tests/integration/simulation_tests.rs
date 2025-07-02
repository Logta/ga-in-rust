#[cfg(test)]
mod tests {
    use ga_prisoners_dilemma::infrastructure::config::{Config, ConfigBuilder};
    use ga_prisoners_dilemma::domain::simulation::Simulation;

    #[test]
    fn test_minimal_simulation() {
        let config = ConfigBuilder::new()
            .generations(10)
            .population(5)
            .mutation_rate(0.1)
            .dna_length(4)
            .report_interval(5)
            .build()
            .unwrap();

        let simulation = Simulation::new(config).unwrap();
        let result = simulation.run();
        
        assert!(result.is_ok());
        let sim_result = result.unwrap();
        
        assert_eq!(sim_result.config.generations, 10);
        assert_eq!(sim_result.config.population, 5);
        assert!(!sim_result.generation_results.is_empty());
    }

    #[test]
    fn test_config_validation() {
        // Test invalid population size
        let config_result = ConfigBuilder::new()
            .population(0)
            .build();
        assert!(config_result.is_err());

        // Test invalid mutation rate
        let config_result = ConfigBuilder::new()
            .mutation_rate(1.5)
            .build();
        assert!(config_result.is_err());

        // Test invalid elite size
        let config_result = ConfigBuilder::new()
            .population(5)
            .elite_size(10)
            .build();
        assert!(config_result.is_err());
    }

    #[test]
    fn test_simulation_progress() {
        let config = ConfigBuilder::new()
            .generations(20)
            .population(8)
            .report_interval(10)
            .build()
            .unwrap();

        let simulation = Simulation::new(config).unwrap();
        let result = simulation.run().unwrap();
        
        // Should have reports at generation 0 and 10
        assert!(result.generation_results.len() >= 2);
        
        // Check that generations are recorded correctly
        assert_eq!(result.generation_results[0].generation, 0);
        if result.generation_results.len() > 1 {
            assert_eq!(result.generation_results[1].generation, 10);
        }
    }

    #[test]
    fn test_different_configurations() {
        let configs = vec![
            ConfigBuilder::new().population(10).build().unwrap(),
            ConfigBuilder::new().population(20).mutation_rate(0.05).build().unwrap(),
            ConfigBuilder::new().dna_length(8).build().unwrap(),
        ];

        for config in configs {
            let simulation = Simulation::new(config.clone()).unwrap();
            let result = simulation.run();
            assert!(result.is_ok(), "Failed with config: {:?}", config);
        }
    }

    #[test]
    fn test_simulation_statistics() {
        let config = ConfigBuilder::new()
            .generations(5)
            .population(6)
            .report_interval(1)
            .build()
            .unwrap();

        let simulation = Simulation::new(config).unwrap();
        let result = simulation.run().unwrap();
        
        // Check that statistics are reasonable
        for gen_result in &result.generation_results {
            assert_eq!(gen_result.dna_list.len(), 6);
            assert_eq!(gen_result.points_list.len(), 6);
            assert!(gen_result.avg_points >= 0.0);
            assert!(gen_result.max_points >= gen_result.min_points);
        }
        
        // Final result should be consistent
        assert_eq!(result.final_result.dna_list.len(), 6);
        assert_eq!(result.final_result.points_list.len(), 6);
        assert!(result.final_result.avg_points >= 0.0);
    }
}