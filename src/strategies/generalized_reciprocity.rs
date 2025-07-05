/// 一般化互恵戦略（Generalized Reciprocity）
/// 
/// 直接的な互恵関係だけでなく、グループ全体への貢献と
/// グループからの恩恵を考慮して協力判断を行う戦略
use crate::simulation::environment::Choice;
use crate::strategies::Strategy;

/// 一般化互恵戦略
#[derive(Debug, Clone)]
pub struct GeneralizedReciprocity {
    /// 協力の閾値（この値以上なら協力）
    cooperation_threshold: f64,
}

impl GeneralizedReciprocity {
    /// 新しいGeneralizedReciprocity戦略を作成
    pub fn new(cooperation_threshold: f64) -> Self {
        Self {
            cooperation_threshold,
        }
    }
    
    /// デフォルトの閾値（0.6）でGeneralizedReciprocity戦略を作成
    pub fn default() -> Self {
        Self::new(0.6)
    }
}

impl Strategy for GeneralizedReciprocity {
    fn name(&self) -> &str {
        "generalized-reciprocity"
    }
    
    fn description(&self) -> &str {
        "グループ全体への貢献と恩恵を考慮する一般化互恵戦略"
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
    fn test_generalized_reciprocity_creation() {
        let strategy = GeneralizedReciprocity::new(0.7);
        assert_eq!(strategy.name(), "generalized-reciprocity");
        assert_eq!(strategy.cooperation_threshold, 0.7);
    }

    #[test]
    fn test_generalized_reciprocity_default() {
        let strategy = GeneralizedReciprocity::default();
        assert_eq!(strategy.cooperation_threshold, 0.6);
    }

    #[test]
    fn test_strategy_description() {
        let strategy = GeneralizedReciprocity::default();
        assert_eq!(strategy.description(), "グループ全体への貢献と恩恵を考慮する一般化互恵戦略");
    }
}