use crate::models::model::{BaseModel, Model};
use rand::{thread_rng, Rng};

use crate::models::model::{Agent};
use crate::models::model;
use crate::models::game::{Game};
use crate::models::game;

pub trait GAOperation {
    fn get_new_game(&self) -> Game;
    fn get_point_list(&self) -> Vec<u64>;
    fn get_dna_list(&self) -> Vec<String>;
}

// トレイトを実装するためだけのデータ型にはUnit構造体が便利
pub struct GA{
    pub old_agents: Vec<Agent>,
    pub mutation_rate: f64,
    pub population: u64,
    pub dna_length: u64,
    pub num_game: u64, // １世代でのゲーム回数
}

// `impl トレイト名 for 型名 {..}`で定義可能
impl GAOperation for GA {
    fn get_new_game(&self) -> Game{
        let agents = (0..self.population).map(|x| model::new_base_model(x,get_dna(&self.old_agents, self.population, self.mutation_rate))).collect();

        game::generate_next_game(self.population, self.mutation_rate, self.num_game,self.dna_length, agents)
    }

    fn get_point_list(&self) -> Vec<u64>{
        self.old_agents.iter().map(|x| x.get_point()).collect()
    }

    fn get_dna_list(&self) -> Vec<String>{
        self.old_agents.iter().map(|x| x.get_dna_2_binary_digits()).collect()
    }
}

fn get_dna(agents: &Vec<Agent>, poplation: u64, mutation_rate: f64) -> String{
    let (ch_ag1, ch_ag2) = choose_model_parent(agents, poplation);

    let mut rng = thread_rng();
    let cross_point: u64 = rng.gen_range(0..ch_ag1.get_dna_length());

    let ch_ag1 = ch_ag1.crossover(&ch_ag2, cross_point as usize);
    ch_ag1.mutation(mutation_rate).get_dna_2_binary_digits()
}

fn choose_model_parent(agents: &Vec<Agent>, poplation: u64) -> (Agent, Agent){
    let sum_point = agents.iter().fold(0, |sum, a| sum + a.get_point()*a.get_point());

    let ch_ag1 = choose_model_roulettes(agents, poplation, sum_point);
    let ch_ag2 = choose_model_roulettes(agents, poplation, sum_point);
    // let ch_ag1 = choose_model_tournament(agents, get_random_model_indexes(poplation, (poplation / 2) as u16).as_ref());
    // let ch_ag2 = choose_model_tournament(agents, get_random_model_indexes(poplation, (poplation / 2) as u16).as_ref());

    (ch_ag1, ch_ag2)
}

fn choose_model_roulettes(agents: &Vec<Agent>, poplation: u64, sum_point: u64) -> Agent {

    let mut rng = thread_rng();
    let mut rand_num1: i64 = rng.gen_range(0..sum_point) as i64;

    for p in 0..poplation{
        rand_num1 -= (agents[p as usize].get_point() * agents[p as usize].get_point()) as i64;
        if rand_num1 <= 0 {
            return agents[p as usize].clone();
        }
    }
    
    return agents[0].clone();
}

fn get_random_model_indexes(poplation: u64, select_num: u16) -> Vec<u64>{
    let mut indexes = (0..poplation).collect::<Vec<u64>>();
    
    for i in 0..poplation{
        let j = (get_rand() * poplation as f64) as usize;
        indexes.swap(i as usize, j);
    }
    (0..select_num as usize).map(|s| indexes[s]).collect::<Vec<u64>>()
}

fn get_rand() -> f64{
    let mut rng = rand::thread_rng();
    rng.gen()
}

fn choose_model_tournament(agents: &Vec<Agent>, indexes: &[u64]) -> Agent {

    let battle_agents = agents.iter().filter(|a| indexes.contains(&a.id));
    
    let mut max_agent: Agent = Agent{
        id: 0,point:0,dna_2_binary_digits:"".to_string(),
        active:false
    };
    for a in battle_agents{
        max_agent = if a.get_point() >= max_agent.get_point() {a.clone()} else {max_agent};
    }
    
    return max_agent.clone();
}

#[test]
fn point_sum_test(){
    
    let agents = [Agent {
        id: 1,
        point: 10,
        dna_2_binary_digits: "11110000".to_string(),
        active: true,
    }, Agent {
        id: 2,
        point: 20,
        dna_2_binary_digits: "11110000".to_string(),
        active: true,
    }, Agent {
        id: 3,
        point: 30,
        dna_2_binary_digits: "11110000".to_string(),
        active: true,
    }];
    
    let sum_point = agents.iter().fold(0, |sum, a| sum + a.get_point());
    assert_eq!(sum_point, 60);
}

#[test]
fn get_agent_test(){
    let mut agents: Vec<Agent> = Vec::new();
    agents.push(Agent {
        id: 1,
        point: 0,
        dna_2_binary_digits: "11110000".to_string(),
        active: true,
    });
    agents.push(Agent {
        id: 2,
        point: 60,
        dna_2_binary_digits: "11110000".to_string(),
        active: true,
    });
    agents.push(Agent {
        id: 3,
        point: 0,
        dna_2_binary_digits: "11110000".to_string(),
        active: true,
    });
    
    let m = choose_model_roulettes(&agents, 3, 60);
    assert_eq!(m.id, 2);
}