use std::u32;

use crate::models::model::{Agent, Model, BaseModel};

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
        let agent1_choose = match agent1.get_choose() {
            Ok(v) => v,
            Err(_) => 0,
        };

        let agent2_choose = match agent2.get_choose() {
            Ok(v) => v,
            Err(_) => 0,
        };

        let option_for_1: Option = get_option(agent1_choose as u64, agent1.get_dna_2_binary_digits().len());
        let option_for_2: Option = get_option(agent2_choose as u64, agent2.get_dna_2_binary_digits().len());

        (
            agent1.set_new_point(get_result_point(&option_for_1, &option_for_2)),
            agent2.set_new_point(get_result_point(&option_for_2, &option_for_1)),
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

pub fn get_option(dna_num: u64, dna_max_num: usize) -> Option{
    let base: u32 = 2; 
    let two_pow:u32 = base.pow(dna_max_num as u32 );
    if dna_num > (two_pow / 2).into() {
        Option::Cooperation
    }else{
        Option::Defection
    }
}