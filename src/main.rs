use ga_prisoners_dilemma::ga::ga::{create_next_generation, GAOperation};
use ga_prisoners_dilemma::models::game::{new_game, GameOperation};
use ga_prisoners_dilemma::models::model::Agent;
use ga_prisoners_dilemma::strategies::utils::RouletteSelectionStrategy;

fn main() {
    const GENERATIONS: usize = 50000;
    const POPULATION: usize = 20;
    const MUTATION_RATE: f64 = 0.01;
    const ROUNDS_PER_GENERATION: usize = 1;
    const DNA_LENGTH: usize = 6;
    const REPORT_INTERVAL: usize = 5000;
    
    let mut game = new_game::<Agent, RouletteSelectionStrategy>(
        POPULATION,
        MUTATION_RATE,
        ROUNDS_PER_GENERATION,
        DNA_LENGTH,
        RouletteSelectionStrategy {},
    );
    
    println!("Genetic Algorithm - Prisoner's Dilemma");
    println!("======================================");
    println!("Population: {}", POPULATION);
    println!("Generations: {}", GENERATIONS);
    println!("Mutation rate: {}", MUTATION_RATE);
    println!("DNA length: {}", DNA_LENGTH);
    println!("\nInitial population:");
    
    for (i, dna) in game.get_dna_list().iter().enumerate() {
        println!("Agent {:2}: {}", i, dna);
    }
    println!();
    
    for generation in 0..GENERATIONS {
        let ga_result = game.run_generation();
        
        if generation % REPORT_INTERVAL == 0 {
            println!("\nGeneration {}", generation);
            println!("{}", "-".repeat(40));
            
            let dna_list = ga_result.get_dna_list();
            let points_list = ga_result.get_points_list();
            
            for i in 0..POPULATION {
                println!("Agent {:2}: {} (points: {})", 
                    i, dna_list[i], points_list[i]);
            }
            
            let avg_points: f64 = points_list.iter().sum::<u64>() as f64 / POPULATION as f64;
            println!("Average points: {:.2}", avg_points);
        }
        
        game = create_next_generation(ga_result, RouletteSelectionStrategy {});
    }

    println!("\n\nFinal Results (Generation {})", GENERATIONS);
    println!("{}", "=".repeat(40));
    
    for (i, dna) in game.get_dna_list().iter().enumerate() {
        println!("Agent {:2}: {}", i, dna);
    }
    
    let final_points = game.get_points_list();
    let avg_points: f64 = final_points.iter().sum::<u64>() as f64 / POPULATION as f64;
    println!("\nFinal average points: {:.2}", avg_points);
}
