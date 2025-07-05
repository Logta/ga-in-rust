/// 一般化互恵戦略（Generalized Reciprocity）
/// 
/// 直接的な互恵関係だけでなく、グループ全体への貢献と
/// グループからの恩恵を考慮して協力判断を行う戦略
use crate::simulation::environment::Choice;
use crate::strategies::Strategy;
use std::collections::HashMap;

/// グループメンバーとの恩恵記録
#[derive(Debug, Clone, PartialEq)]
pub struct GroupBenefit {
    /// このメンバーから受けた恩恵の総量
    pub received_benefits: f64,
    /// このメンバーに与えた恩恵の総量
    pub given_benefits: f64,
}

impl GroupBenefit {
    /// 新しいGroupBenefitを作成
    pub fn new() -> Self {
        Self {
            received_benefits: 0.0,
            given_benefits: 0.0,
        }
    }
    
    /// 恩恵バランス（受けた恩恵 - 与えた恩恵）を計算
    pub fn benefit_balance(&self) -> f64 {
        self.received_benefits - self.given_benefits
    }
}

/// 一般化互恵戦略
#[derive(Debug, Clone)]
pub struct GeneralizedReciprocity {
    /// 協力の閾値（この値以上なら協力）
    cooperation_threshold: f64,
    /// グループメンバーとの恩恵記録（メンバーID -> 恩恵情報）
    group_benefits: HashMap<String, GroupBenefit>,
}

impl GeneralizedReciprocity {
    /// 新しいGeneralizedReciprocity戦略を作成
    pub fn new(cooperation_threshold: f64) -> Self {
        Self {
            cooperation_threshold,
            group_benefits: HashMap::new(),
        }
    }
    
    /// デフォルトの閾値（0.6）でGeneralizedReciprocity戦略を作成
    pub fn default() -> Self {
        Self::new(0.6)
    }
    
    /// 特定のメンバーから恩恵を受けたことを記録
    pub fn record_received_benefit(&mut self, member_id: &str, benefit: f64) {
        let entry = self.group_benefits.entry(member_id.to_string()).or_insert_with(GroupBenefit::new);
        entry.received_benefits += benefit;
    }
    
    /// 特定のメンバーに恩恵を与えたことを記録
    pub fn record_given_benefit(&mut self, member_id: &str, benefit: f64) {
        let entry = self.group_benefits.entry(member_id.to_string()).or_insert_with(GroupBenefit::new);
        entry.given_benefits += benefit;
    }
    
    /// 特定のメンバーとの恩恵記録を取得
    pub fn get_benefit_record(&self, member_id: &str) -> Option<&GroupBenefit> {
        self.group_benefits.get(member_id)
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

    #[test]
    fn test_group_benefit_creation() {
        let benefit = GroupBenefit::new();
        assert_eq!(benefit.received_benefits, 0.0);
        assert_eq!(benefit.given_benefits, 0.0);
        assert_eq!(benefit.benefit_balance(), 0.0);
    }

    #[test]
    fn test_group_benefit_balance_calculation() {
        let mut benefit = GroupBenefit::new();
        benefit.received_benefits = 5.0;
        benefit.given_benefits = 3.0;
        assert_eq!(benefit.benefit_balance(), 2.0);

        benefit.received_benefits = 2.0;
        benefit.given_benefits = 4.0;
        assert_eq!(benefit.benefit_balance(), -2.0);
    }

    #[test]
    fn test_record_received_benefit() {
        let mut strategy = GeneralizedReciprocity::default();
        
        // 初回記録
        strategy.record_received_benefit("member1", 3.0);
        let record = strategy.get_benefit_record("member1").unwrap();
        assert_eq!(record.received_benefits, 3.0);
        assert_eq!(record.given_benefits, 0.0);
        
        // 追加記録
        strategy.record_received_benefit("member1", 2.0);
        let record = strategy.get_benefit_record("member1").unwrap();
        assert_eq!(record.received_benefits, 5.0);
        assert_eq!(record.given_benefits, 0.0);
    }

    #[test]
    fn test_record_given_benefit() {
        let mut strategy = GeneralizedReciprocity::default();
        
        // 初回記録
        strategy.record_given_benefit("member2", 4.0);
        let record = strategy.get_benefit_record("member2").unwrap();
        assert_eq!(record.received_benefits, 0.0);
        assert_eq!(record.given_benefits, 4.0);
        
        // 追加記録
        strategy.record_given_benefit("member2", 1.0);
        let record = strategy.get_benefit_record("member2").unwrap();
        assert_eq!(record.received_benefits, 0.0);
        assert_eq!(record.given_benefits, 5.0);
    }

    #[test]
    fn test_mixed_benefit_records() {
        let mut strategy = GeneralizedReciprocity::default();
        
        // 同じメンバーに対する受恩・与恩の混合
        strategy.record_received_benefit("member3", 3.0);
        strategy.record_given_benefit("member3", 2.0);
        strategy.record_received_benefit("member3", 1.0);
        
        let record = strategy.get_benefit_record("member3").unwrap();
        assert_eq!(record.received_benefits, 4.0);
        assert_eq!(record.given_benefits, 2.0);
        assert_eq!(record.benefit_balance(), 2.0);
    }

    #[test]
    fn test_nonexistent_member_record() {
        let strategy = GeneralizedReciprocity::default();
        assert!(strategy.get_benefit_record("nonexistent").is_none());
    }
}