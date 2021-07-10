use std::u32;

use crate::models::model;
use crate::models::model::{Agent, BaseModel, Model};
use rand::Rng;

pub enum Option {
    Cooperation, //協力
    Defection,   //裏切
}

pub trait StrategyOperation<T>
where
    T: BaseModel,
{
    fn get_result(&self, agent1: T, agent2: T) -> (T, T);
    fn get_new_strategy() -> Self;
}

#[derive(Clone)]
pub struct ThresholdSelectionStrategy {}

#[derive(Clone)]
pub struct RouletteSelectionStrategy {}

impl<T> StrategyOperation<T> for ThresholdSelectionStrategy
where
    T: Model,
{
    fn get_result(&self, agent1: T, agent2: T) -> (T, T) {
        let agent1_choose = match agent1.get_choose() {
            Ok(v) => v,
            Err(_) => 0,
        };

        let agent2_choose = match agent2.get_choose() {
            Ok(v) => v,
            Err(_) => 0,
        };

        let option_for_1: Option = get_option_threshold(
            agent1_choose as u64,
            agent1.get_dna_2_binary_digits().len() as usize,
        );
        let option_for_2: Option = get_option_threshold(
            agent2_choose as u64,
            agent2.get_dna_2_binary_digits().len() as usize,
        );

        (
            agent1
                .set_new_point(agent1.get_point() + get_result_point(&option_for_1, &option_for_2)),
            agent2
                .set_new_point(agent2.get_point() + get_result_point(&option_for_2, &option_for_1)),
        )
    }

    fn get_new_strategy() -> Self {
        Self {}
    }
}

impl<T> StrategyOperation<T> for RouletteSelectionStrategy
where
    T: Model,
{
    fn get_result(&self, agent1: T, agent2: T) -> (T, T) {
        let option_for_1: Option = get_option_probability(
            agent1.get_dna_sum(),
            agent1.get_dna_2_binary_digits().len() as usize,
        );
        let option_for_2: Option = get_option_probability(
            agent2.get_dna_sum(),
            agent2.get_dna_2_binary_digits().len() as usize,
        );

        (
            agent1
                .set_new_point(agent1.get_point() + get_result_point(&option_for_1, &option_for_2)),
            agent2
                .set_new_point(agent2.get_point() + get_result_point(&option_for_2, &option_for_1)),
        )
    }

    fn get_new_strategy() -> Self {
        Self {}
    }
}

// own_optionが取得する得点を計算する
fn get_result_point(own_option: &Option, enemy_option: &Option) -> u64 {
    match (own_option, enemy_option) {
        (Option::Cooperation, Option::Cooperation) => 3,
        (Option::Cooperation, Option::Defection) => 0,
        (Option::Defection, Option::Cooperation) => 5,
        _ => 1,
    }
}

pub fn get_option(dna_num: u64, dna_max_num: u16) -> Option {
    if dna_num < (dna_max_num / 2).into() {
        Option::Cooperation
    } else {
        Option::Defection
    }
}

//遺伝子配列中に1の数が半数を超えたらCを選ぶ
pub fn get_option_threshold(dna_num: u64, dna_max_num: usize) -> Option {
    let base: u32 = 2;
    let two_pow: u32 = base.pow(dna_max_num as u32);
    if dna_num < (two_pow / 2).into() {
        Option::Cooperation
    } else {
        Option::Defection
    }
}

//確率で選択肢を選ぶ
pub fn get_option_probability(dna_num: u64, dna_max_num: usize) -> Option {
    let mut rng = rand::thread_rng();
    let probability: f64 = rng.gen();

    match (
        dna_num == 0,
        probability < (dna_num as f64 / dna_max_num as f64).into(),
    ) {
        (true, _) => Option::Defection,
        (false, true) => Option::Cooperation,
        (false, false) => Option::Defection,
    }
}

#[test]
fn get_option_test() {
    let m1 = Agent::new_base_model(1, "11110100".to_string());
    let m2 = Agent::new_base_model(1, "11110100".to_string());

    let option_for_1: Option =
        get_option(m1.get_dna_sum(), m1.get_dna_2_binary_digits().len() as u16);
    let option_for_2: Option =
        get_option(m2.get_dna_sum(), m2.get_dna_2_binary_digits().len() as u16);
    assert_eq!(get_result_point(&option_for_1, &option_for_2), 1);

    let m1 = Agent::new_base_model(1, "11100000".to_string());
    let m2 = Agent::new_base_model(1, "11100000".to_string());

    let option_for_1: Option =
        get_option(m1.get_dna_sum(), m1.get_dna_2_binary_digits().len() as u16);
    let option_for_2: Option =
        get_option(m2.get_dna_sum(), m2.get_dna_2_binary_digits().len() as u16);
    assert_eq!(get_result_point(&option_for_1, &option_for_2), 3);

    let m3 = Agent::new_base_model(1, "11100000".to_string());
    assert_eq!(m3.get_point(), 0);

    let m3 = m3.set_new_point(m3.get_point() + 3);
    assert_eq!(m3.get_point(), 3);

    let m3 = m3.set_new_point(m3.get_point() + 3);
    assert_eq!(m3.get_point(), 6);
}
