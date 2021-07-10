use super::model::{Agent, BaseModel, Model};
use crate::ga::ga::GA;
use crate::strategies::utils::{RouletteSelectionStrategy, StrategyOperation};
use rand::{thread_rng, Rng};

pub trait GameOperation<T, U>
where
    T: Model,
    U: StrategyOperation<T>,
{
    fn get_point_list(&self) -> Vec<u64>;
    fn get_dna_list(&self) -> Vec<String>;
    fn get_mutation_rate(&self) -> f64;
    fn get_population(&self) -> u64;
    fn get_dna_length(&self) -> u64;
    fn do_game(&mut self) -> GA<T>;
    fn one_shot_game(&mut self);
}

pub struct Game<T: BaseModel, U: StrategyOperation<T>> {
    agents: Vec<Box<T>>,
    mutation_rate: f64,
    population: u64,
    dna_length: u64,
    num_game: u64, // １世代でのゲーム回数
    strategy: U,
}

impl<T, U> GameOperation<T, U> for Game<T, U>
where
    T: Model,
    U: StrategyOperation<T>,
{
    fn get_point_list(&self) -> Vec<u64> {
        self.agents.iter().map(|x| x.get_point()).collect()
    }

    fn get_dna_list(&self) -> Vec<String> {
        self.agents.iter().map(|x| x.get_dna()).collect()
    }

    fn get_mutation_rate(&self) -> f64 {
        self.mutation_rate
    }

    fn get_population(&self) -> u64 {
        self.population
    }

    fn get_dna_length(&self) -> u64 {
        self.dna_length
    }

    fn do_game(&mut self) -> GA<T> {
        for _ in 0..self.num_game {
            self.one_shot_game();
        }

        GA {
            old_agents: self.agents.clone(),
            mutation_rate: self.mutation_rate,
            population: self.population,
            num_game: self.num_game,
            dna_length: self.dna_length,
        }
    }

    fn one_shot_game(&mut self) {
        for proponent in 0..self.agents.len() {
            for opponent in proponent..self.agents.len() {
                if opponent == proponent {
                    continue;
                };
                let (pro, opp) = self.strategy.get_result(
                    *self.agents[proponent].clone(),
                    *self.agents[opponent].clone(),
                );
                self.agents[proponent] = Box::from(pro);
                self.agents[opponent] = Box::from(opp);
            }
        }
    }
}

pub fn new_game<T, U>(
    population: u64,
    mutation_rate: f64,
    num_game: u64,
    dna_length: u64,
    strategy: U,
) -> Game<T, U>
where
    T: BaseModel,
    U: StrategyOperation<T>,
{
    let agents = (0..population)
        .map(|x| Box::from(T::new_base_model(x, get_dna(dna_length as u32))))
        .collect::<Vec<Box<T>>>();
    Game::<T, U> {
        population,
        mutation_rate,
        agents,
        dna_length,
        num_game,
        strategy,
    }
}

pub fn generate_next_game<T, U>(
    population: u64,
    mutation_rate: f64,
    num_game: u64,
    dna_length: u64,
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
        num_game,
        strategy,
    }
}

fn get_dna(num: u32) -> String {
    let base: u32 = 2;
    let two_pow: u32 = base.pow(num);

    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(0..two_pow);

    format!("{:0>1$b}", n, num as usize)
}

#[test]
fn game() {
    let g =
        new_game::<Agent, RouletteSelectionStrategy>(10, 0.1, 6, 6, RouletteSelectionStrategy {});
    for dna in g.get_dna_list().iter() {
        println!("{}", dna);
        assert_eq!(6, dna.len());
    }
}

#[test]
fn one_shot_game_test() {
    let mut agents: Vec<Box<Agent>> = Vec::new();
    let agent1 = Agent {
        id: 1,
        point: 0,
        dna_2_binary_digits: "11111111".to_string(),
        active: true,
    };
    let agent2 = Agent {
        id: 2,
        point: 0,
        dna_2_binary_digits: "11111111".to_string(),
        active: true,
    };
    let agent3 = Agent {
        id: 3,
        point: 0,
        dna_2_binary_digits: "11111111".to_string(),
        active: true,
    };

    agents.push(Box::<Agent>::from(agent1));
    agents.push(Box::<Agent>::from(agent2));
    agents.push(Box::<Agent>::from(agent3));
    let mut g = Game::<Agent, RouletteSelectionStrategy> {
        population: 2,
        mutation_rate: 0.1,
        agents,
        dna_length: 8,
        num_game: 1,
        strategy: RouletteSelectionStrategy {},
    };
    g.one_shot_game();

    assert_eq!(g.agents[0].get_point(), 6);
    assert_eq!(g.agents[1].get_point(), 6);
    assert_eq!(g.agents[2].get_point(), 6);
}
