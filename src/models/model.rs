use core::num::ParseIntError;

// `trait トレイト名 {..}`でトレイトを定義
pub trait BaseModel {
    fn mutation(&self) -> Self;
    fn crossover(&self, other: &Self, crossing_point: u32) -> &Self;
    fn set_new_point(&self, point: u64) -> Self;
    fn get_choose(&self) -> Result<u32, ParseIntError>;
    fn get_point(&self) -> u64;
}

pub trait Model: BaseModel {
    fn get_dna_2_binary_digits(&self) -> String;
}

// トレイトを実装するためだけのデータ型にはUnit構造体が便利
pub struct Ajent{
    id: u64,
    point: u64,
    dna_2_binary_digits: String,
    active: bool,
}

// `impl トレイト名 for 型名 {..}`で定義可能
impl BaseModel for Ajent {

    fn get_point(&self) -> u64 {
        self.point
    }

    fn get_choose(&self) -> Result<u32, ParseIntError> {
        let r = u32::from_str_radix(&self.dna_2_binary_digits, 2)?;
        return Ok(r)
    }

    fn set_new_point(&self, point: u64) -> Ajent {
        let m: &Ajent = self.clone();
        Ajent {
            point,
            id: m.id,
            dna_2_binary_digits: String::from(&m.dna_2_binary_digits),
            active: m.active
        }
    }

    fn crossover(&self, _other: &Ajent, _crossing_point: u32) -> &Ajent {
        return self
    }

    fn mutation(&self) -> Ajent {
        let m: &Ajent = self.clone();
        let vec_dna: Vec<char> = m.dna_2_binary_digits.chars().collect();
        let new_dna: String = vec_dna.into_iter().map(|x| mutation_2_one_factor(x)).collect();
        Ajent {
            id: m.id,
            point: 0,
            dna_2_binary_digits: new_dna,
            active: true,
        }
    }
}

// `impl トレイト名 for 型名 {..}`で定義可能
impl Model for Ajent {

    fn get_dna_2_binary_digits(&self) -> String{
        self.dna_2_binary_digits.clone()
    }
}

pub fn new_base_model(id: u64, dna_2_binary_digits: String) -> Ajent {
    Ajent {
        id: id,
        point: 0,
        dna_2_binary_digits: String::from(dna_2_binary_digits.clone()),
        active: true,
    }
}

fn mutation_2_one_factor(c: char) -> char {
    match c {
        '1' => '0',
        '0' => '1',
        _ => '0',
    }
}