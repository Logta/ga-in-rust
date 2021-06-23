use std::u32;

use crate::models::model::{Agent, Model, BaseModel};
use crate::models::model;
use rand::Rng;

pub enum Option {
    Cooperation, //協力
    Defection, //裏切
}

pub trait StrategyOperation {
    fn get_result(agent1: Agent, agent2: Agent) -> (Agent, Agent);
}

#[derive (Clone)]
pub struct Strategy{}

impl StrategyOperation for Strategy {
    fn get_result(agent1: Agent, agent2: Agent) -> (Agent, Agent){
        // let agent1_choose = match agent1.get_choose() {
        //     Ok(v) => v,
        //     Err(_) => 0,
        // };

        // let agent2_choose = match agent2.get_choose() {
        //     Ok(v) => v,
        //     Err(_) => 0,
        // };

        // let option_for_1: Option = get_option2(agent1_choose as u64, agent1.get_dna_2_binary_digits().len() as u16);
        // let option_for_2: Option = get_option2(agent2_choose as u64, agent2.get_dna_2_binary_digits().len() as u16);

        // let option_for_1: Option = get_option(agent1.get_dna_sum(), agent1.get_dna_2_binary_digits().len() as u16);
        // let option_for_2: Option = get_option(agent2.get_dna_sum(), agent2.get_dna_2_binary_digits().len() as u16);

        let option_for_1: Option = get_option_probability(agent1.get_dna_sum(), agent1.get_dna_2_binary_digits().len() as usize);
        let option_for_2: Option = get_option_probability(agent2.get_dna_sum(), agent2.get_dna_2_binary_digits().len() as usize);

        (
            agent1.set_new_point(agent1.get_point() + get_result_point(&option_for_1, &option_for_2)),
            agent2.set_new_point(agent2.get_point() + get_result_point(&option_for_2, &option_for_1)),
        )
    }
}

// own_optionが取得する得点を計算する
fn get_result_point(own_option: &Option, enemy_option: &Option) -> u64{
    match (own_option, enemy_option) {
        (Option::Cooperation, Option::Cooperation) => 3,
        (Option::Cooperation, Option::Defection) => 0,
        (Option::Defection, Option::Cooperation) => 5,
        _ => 1,
    }
}

pub fn get_option(dna_num: u64, dna_max_num: u16) -> Option{
    if dna_num < (dna_max_num / 2).into() {
        Option::Cooperation
    }else{
        Option::Defection
    }
}

pub fn get_option2(dna_num: u64, dna_max_num: usize) -> Option{
    let base: u32 = 2; 
    let two_pow:u32 = base.pow(dna_max_num as u32 );
    if dna_num < (two_pow / 2).into() {
        Option::Cooperation
    }else{
        Option::Defection
    }
}

pub fn get_option_probability(dna_num: u64, dna_max_num: usize) -> Option{
    let base: u32 = 2; 
    let two_pow = base.pow(dna_max_num as u32 );

    let mut rng = rand::thread_rng();
    let probability: f64 = rng.gen();

    match(two_pow==0, probability < (dna_num as f64 / two_pow as f64).into()) {
        (true, _) => Option::Cooperation,
        (false, true) => Option::Cooperation,
        (false, false) => Option::Defection,
    }
}

#[test]
fn get_option_test(){
    
    let m1 = model::new_base_model(1,"11110100".to_string());
    let m2 = model::new_base_model(1,"11110100".to_string());
    
    let option_for_1: Option = get_option(m1.get_dna_sum(), m1.get_dna_2_binary_digits().len() as u16);
    let option_for_2: Option = get_option(m2.get_dna_sum(), m2.get_dna_2_binary_digits().len() as u16);
    assert_eq!(get_result_point(&option_for_1, &option_for_2), 1);
    
    let m1 = model::new_base_model(1,"11100000".to_string());
    let m2 = model::new_base_model(1,"11100000".to_string());
    
    let option_for_1: Option = get_option(m1.get_dna_sum(), m1.get_dna_2_binary_digits().len() as u16);
    let option_for_2: Option = get_option(m2.get_dna_sum(), m2.get_dna_2_binary_digits().len() as u16);
    assert_eq!(get_result_point(&option_for_1, &option_for_2), 3);

    let m3 = model::new_base_model(1,"11100000".to_string());
    assert_eq!(m3.get_point(), 0);

    let m3 = m3.set_new_point(m3.get_point() + 3);
    assert_eq!(m3.get_point(), 3);

    let m3 = m3.set_new_point(m3.get_point() + 3);
    assert_eq!(m3.get_point(), 6);
}