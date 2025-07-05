use crate::simulation::environment::Choice;
use super::Strategy;

/// 常に裏切る戦略
#[derive(Clone, Debug)]
pub struct AlwaysDefect;

impl Strategy for AlwaysDefect {
    fn name(&self) -> &str {
        "always-defect"
    }
    
    fn description(&self) -> &str {
        "常に裏切る戦略"
    }
    
    fn decide(&self, _history: &[(Choice, Choice)], _round: usize) -> Choice {
        Choice::Defect
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_always_defect() {
        let strategy = AlwaysDefect;
        
        // 履歴がない場合
        assert_eq!(strategy.decide(&vec![], 0), Choice::Defect);
        
        // 履歴がある場合でも常に裏切り
        let history = vec![(Choice::Defect, Choice::Cooperate)];
        assert_eq!(strategy.decide(&history, 1), Choice::Defect);
    }
}