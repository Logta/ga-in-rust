use crate::simulation::environment::Choice;
use super::Strategy;

/// しっぺ返し戦略（Tit-for-Tat）
/// 
/// 初回は協力し、以降は相手の前回の行動を真似る戦略
#[derive(Clone, Debug)]
pub struct TitForTat;

impl Strategy for TitForTat {
    fn name(&self) -> &str {
        "tit-for-tat"
    }
    
    fn description(&self) -> &str {
        "相手の前回の行動を真似る戦略"
    }
    
    fn decide(&self, history: &[(Choice, Choice)], _round: usize) -> Choice {
        if history.is_empty() {
            Choice::Cooperate // 初回は協力
        } else {
            history.last().unwrap().1 // 相手の前回の行動を真似る
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tit_for_tat_first_round_cooperates() {
        let tft = TitForTat;
        let history = vec![];
        assert_eq!(tft.decide(&history, 0), Choice::Cooperate);
    }
    
    #[test]
    fn test_tit_for_tat_copies_opponent() {
        let tft = TitForTat;
        
        // 相手が協力した場合
        let history = vec![(Choice::Cooperate, Choice::Cooperate)];
        assert_eq!(tft.decide(&history, 1), Choice::Cooperate);
        
        // 相手が裏切った場合
        let history = vec![(Choice::Cooperate, Choice::Defect)];
        assert_eq!(tft.decide(&history, 1), Choice::Defect);
    }
}