use crate::models::model;
use crate::strategies::utils::{StrategyOperation, Strategy};
use crate::ga::ga::{GA};

use super::model::{Agent, BaseModel, Model};
use rand::{thread_rng, Rng};

pub trait GameOperation {
    fn get_point_list(&self) -> Vec<u64>;
    fn get_dna_list(&self) -> Vec<String>;
    fn get_mutation_rate(&self) -> f64;
    fn get_population(&self) -> u64;
    fn get_dna_length(&self) -> u64;
    fn do_game(&mut self) -> GA;
    fn one_shot_game(&mut self);
}

#[derive (Clone)]
pub struct Game{
    agents: Vec<Agent>,
    mutation_rate: f64,
    population: u64,
    dna_length: u64,
    num_game: u64, // １世代でのゲーム回数
    strategy: Strategy,
}

impl GameOperation for Game {
    fn get_point_list(&self) -> Vec<u64>{
        self.agents.iter().map(|x| x.get_point()).collect()
    }

    fn get_dna_list(&self) -> Vec<String>{
        self.agents.iter().map(|x| x.get_dna_2_binary_digits()).collect()
    }

    fn get_mutation_rate(&self) -> f64{
        self.mutation_rate
    }

    fn get_population(&self) -> u64{
        self.population
    }

    fn get_dna_length(&self) -> u64{
        self.dna_length
    }
    
    fn do_game(&mut self) -> GA{
        for _ in 0..self.num_game {
            self.one_shot_game();
        }

        let g = self.clone();
        
        GA{
            old_agents: g.agents,
            mutation_rate: g.mutation_rate,
            population: g.population,
        }
    }

    fn one_shot_game(&mut self){
        for proponent in 0..self.agents.len(){
            for opponent in proponent..self.agents.len(){
                let (pro, opp) = Strategy::get_result(self.agents[proponent].clone(), self.agents[opponent].clone());
                self.agents[proponent]=pro;
                self.agents[opponent]=opp;
            }
        }
    }
}

pub fn new_game(population: u64, mutation_rate: f64,num_game:u64, dna_length: u64) -> Game {
    let agents = (0..population).map(|x| model::new_base_model(x,get_dna(dna_length as u32))).collect();

    Game {
        population,
        mutation_rate,
        agents,
        dna_length,
        num_game,
        strategy: Strategy{}
    }
}

fn get_dna(num: u32) -> String {
    let base: u32 = 2; 
    let two_pow:u32 = base.pow(num);

    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(0..two_pow);

    format!("{:0>1$b}", n, num as usize)
}


#[test]
fn game(){
    let g = new_game(10, 0.1, 6, 6);
    for dna in g.get_dna_list().iter(){
        println!("{}", dna);
        assert_eq!(6, dna.len());
    }
}