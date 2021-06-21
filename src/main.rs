use ga_prisoners_dilemma::ga::ga::GAOperation;
use ga_prisoners_dilemma::models::game;
use ga_prisoners_dilemma::models::game::GameOperation;

fn main() {
    // ゲームの用意
    let generation = 5000;
    let mut game = game::new_game(10, 0.1, 50,6);
    println!("GA on prisoners' dilemma Start!!");
    println!("最初の世代のDNA一覧");
    for dna in game.get_dna_list().iter(){
        println!("{}",dna);
    }
    println!("");
    
    // GA開始
    for index in 0..generation {
        let ga = game.do_game();
        game = ga.get_new_game();
        if index % 500 == 0{
            println!("-----")}
    }

    // 結果確認
    println!("GA {}世代が完了しました!", generation);
    for dna in game.get_dna_list().iter(){
        println!("{}",dna);
    }
    println!("");
}
