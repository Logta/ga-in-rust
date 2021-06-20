use crate::models::model::{BaseModel};
use rand::{thread_rng, Rng};

use crate::models::model::{Agent};
use crate::models::game::{Game};
use crate::models::game;

pub trait GAOperation {
    fn get_new_game(&self) -> Game;
}

// トレイトを実装するためだけのデータ型にはUnit構造体が便利
pub struct GA{
    pub old_agents: Vec<Agent>,
    pub mutation_rate: f64,
    pub population: u64,
}

// `impl トレイト名 for 型名 {..}`で定義可能
impl GAOperation for GA {
    fn get_new_game(&self) -> Game{
        game::new_game(10, 0.1, 50,6)
    }
}

fn choose_model_parent(agents: &Vec<Agent>, poplation: u64) -> (Agent, Agent){
    let sum_point = agents.iter().fold(0, |sum, a| sum + a.get_point());

    let ch_ag1 = choose_model(agents, poplation, sum_point);
    let ch_ag2 = choose_model(agents, poplation, sum_point);

    (ch_ag1, ch_ag2)
}

fn choose_model(agents: &Vec<Agent>, poplation: u64, sum_point: u64) -> Agent {

    let mut rng = thread_rng();
    let mut rand_num1: u64 = rng.gen_range(0..sum_point);

    for p in 0..poplation{
        rand_num1 -= agents[p as usize].get_point();
        if p < 0 {
            return agents[rand_num1 as usize].clone();
        }
    }
    
    return agents[0].clone();
}