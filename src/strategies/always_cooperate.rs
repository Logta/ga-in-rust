use crate::simulation::environment::Choice;
use super::Strategy;

/// 常に協力する戦略
#[derive(Clone, Debug)]
pub struct AlwaysCooperate;

impl Strategy for AlwaysCooperate {
    fn name(&self) -> &str {
        "always-cooperate"
    }
    
    fn description(&self) -> &str {
        "常に協力する戦略"
    }
    
    fn decide(&self, _history: &[(Choice, Choice)], _round: usize) -> Choice {
        Choice::Cooperate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_always_cooperate() {
        let strategy = AlwaysCooperate;
        
        // 履歴がない場合
        assert_eq!(strategy.decide(&vec![], 0), Choice::Cooperate);
        
        // 履歴がある場合でも常に協力
        let history = vec![(Choice::Cooperate, Choice::Defect)];
        assert_eq!(strategy.decide(&history, 1), Choice::Cooperate);
    }
}