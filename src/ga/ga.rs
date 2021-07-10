use crate::models::model::{Agent, BaseModel, Model};
use crate::strategies::utils::StrategyOperation;
use rand::{thread_rng, Rng};

use crate::models::game;
use crate::models::game::Game;

pub trait GAOperation {
    fn get_point_list(&self) -> Vec<u64>;
    fn get_dna_list(&self) -> Vec<String>;
}

// トレイトを実装するためだけのデータ型にはUnit構造体が便利
pub struct GA<T: BaseModel> {
    pub old_agents: Vec<Box<T>>,
    pub mutation_rate: f64,
    pub population: u64,
    pub dna_length: u64,
    pub num_game: u64, // １世代でのゲーム回数
}

// `impl トレイト名 for 型名 {..}`で定義可能
impl<T: Model> GAOperation for GA<T> {
    fn get_dna_list(&self) -> Vec<String> {
        self.old_agents
            .iter()
            .map(|x| x.get_dna_2_binary_digits())
            .collect()
    }

    fn get_point_list(&self) -> Vec<u64> {
        let iter = self.old_agents.iter();
        iter.map(|x| x.get_point()).collect()
    }
}

pub fn get_new_game<T, U>(ga: GA<T>, strategy: U) -> Game<T, U>
where
    T: Model,
    U: StrategyOperation<T>,
{
    let agents = (0..ga.population)
        .map(|x| {
            Box::from(T::new_base_model(
                x,
                get_dna(&ga.old_agents, ga.population, ga.mutation_rate),
            ))
        })
        .collect::<Vec<Box<T>>>();

    game::generate_next_game::<T, U>(
        ga.population,
        ga.mutation_rate,
        ga.num_game,
        ga.dna_length,
        agents,
        strategy,
    )
}

fn get_dna<T: Model>(agents: &Vec<Box<T>>, poplation: u64, mutation_rate: f64) -> String {
    let (ch_ag1, ch_ag2) = choose_model_parent(agents, poplation);

    let mut rng = thread_rng();
    let cross_point: u64 = rng.gen_range(0..ch_ag1.get_dna_length());

    let ch_ag1 = ch_ag1.crossover(&ch_ag2, cross_point as usize);
    ch_ag1.mutation(mutation_rate).get_dna_2_binary_digits()
}

fn choose_model_parent<T: BaseModel>(agents: &Vec<Box<T>>, poplation: u64) -> (T, T) {
    let sum_point = agents
        .iter()
        .fold(0, |sum, a| sum + a.get_point() * a.get_point());

    let ch_ag1 = choose_model_roulettes(agents, poplation, sum_point);
    let ch_ag2 = choose_model_roulettes(agents, poplation, sum_point);

    (ch_ag1, ch_ag2)
}

fn choose_model_roulettes<T: BaseModel>(agents: &Vec<Box<T>>, poplation: u64, sum_point: u64) -> T {
    let mut rng = thread_rng();
    let mut rand_num1: i64 = rng.gen_range(0..sum_point) as i64;

    for p in 0..poplation {
        rand_num1 -= (agents[p as usize].get_point() * agents[p as usize].get_point()) as i64;
        if rand_num1 <= 0 {
            return *agents[p as usize].clone();
        }
    }
    return *agents[0].clone();
}

fn get_random_model_indexes(poplation: u64, select_num: u16) -> Vec<u64> {
    let mut indexes = (0..poplation).collect::<Vec<u64>>();

    for i in 0..poplation {
        let j = (get_rand() * poplation as f64) as usize;
        indexes.swap(i as usize, j);
    }
    (0..select_num as usize)
        .map(|s| indexes[s])
        .collect::<Vec<u64>>()
}

fn get_rand() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

fn choose_model_tournament(agents: &Vec<Agent>, indexes: &[u64]) -> Agent {
    let battle_agents = agents.iter().filter(|a| indexes.contains(&a.id));

    let mut max_agent: Agent = Agent {
        id: 0,
        point: 0,
        dna_2_binary_digits: "".to_string(),
        active: false,
    };
    for a in battle_agents {
        max_agent = if a.get_point() >= max_agent.get_point() {
            a.clone()
        } else {
            max_agent
        };
    }
    return max_agent.clone();
}

#[test]
fn point_sum_test() {
    let agents = [
        Agent {
            id: 1,
            point: 10,
            dna_2_binary_digits: "11110000".to_string(),
            active: true,
        },
        Agent {
            id: 2,
            point: 20,
            dna_2_binary_digits: "11110000".to_string(),
            active: true,
        },
        Agent {
            id: 3,
            point: 30,
            dna_2_binary_digits: "11110000".to_string(),
            active: true,
        },
    ];
    let sum_point = agents.iter().fold(0, |sum, a| sum + a.get_point());
    assert_eq!(sum_point, 60);
}

#[test]
fn get_agent_test() {
    let mut agents: Vec<Box<Agent>> = Vec::new();
    agents.push(Box::from(Agent {
        id: 1,
        point: 0,
        dna_2_binary_digits: "11110000".to_string(),
        active: true,
    }));
    agents.push(Box::from(Agent {
        id: 2,
        point: 60,
        dna_2_binary_digits: "11110000".to_string(),
        active: true,
    }));
    agents.push(Box::from(Agent {
        id: 3,
        point: 0,
        dna_2_binary_digits: "11110000".to_string(),
        active: true,
    }));
    let m = choose_model_roulettes(&agents, 3, 60);
    assert_eq!(m.id, 2);
}
