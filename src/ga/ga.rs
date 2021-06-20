use crate::models::model;

use super::super::models::model::{Agent};

pub trait GAOperation {
    fn get_model(&self) -> Agent;
    fn get_new_model(&self) -> Agent;
}

// トレイトを実装するためだけのデータ型にはUnit構造体が便利
pub struct GA{
    old_agents: Agent,
}

// `impl トレイト名 for 型名 {..}`で定義可能
impl GAOperation for GA {
    fn get_model(&self) -> Agent{
        model::new_base_model(1,"11110000".to_string())
    }

    fn get_new_model(&self) -> Agent{
        model::new_base_model(1,"11110000".to_string())
    }
}