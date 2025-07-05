use crate::simulation::environment::Choice;
use crate::core::random::utils;
use super::Strategy;

/// ランダム戦略
/// 
/// 50%の確率で協力または裏切りを選択
#[derive(Clone, Debug)]
pub struct Random;

impl Strategy for Random {
    fn name(&self) -> &str {
        "random"
    }
    
    fn description(&self) -> &str {
        "ランダムに行動を選択する戦略"
    }
    
    fn decide(&self, _history: &[(Choice, Choice)], _round: usize) -> Choice {
        if utils::random_bool(0.5).unwrap_or(false) {
            Choice::Cooperate
        } else {
            Choice::Defect
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_random_strategy_name() {
        let strategy = Random;
        assert_eq!(strategy.name(), "random");
    }
    
    #[test]
    fn test_random_returns_valid_choice() {
        let strategy = Random;
        let history = vec![];
        
        // 複数回実行して、両方の選択肢が返されることを確認
        let mut cooperate_count = 0;
        let mut defect_count = 0;
        
        for _ in 0..100 {
            match strategy.decide(&history, 0) {
                Choice::Cooperate => cooperate_count += 1,
                Choice::Defect => defect_count += 1,
            }
        }
        
        // 両方の選択肢が少なくとも1回は選ばれているはず
        assert!(cooperate_count > 0);
        assert!(defect_count > 0);
    }
}