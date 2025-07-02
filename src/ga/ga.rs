use crate::models::model::{AgentId, BaseModel, Dna, Model, Points};
use crate::strategies::utils::StrategyOperation;
use rand::{thread_rng, Rng};

use crate::models::game;
use crate::models::game::Game;

pub trait GAOperation<T: BaseModel> {
    fn get_points_list(&self) -> Vec<Points>;
    fn get_dna_list(&self) -> Vec<String>;
}

pub struct GA<T: BaseModel> {
    pub old_agents: Vec<Box<T>>,
    pub mutation_rate: f64,
    pub population: usize,
    pub dna_length: usize,
    pub num_games: usize,
}

impl<T: Model> GAOperation<T> for GA<T> {
    fn get_dna_list(&self) -> Vec<String> {
        self.old_agents
            .iter()
            .map(|agent| agent.get_dna_binary().to_string())
            .collect()
    }

    fn get_points_list(&self) -> Vec<Points> {
        self.old_agents
            .iter()
            .map(|agent| agent.get_points())
            .collect()
    }
}

pub fn create_next_generation<T, U>(ga: GA<T>, strategy: U) -> Game<T, U>
where
    T: Model,
    U: StrategyOperation<T>,
{
    let agents = (0..ga.population)
        .map(|i| {
            Box::from(T::new(
                i as AgentId,
                generate_offspring_dna(&ga.old_agents, ga.population, ga.mutation_rate),
            ))
        })
        .collect::<Vec<Box<T>>>();

    game::generate_next_game::<T, U>(
        ga.population,
        ga.mutation_rate,
        ga.num_games,
        ga.dna_length,
        agents,
        strategy,
    )
}

fn generate_offspring_dna<T: Model>(
    agents: &[Box<T>],
    population: usize,
    mutation_rate: f64,
) -> Dna {
    let (parent1, parent2) = select_parents(agents, population);

    let mut rng = thread_rng();
    let cross_point = rng.gen_range(0..parent1.get_dna_length());

    let offspring = parent1.crossover(&parent2, cross_point);
    offspring
        .mutation(mutation_rate)
        .get_dna_binary()
        .to_string()
}

fn select_parents<T: BaseModel>(agents: &[Box<T>], population: usize) -> (T, T) {
    let fitness_sum = agents
        .iter()
        .map(|a| {
            let points = a.get_points();
            points * points
        })
        .sum();

    let parent1 = roulette_wheel_selection(agents, population, fitness_sum);
    let parent2 = roulette_wheel_selection(agents, population, fitness_sum);

    (parent1, parent2)
}

fn roulette_wheel_selection<T: BaseModel>(
    agents: &[Box<T>],
    _population: usize,
    fitness_sum: u64,
) -> T {
    let mut rng = thread_rng();
    let mut selection_point = rng.gen_range(0..fitness_sum) as i64;

    for agent in agents {
        let fitness = agent.get_points();
        selection_point -= (fitness * fitness) as i64;
        if selection_point <= 0 {
            return (**agent).clone();
        }
    }

    (**agents.first().expect("Empty agents list")).clone()
}

#[test]
fn points_sum_test() {
    use crate::models::model::Agent;

    let agents = [
        Agent {
            id: 1,
            points: 10,
            dna: "11110000".to_string(),
            active: true,
        },
        Agent {
            id: 2,
            points: 20,
            dna: "11110000".to_string(),
            active: true,
        },
        Agent {
            id: 3,
            points: 30,
            dna: "11110000".to_string(),
            active: true,
        },
    ];
    let sum_points: u64 = agents.iter().map(|a| a.get_points()).sum();
    assert_eq!(sum_points, 60);
}

#[test]
fn selection_test() {
    use crate::models::model::Agent;

    let agents: Vec<Box<Agent>> = vec![
        Box::new(Agent {
            id: 1,
            points: 0,
            dna: "11110000".to_string(),
            active: true,
        }),
        Box::new(Agent {
            id: 2,
            points: 60,
            dna: "11110000".to_string(),
            active: true,
        }),
        Box::new(Agent {
            id: 3,
            points: 0,
            dna: "11110000".to_string(),
            active: true,
        }),
    ];
    let selected = roulette_wheel_selection(&agents, 3, 3600);
    assert_eq!(selected.id, 2);
}
