use core::num::ParseIntError;
use rand::{thread_rng, Rng};

// `trait トレイト名 {..}`でトレイトを定義
pub trait BaseModel {
    fn mutation(&self, mutation_rate: f64) -> Self;
    fn crossover(&self, other: &Self, crossing_point: usize) -> Self;
    fn set_new_point(&self, point: u64) -> Self;
    fn get_choose(&self) -> Result<u32, ParseIntError>;
    fn get_point(&self) -> u64;
}

pub trait Model: BaseModel {
    fn get_dna_2_binary_digits(&self) -> String;
}

// トレイトを実装するためだけのデータ型にはUnit構造体が便利
pub struct Agent {
    id: u64,
    point: u64,
    dna_2_binary_digits: String,
    active: bool,
}

// `impl トレイト名 for 型名 {..}`で定義可能
impl BaseModel for Agent {
    fn get_point(&self) -> u64 {
        self.point
    }

    fn get_choose(&self) -> Result<u32, ParseIntError> {
        let r = u32::from_str_radix(&self.dna_2_binary_digits, 2)?;
        return Ok(r)
    }

    fn set_new_point(&self, point: u64) -> Agent {
        let m: &Agent = self.clone();
        Agent {
            point,
            id: m.id,
            dna_2_binary_digits: String::from(&m.dna_2_binary_digits),
            active: m.active
        }
    }

    fn crossover(&self, _other: &Agent, _crossing_point: usize) -> Agent {
        let m: &Agent = self.clone();
        let head = self.dna_2_binary_digits.chars().take(_crossing_point).collect::<String>();
        let tail = _other.dna_2_binary_digits.chars().skip(_crossing_point).collect::<String>();

        let new_dna: String = head + &tail;
        Agent {
            id: m.id,
            point: 0,
            dna_2_binary_digits: new_dna,
            active: true,
        }
    }

    fn mutation(&self, mutation_rate: f64) -> Agent {
        let m: &Agent = self.clone();
        let vec_dna: Vec<char> = m.dna_2_binary_digits.chars().collect();
        let new_dna: String = vec_dna.into_iter().map(|x| mutation_2_one_factor(x, mutation_rate)).collect();
        Agent {
            id: m.id,
            point: 0,
            dna_2_binary_digits: new_dna,
            active: true,
        }
    }
}

// `impl トレイト名 for 型名 {..}`で定義可能
impl Model for Agent {
    fn get_dna_2_binary_digits(&self) -> String {
        self.dna_2_binary_digits.clone()
    }
}

pub fn new_base_model(id: u64, dna_2_binary_digits: String) -> Agent {
    Agent {
        id,
        point: 0,
        dna_2_binary_digits: String::from(dna_2_binary_digits.clone()),
        active: true,
    }
}

fn mutation_2_one_factor(c: char, mutation_rate: f64) -> char {
    
    let mut rng = rand::thread_rng();
    let probability: f64 = rng.gen();
    
    // c => 元のdna因子、
    // probability < mutation_rate => true  : 突然変異する、因子を反転させる
    //                                false : 突然変異しない
    match (c, probability < mutation_rate) {
        ('1', true) => '0',
        ('0', true) => '1',
        _ => c,
    }
}

#[test]
fn dna_operation_test(){
    
    let mut m1 = new_base_model(1,"11110000".to_string());
    let mut m2 = new_base_model(1,"00001111".to_string());
    assert_eq!("11110000", m1.get_dna_2_binary_digits());
    assert_eq!("00001111", m2.get_dna_2_binary_digits());
    
    m2 = m1.crossover(&m2, 4);
    assert_eq!("11111111", m2.get_dna_2_binary_digits());

    m1 = m1.mutation(0.2);
    assert_eq!(8, m1.get_dna_2_binary_digits().len());

}