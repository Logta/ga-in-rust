use crate::core::errors::{GAError, GAResult};
use crate::ga::ga::{create_next_generation, GAOperation};
use crate::infrastructure::config::Config;
use crate::models::game::{new_game, GameOperation};
use crate::models::model::Agent;
use crate::strategies::utils::RouletteSelectionStrategy;

pub struct Simulation {
    config: Config,
}

impl Simulation {
    pub fn new(config: Config) -> GAResult<Self> {
        config.validate()?;
        Ok(Self { config })
    }

    pub fn run(&self) -> GAResult<SimulationResult> {
        let mut game = new_game::<Agent, RouletteSelectionStrategy>(
            self.config.population,
            self.config.mutation_rate,
            self.config.rounds_per_generation,
            self.config.dna_length,
            RouletteSelectionStrategy {},
        );

        self.print_header(&game);

        let mut results = Vec::new();

        for generation in 0..self.config.generations {
            let ga_result = game
                .run_generation()
                .map_err(|_| GAError::GameExecutionError("Failed to run generation".to_string()))?;

            if generation % self.config.report_interval == 0 {
                let generation_stats = self.collect_generation_stats(generation, &ga_result);
                self.print_generation_report(&generation_stats);
                results.push(generation_stats);
            }

            game = create_next_generation(ga_result, RouletteSelectionStrategy {});
        }

        let final_stats = self.collect_final_stats(&game)?;
        self.print_final_report(&final_stats);

        Ok(SimulationResult {
            config: self.config.clone(),
            generation_results: results,
            final_result: final_stats,
        })
    }

    fn print_header<T, U>(&self, game: &T)
    where
        T: GameOperation<Agent, U>,
        U: crate::strategies::utils::StrategyOperation<Agent>,
    {
        println!("Genetic Algorithm - Prisoner's Dilemma");
        println!("======================================");
        println!("Population: {}", self.config.population);
        println!("Generations: {}", self.config.generations);
        println!("Mutation rate: {}", self.config.mutation_rate);
        println!("DNA length: {}", self.config.dna_length);
        println!("\nInitial population:");

        for (i, dna) in game.get_dna_list().iter().enumerate() {
            println!("Agent {:2}: {}", i, dna);
        }
        println!();
    }

    fn collect_generation_stats<T>(&self, generation: usize, ga_result: &T) -> GenerationStats
    where
        T: GAOperation<Agent>,
    {
        let dna_list = ga_result.get_dna_list();
        let points_list = ga_result.get_points_list();
        let avg_points = points_list.iter().sum::<u64>() as f64 / self.config.population as f64;
        let max_points = *points_list.iter().max().unwrap_or(&0);
        let min_points = *points_list.iter().min().unwrap_or(&0);

        GenerationStats {
            generation,
            dna_list,
            points_list,
            avg_points,
            max_points,
            min_points,
        }
    }

    fn collect_final_stats<T, U>(&self, game: &T) -> GAResult<FinalStats>
    where
        T: GameOperation<Agent, U>,
        U: crate::strategies::utils::StrategyOperation<Agent>,
    {
        let dna_list = game.get_dna_list();
        let points_list = game.get_points_list();
        let avg_points = points_list.iter().sum::<u64>() as f64 / self.config.population as f64;

        Ok(FinalStats {
            dna_list,
            points_list,
            avg_points,
        })
    }

    fn print_generation_report(&self, stats: &GenerationStats) {
        println!("\nGeneration {}", stats.generation);
        println!("{}", "-".repeat(40));

        for i in 0..self.config.population {
            println!(
                "Agent {:2}: {} (points: {})",
                i, stats.dna_list[i], stats.points_list[i]
            );
        }

        println!("Average points: {:.2}", stats.avg_points);
        println!("Max points: {}", stats.max_points);
        println!("Min points: {}", stats.min_points);
    }

    fn print_final_report(&self, stats: &FinalStats) {
        println!("\n\nFinal Results (Generation {})", self.config.generations);
        println!("{}", "=".repeat(40));

        for (i, dna) in stats.dna_list.iter().enumerate() {
            println!("Agent {:2}: {} (points: {})", i, dna, stats.points_list[i]);
        }

        println!("\nFinal average points: {:.2}", stats.avg_points);
    }
}

#[derive(Debug, Clone)]
pub struct GenerationStats {
    pub generation: usize,
    pub dna_list: Vec<String>,
    pub points_list: Vec<u64>,
    pub avg_points: f64,
    pub max_points: u64,
    pub min_points: u64,
}

#[derive(Debug, Clone)]
pub struct FinalStats {
    pub dna_list: Vec<String>,
    pub points_list: Vec<u64>,
    pub avg_points: f64,
}

#[derive(Debug, Clone)]
pub struct SimulationResult {
    pub config: Config,
    pub generation_results: Vec<GenerationStats>,
    pub final_result: FinalStats,
}
