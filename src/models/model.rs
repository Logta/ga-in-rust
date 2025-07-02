use core::num::ParseIntError;
use rand::Rng;

pub type Dna = String;
pub type AgentId = u64;
pub type Points = u64;

pub trait BaseModel: Clone {
    fn mutation(&self, mutation_rate: f64) -> Self;
    fn crossover(&self, other: &Self, crossing_point: usize) -> Self;
    fn with_points(&self, points: Points) -> Self;
    fn get_choice(&self) -> Result<u32, ParseIntError>;
    fn get_points(&self) -> Points;
    fn get_dna_length(&self) -> usize;
    fn get_dna_sum(&self) -> u64;
    fn get_dna(&self) -> &str;
    fn new(id: AgentId, dna: Dna) -> Self;
}

pub trait Model: BaseModel {
    fn get_dna_binary(&self) -> &str;
}

#[derive(Clone, Debug)]
pub struct Agent {
    pub(crate) id: AgentId,
    pub(crate) points: Points,
    pub(crate) dna: Dna,
    pub(crate) active: bool,
}

impl BaseModel for Agent {
    fn get_points(&self) -> Points {
        self.points
    }

    fn get_choice(&self) -> Result<u32, ParseIntError> {
        u32::from_str_radix(&self.dna, 2)
    }

    fn with_points(&self, points: Points) -> Agent {
        Agent {
            points,
            id: self.id,
            dna: self.dna.clone(),
            active: self.active,
        }
    }

    fn crossover(&self, other: &Agent, crossing_point: usize) -> Agent {
        let head = &self.dna[..crossing_point.min(self.dna.len())];
        let tail = &other.dna[crossing_point.min(other.dna.len())..];
        
        Agent {
            id: self.id,
            points: 0,
            dna: format!("{}{}", head, tail),
            active: true,
        }
    }

    fn mutation(&self, mutation_rate: f64) -> Agent {
        let new_dna: String = self.dna
            .chars()
            .map(|c| mutate_bit(c, mutation_rate))
            .collect();
            
        Agent {
            id: self.id,
            points: 0,
            dna: new_dna,
            active: true,
        }
    }

    fn get_dna_length(&self) -> usize {
        self.dna.len()
    }

    fn get_dna_sum(&self) -> u64 {
        self.dna
            .chars()
            .filter(|&c| c == '1')
            .count() as u64
    }

    fn get_dna(&self) -> &str {
        &self.dna
    }

    fn new(id: AgentId, dna: Dna) -> Self {
        Self {
            id,
            points: 0,
            dna,
            active: true,
        }
    }
}

impl Model for Agent {
    fn get_dna_binary(&self) -> &str {
        &self.dna
    }
}

fn mutate_bit(bit: char, mutation_rate: f64) -> char {
    let mut rng = rand::thread_rng();
    if rng.gen::<f64>() < mutation_rate {
        match bit {
            '0' => '1',
            '1' => '0',
            _ => bit,
        }
    } else {
        bit
    }
}

#[test]
fn dna_operation_test() {
    let mut m1: Agent = BaseModel::new(1, "11110000".to_string());
    let mut m2: Agent = BaseModel::new(1, "00001111".to_string());
    assert_eq!("11110000", m1.get_dna_binary());
    assert_eq!("00001111", m2.get_dna_binary());
    assert_eq!(4, m1.get_dna_sum());
    assert_eq!(4, m2.get_dna_sum());
    m2 = m1.crossover(&m2, 4);
    assert_eq!("11111111", m2.get_dna_binary());
    assert_eq!(8, m2.get_dna_sum());

    m1 = m1.mutation(0.2);
    assert_eq!(8, m1.get_dna_binary().len());
}
