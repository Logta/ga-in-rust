use super::model::{BaseModel, Model, Points};
use crate::ga::ga::GA;
use crate::strategies::utils::StrategyOperation;
use rand::{thread_rng, Rng};

pub trait GameOperation<T, U>
where
    T: Model,
    U: StrategyOperation<T>,
{
    fn get_points_list(&self) -> Vec<Points>;
    fn get_dna_list(&self) -> Vec<String>;
    fn get_mutation_rate(&self) -> f64;
    fn get_population(&self) -> usize;
    fn get_dna_length(&self) -> usize;
    fn run_generation(&mut self) -> Result<GA<T>, String>;
    fn play_round(&mut self);
}

pub struct Game<T: BaseModel, U: StrategyOperation<T>> {
    agents: Vec<Box<T>>,
    mutation_rate: f64,
    population: usize,
    dna_length: usize,
    rounds_per_generation: usize,
    strategy: U,
}

impl<T, U> GameOperation<T, U> for Game<T, U>
where
    T: Model,
    U: StrategyOperation<T>,
{
    fn get_points_list(&self) -> Vec<Points> {
        self.agents.iter().map(|agent| agent.get_points()).collect()
    }

    fn get_dna_list(&self) -> Vec<String> {
        self.agents
            .iter()
            .map(|agent| agent.get_dna().to_string())
            .collect()
    }

    fn get_mutation_rate(&self) -> f64 {
        self.mutation_rate
    }

    fn get_population(&self) -> usize {
        self.population
    }

    fn get_dna_length(&self) -> usize {
        self.dna_length
    }

    fn run_generation(&mut self) -> Result<GA<T>, String> {
        if self.agents.is_empty() {
            return Err("Cannot run generation with empty population".to_string());
        }

        for _ in 0..self.rounds_per_generation {
            self.play_round();
        }

        Ok(GA {
            old_agents: self.agents.clone(),
            mutation_rate: self.mutation_rate,
            population: self.population,
            num_games: self.rounds_per_generation,
            dna_length: self.dna_length,
        })
    }

    fn play_round(&mut self) {
        for i in 0..self.agents.len() {
            for j in (i + 1)..self.agents.len() {
                let (updated_i, updated_j) =
                    self.strategy.play_match(&*self.agents[i], &*self.agents[j]);
                self.agents[i] = Box::new(updated_i);
                self.agents[j] = Box::new(updated_j);
            }
        }
    }
}

pub fn new_game<T, U>(
    population: usize,
    mutation_rate: f64,
    rounds_per_generation: usize,
    dna_length: usize,
    strategy: U,
) -> Game<T, U>
where
    T: BaseModel,
    U: StrategyOperation<T>,
{
    let agents = (0..population)
        .map(|i| Box::new(T::new(i as u64, generate_random_dna(dna_length))))
        .collect();

    Game {
        population,
        mutation_rate,
        agents,
        dna_length,
        rounds_per_generation,
        strategy,
    }
}

pub fn generate_next_game<T, U>(
    population: usize,
    mutation_rate: f64,
    rounds_per_generation: usize,
    dna_length: usize,
    agents: Vec<Box<T>>,
    strategy: U,
) -> Game<T, U>
where
    T: BaseModel,
    U: StrategyOperation<T>,
{
    Game {
        population,
        mutation_rate,
        agents,
        dna_length,
        rounds_per_generation,
        strategy,
    }
}

fn generate_random_dna(length: usize) -> String {
    let mut rng = thread_rng();
    (0..length)
        .map(|_| if rng.gen_bool(0.5) { '1' } else { '0' })
        .collect()
}

#[test]
fn game_creation_test() {
    use crate::models::model::Agent;
    use crate::strategies::utils::RouletteSelectionStrategy;

    let game =
        new_game::<Agent, RouletteSelectionStrategy>(10, 0.1, 6, 6, RouletteSelectionStrategy {});

    assert_eq!(game.get_population(), 10);
    for dna in game.get_dna_list() {
        assert_eq!(dna.len(), 6);
    }
}

#[test]
fn play_round_test() {
    use crate::models::model::Agent;
    use crate::strategies::utils::RouletteSelectionStrategy;

    let agents: Vec<Box<Agent>> = vec![
        Box::new(Agent {
            id: 1,
            points: 0,
            dna: "11111111".to_string(),
            active: true,
        }),
        Box::new(Agent {
            id: 2,
            points: 0,
            dna: "11111111".to_string(),
            active: true,
        }),
        Box::new(Agent {
            id: 3,
            points: 0,
            dna: "11111111".to_string(),
            active: true,
        }),
    ];

    let mut game = Game::<Agent, RouletteSelectionStrategy> {
        population: 3,
        mutation_rate: 0.1,
        agents,
        dna_length: 8,
        rounds_per_generation: 1,
        strategy: RouletteSelectionStrategy {},
    };

    game.play_round();

    assert_eq!(game.agents[0].get_points(), 6);
    assert_eq!(game.agents[1].get_points(), 6);
    assert_eq!(game.agents[2].get_points(), 6);
}
