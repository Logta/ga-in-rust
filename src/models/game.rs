use crate::models::model;

use super::model::{Ajent, BaseModel, Model};
use rand::{thread_rng, Rng};

pub trait GameOperation {
    fn get_point_list(&self) -> Vec<u64>;
    fn get_dna_list(&self) -> Vec<String>;
    fn get_mutation_rate(&self) -> u64;
    fn get_population(&self) -> u64;
}

// トレイトを実装するためだけのデータ型にはUnit構造体が便利
pub struct Game{
    agents: Vec<Ajent>,
    mutation_rate: u64,
    population: u64,
    dna_length: u64,
}

// `impl トレイト名 for 型名 {..}`で定義可能
impl GameOperation for Game {
    fn get_point_list(&self) -> Vec<u64>{
        self.agents.iter().map(|x| x.get_point()).collect()
    }

    fn get_dna_list(&self) -> Vec<String>{
        self.agents.iter().map(|x| x.get_dna_2_binary_digits()).collect()
    }

    fn get_mutation_rate(&self) -> u64{
        self.mutation_rate
    }

    fn get_population(&self) -> u64{
        self.population
    }
}

pub fn new_game(population: u64, mutation_rate: u64, dna_length: u64) -> Game {
    let agents = (0..population).map(|x| model::new_base_model(x,get_dna(dna_length as u32))).collect();

    Game {
        population,
        mutation_rate,
        agents,
        dna_length,
    }
}

fn get_dna(num: u32) -> String {
    let base: u32 = 2; 
    let k:u32 = base.pow(num);
    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(0..k);
    format!("{:08b}", n)
}