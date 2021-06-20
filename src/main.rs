use ga_prisoners_dilemma::ga::ga::GAOperation;
use ga_prisoners_dilemma::models::game;
use ga_prisoners_dilemma::models::game::GameOperation;

fn main() {
    // ゲームの用意
    let mut game = game::new_game(10, 0.1, 50,6);
    for dna in game.get_dna_list().iter(){
        println!("{}",dna);
    }
    
    // GA開始
    for _ in 0..5000 {
        let ga = game.do_game();
        game = ga.get_new_game();
    }

    // 結果確認
    for dna in game.get_dna_list().iter(){
        println!("{}",dna);
    }
}
