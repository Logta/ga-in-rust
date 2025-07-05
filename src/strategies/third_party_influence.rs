/// 第三者影響戦略（Third-Party Influence）
/// 
/// 直接的な相互作用だけでなく、第三者からの評判情報と
/// 社会的圧力を考慮して協力判断を行う戦略
use crate::simulation::environment::Choice;
use crate::strategies::Strategy;
use std::collections::HashMap;

/// 第三者からの観察記録
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationRecord {
    /// 観察者のID
    pub observer_id: String,
    /// 観察された行動
    pub observed_action: Choice,
    /// 観察された相手のID
    pub target_id: String,
    /// 観察の信頼度（0.0-1.0）
    pub credibility: f64,
}

impl ObservationRecord {
    /// 新しい観察記録を作成
    pub fn new(observer_id: String, observed_action: Choice, target_id: String, credibility: f64) -> Self {
        Self {
            observer_id,
            observed_action,
            target_id,
            credibility,
        }
    }
}

/// 第三者影響戦略
#[derive(Debug, Clone)]
pub struct ThirdPartyInfluence {
    /// 協力の閾値（この値以上なら協力）
    cooperation_threshold: f64,
    /// 第三者からの観察記録（対象ID -> 観察記録のリスト）
    observations: HashMap<String, Vec<ObservationRecord>>,
}

impl ThirdPartyInfluence {
    /// 新しいThirdPartyInfluence戦略を作成
    pub fn new(cooperation_threshold: f64) -> Self {
        Self {
            cooperation_threshold,
            observations: HashMap::new(),
        }
    }
    
    /// デフォルトの閾値（0.5）でThirdPartyInfluence戦略を作成
    pub fn default() -> Self {
        Self::new(0.5)
    }
    
    /// 第三者からの観察情報を追加
    pub fn add_observation(&mut self, observation: ObservationRecord) {
        let target_observations = self.observations.entry(observation.target_id.clone()).or_insert_with(Vec::new);
        target_observations.push(observation);
    }
    
    /// 特定の対象に関する観察記録を取得
    pub fn get_observations(&self, target_id: &str) -> Option<&Vec<ObservationRecord>> {
        self.observations.get(target_id)
    }
    
    /// 観察記録から対象の協力率を計算
    fn calculate_cooperation_rate(&self, target_id: &str) -> f64 {
        let observations = match self.get_observations(target_id) {
            Some(obs) => obs,
            None => return 0.5, // 情報がない場合はニュートラル
        };
        
        if observations.is_empty() {
            return 0.5;
        }
        
        let total_weight: f64 = observations.iter().map(|obs| obs.credibility).sum();
        if total_weight == 0.0 {
            return 0.5;
        }
        
        let cooperation_weight: f64 = observations.iter()
            .filter(|obs| obs.observed_action == Choice::Cooperate)
            .map(|obs| obs.credibility)
            .sum();
        
        cooperation_weight / total_weight
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

    #[test]
    fn test_observation_record_creation() {
        let record = ObservationRecord::new(
            "observer1".to_string(),
            Choice::Cooperate,
            "target1".to_string(),
            0.8
        );
        assert_eq!(record.observer_id, "observer1");
        assert_eq!(record.observed_action, Choice::Cooperate);
        assert_eq!(record.target_id, "target1");
        assert_eq!(record.credibility, 0.8);
    }

    #[test]
    fn test_add_observation() {
        let mut strategy = ThirdPartyInfluence::default();
        
        let observation = ObservationRecord::new(
            "observer1".to_string(),
            Choice::Cooperate,
            "target1".to_string(),
            0.9
        );
        
        strategy.add_observation(observation.clone());
        
        let observations = strategy.get_observations("target1").unwrap();
        assert_eq!(observations.len(), 1);
        assert_eq!(observations[0], observation);
    }

    #[test]
    fn test_multiple_observations_same_target() {
        let mut strategy = ThirdPartyInfluence::default();
        
        let obs1 = ObservationRecord::new(
            "observer1".to_string(),
            Choice::Cooperate,
            "target1".to_string(),
            0.8
        );
        
        let obs2 = ObservationRecord::new(
            "observer2".to_string(),
            Choice::Defect,
            "target1".to_string(),
            0.7
        );
        
        strategy.add_observation(obs1.clone());
        strategy.add_observation(obs2.clone());
        
        let observations = strategy.get_observations("target1").unwrap();
        assert_eq!(observations.len(), 2);
        assert_eq!(observations[0], obs1);
        assert_eq!(observations[1], obs2);
    }

    #[test]
    fn test_observations_different_targets() {
        let mut strategy = ThirdPartyInfluence::default();
        
        let obs1 = ObservationRecord::new(
            "observer1".to_string(),
            Choice::Cooperate,
            "target1".to_string(),
            0.8
        );
        
        let obs2 = ObservationRecord::new(
            "observer1".to_string(),
            Choice::Defect,
            "target2".to_string(),
            0.7
        );
        
        strategy.add_observation(obs1);
        strategy.add_observation(obs2);
        
        assert_eq!(strategy.get_observations("target1").unwrap().len(), 1);
        assert_eq!(strategy.get_observations("target2").unwrap().len(), 1);
        assert!(strategy.get_observations("target3").is_none());
    }

    #[test]
    fn test_calculate_cooperation_rate_no_observations() {
        let strategy = ThirdPartyInfluence::default();
        assert_eq!(strategy.calculate_cooperation_rate("unknown"), 0.5);
    }

    #[test]
    fn test_calculate_cooperation_rate_all_cooperate() {
        let mut strategy = ThirdPartyInfluence::default();
        
        strategy.add_observation(ObservationRecord::new(
            "obs1".to_string(), Choice::Cooperate, "target".to_string(), 0.8
        ));
        strategy.add_observation(ObservationRecord::new(
            "obs2".to_string(), Choice::Cooperate, "target".to_string(), 0.6
        ));
        
        // 全て協力 -> 1.0
        assert_eq!(strategy.calculate_cooperation_rate("target"), 1.0);
    }

    #[test]
    fn test_calculate_cooperation_rate_mixed() {
        let mut strategy = ThirdPartyInfluence::default();
        
        // 協力: 信頼度 0.8
        strategy.add_observation(ObservationRecord::new(
            "obs1".to_string(), Choice::Cooperate, "target".to_string(), 0.8
        ));
        // 裏切り: 信頼度 0.4
        strategy.add_observation(ObservationRecord::new(
            "obs2".to_string(), Choice::Defect, "target".to_string(), 0.4
        ));
        
        // 協力率 = 0.8 / (0.8 + 0.4) = 2/3 ≈ 0.6667
        let rate = strategy.calculate_cooperation_rate("target");
        assert!((rate - 0.6666666666666666).abs() < 0.0001);
    }
}