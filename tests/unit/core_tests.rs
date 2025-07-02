#[cfg(test)]
mod tests {
    use ga_prisoners_dilemma::core::{errors::*, types::*, traits::*};

    #[test]
    fn test_error_types() {
        let error = GAError::EmptyPopulation;
        assert_eq!(error.to_string(), "Population cannot be empty");

        let error = GAError::InvalidPopulationSize(0);
        assert!(error.to_string().contains("Invalid population size: 0"));
    }

    #[test]
    fn test_type_constants() {
        assert_eq!(DEFAULT_POPULATION, 20);
        assert_eq!(DEFAULT_GENERATIONS, 50_000);
        assert!(DEFAULT_MUTATION_RATE > 0.0 && DEFAULT_MUTATION_RATE < 1.0);
    }

    #[test]
    fn test_statistics_trait() {
        let data: Vec<Points> = vec![1, 2, 3, 4, 5];
        assert_eq!(data.mean(), 3.0);
        assert_eq!(data.max(), Some(5));
        assert_eq!(data.min(), Some(1));
        assert!(data.std_deviation() > 0.0);
    }

    #[test]
    fn test_validation_functions() {
        use ga_prisoners_dilemma::core::errors::validation::*;

        // Population size validation
        assert!(validate_population_size(10).is_ok());
        assert!(validate_population_size(0).is_err());

        // Mutation rate validation
        assert!(validate_mutation_rate(0.5).is_ok());
        assert!(validate_mutation_rate(-0.1).is_err());
        assert!(validate_mutation_rate(1.1).is_err());

        // DNA validation
        assert!(validate_dna("101010").is_ok());
        assert!(validate_dna("102030").is_err());
        assert!(validate_dna("").is_err());

        // Elite size validation
        assert!(validate_elite_size(2, 10).is_ok());
        assert!(validate_elite_size(10, 10).is_err());

        // Crossover point validation
        assert!(validate_crossover_point(3, 6).is_ok());
        assert!(validate_crossover_point(6, 6).is_err());
    }

    #[test]
    fn test_game_constants() {
        assert_eq!(COOPERATE_COOPERATE_REWARD, 3);
        assert_eq!(DEFECT_COOPERATE_REWARD, 5);
        assert_eq!(COOPERATE_DEFECT_REWARD, 0);
        assert_eq!(DEFECT_DEFECT_REWARD, 1);
    }
}