use crate::models::model::{BaseModel, Model};
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub enum Choice {
    Cooperate,
    Defect,
}

pub trait StrategyOperation<T>: Clone
where
    T: BaseModel,
{
    fn play_match(&self, agent1: &T, agent2: &T) -> (T, T);
}

#[derive(Clone)]
pub struct ThresholdSelectionStrategy {}

#[derive(Clone)]
pub struct RouletteSelectionStrategy {}

impl<T> StrategyOperation<T> for ThresholdSelectionStrategy
where
    T: Model,
{
    fn play_match(&self, agent1: &T, agent2: &T) -> (T, T) {
        let choice1 = agent1
            .get_choice()
            .map(|v| get_threshold_choice(v as u64, agent1.get_dna_binary().len()))
            .unwrap_or(Choice::Defect);

        let choice2 = agent2
            .get_choice()
            .map(|v| get_threshold_choice(v as u64, agent2.get_dna_binary().len()))
            .unwrap_or(Choice::Defect);

        let points1 = calculate_payoff(&choice1, &choice2);
        let points2 = calculate_payoff(&choice2, &choice1);

        (
            agent1.with_points(agent1.get_points() + points1),
            agent2.with_points(agent2.get_points() + points2),
        )
    }
}

impl<T> StrategyOperation<T> for RouletteSelectionStrategy
where
    T: Model,
{
    fn play_match(&self, agent1: &T, agent2: &T) -> (T, T) {
        let choice1 = get_probabilistic_choice(agent1.get_dna_sum(), agent1.get_dna_binary().len());
        let choice2 = get_probabilistic_choice(agent2.get_dna_sum(), agent2.get_dna_binary().len());

        let points1 = calculate_payoff(&choice1, &choice2);
        let points2 = calculate_payoff(&choice2, &choice1);

        (
            agent1.with_points(agent1.get_points() + points1),
            agent2.with_points(agent2.get_points() + points2),
        )
    }
}

pub fn calculate_payoff(my_choice: &Choice, opponent_choice: &Choice) -> u64 {
    match (my_choice, opponent_choice) {
        (Choice::Cooperate, Choice::Cooperate) => 3,
        (Choice::Cooperate, Choice::Defect) => 0,
        (Choice::Defect, Choice::Cooperate) => 5,
        (Choice::Defect, Choice::Defect) => 1,
    }
}

fn get_threshold_choice(dna_value: u64, dna_length: usize) -> Choice {
    let threshold = 1u64 << (dna_length - 1);
    if dna_value < threshold {
        Choice::Cooperate
    } else {
        Choice::Defect
    }
}

fn get_probabilistic_choice(ones_count: u64, dna_length: usize) -> Choice {
    if ones_count == 0 {
        return Choice::Defect;
    }

    let mut rng = rand::thread_rng();
    let cooperation_probability = ones_count as f64 / dna_length as f64;

    if rng.gen::<f64>() < cooperation_probability {
        Choice::Cooperate
    } else {
        Choice::Defect
    }
}

#[test]
fn payoff_test() {
    assert_eq!(calculate_payoff(&Choice::Cooperate, &Choice::Cooperate), 3);
    assert_eq!(calculate_payoff(&Choice::Cooperate, &Choice::Defect), 0);
    assert_eq!(calculate_payoff(&Choice::Defect, &Choice::Cooperate), 5);
    assert_eq!(calculate_payoff(&Choice::Defect, &Choice::Defect), 1);
}

#[test]
fn agent_points_test() {
    use crate::models::model::Agent;

    let agent = Agent::new(1, "11100000".to_string());
    assert_eq!(agent.get_points(), 0);

    let agent = agent.with_points(3);
    assert_eq!(agent.get_points(), 3);

    let agent = agent.with_points(6);
    assert_eq!(agent.get_points(), 6);
}
