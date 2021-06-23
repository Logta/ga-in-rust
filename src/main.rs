use ga_prisoners_dilemma::ga::ga::GAOperation;
use ga_prisoners_dilemma::models::game;
use ga_prisoners_dilemma::models::game::GameOperation;

fn main() {
    // ゲームの用意
    let generation = 50000;
    let population = 20;
    let mut game = game::new_game(population, 0.1, 1,6);
    println!("GA on prisoners' dilemma Start!!");
    println!("最初の世代のDNA一覧");
    for dna in game.get_dna_list().iter(){
        println!("{}",dna);
    }
    println!("");
    
    // GA開始
    for index in 0..generation {
        let ga = game.do_game();
        if index % 5000 == 0{
            println!("-----");
            for p in 0..population{
                println!("{}",ga.get_dna_list()[p as usize]);
                println!("{}",ga.get_point_list()[p as usize]);
            }
            println!("-----");
        }
        game = ga.get_new_game();
    }

    // 結果確認
    println!("GA {}世代が完了しました!", generation);
    for dna in game.get_dna_list().iter(){
        println!("{}",dna);
    }
    println!("");
}
