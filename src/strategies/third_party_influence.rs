/// 第三者影響戦略（Third-Party Influence）
/// 
/// 直接的な相互作用だけでなく、第三者からの評判情報と
/// 社会的圧力を考慮して協力判断を行う戦略
use crate::simulation::environment::Choice;
use crate::strategies::Strategy;
use std::collections::HashMap;

/// 第三者影響戦略
#[derive(Debug, Clone)]
pub struct ThirdPartyInfluence {
    /// 協力の閾値（この値以上なら協力）
    cooperation_threshold: f64,
}

impl ThirdPartyInfluence {
    /// 新しいThirdPartyInfluence戦略を作成
    pub fn new(cooperation_threshold: f64) -> Self {
        Self {
            cooperation_threshold,
        }
    }
    
    /// デフォルトの閾値（0.5）でThirdPartyInfluence戦略を作成
    pub fn default() -> Self {
        Self::new(0.5)
    }
}

impl Strategy for ThirdPartyInfluence {
    fn name(&self) -> &str {
        "third-party-influence"
    }
    
    fn description(&self) -> &str {
        "第三者からの評判情報と社会的圧力を考慮する戦略"
    }
    
    fn decide(&self, _history: &[(Choice, Choice)], _round: usize) -> Choice {
        // TODO: 実装予定
        Choice::Cooperate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_third_party_influence_creation() {
        let strategy = ThirdPartyInfluence::new(0.8);
        assert_eq!(strategy.name(), "third-party-influence");
        assert_eq!(strategy.cooperation_threshold, 0.8);
    }

    #[test]
    fn test_third_party_influence_default() {
        let strategy = ThirdPartyInfluence::default();
        assert_eq!(strategy.cooperation_threshold, 0.5);
    }

    #[test]
    fn test_strategy_description() {
        let strategy = ThirdPartyInfluence::default();
        assert_eq!(strategy.description(), "第三者からの評判情報と社会的圧力を考慮する戦略");
    }
}