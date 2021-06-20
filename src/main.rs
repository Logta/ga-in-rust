use ga_prisoners_dilemma::models::game;
use ga_prisoners_dilemma::models::game::GameOperation;

fn main() {
    let g = game::new_game(10, 0.1, 6);
    for dna in g.get_dna_list().iter(){
        println!("{}",dna);
    }
}
