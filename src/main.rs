use ga_prisoners_dilemma::models::game;
use ga_prisoners_dilemma::models::game::GameOperation;
use ga_prisoners_dilemma::models::model;
use ga_prisoners_dilemma::models::model::BaseModel;
use ga_prisoners_dilemma::models::model::Model;

fn main() {
    let mut m = model::new_base_model(1,"11110000".to_string());
    println!("{}", m.get_dna_2_binary_digits());
    println!("{}", m.get_point());
    m = m.mutation();
    println!("{}", m.get_dna_2_binary_digits());

    let g = game::new_game(10, 1, 6);
    for dna in g.get_dna_list().iter(){
        println!("{}",dna);
    }
}
