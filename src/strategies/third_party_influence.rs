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

/// 評判情報の伝達記録
#[derive(Debug, Clone, PartialEq)]
pub struct ReputationMessage {
    /// 送信者のID
    pub sender_id: String,
    /// 評判対象のID
    pub subject_id: String,
    /// 評判スコア（0.0-1.0、0.5がニュートラル）
    pub reputation_score: f64,
    /// メッセージの影響力（0.0-1.0）
    pub influence: f64,
}

impl ReputationMessage {
    /// 新しい評判メッセージを作成
    pub fn new(sender_id: String, subject_id: String, reputation_score: f64, influence: f64) -> Self {
        Self {
            sender_id,
            subject_id,
            reputation_score,
            influence,
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
    /// 受信した評判メッセージ（対象ID -> メッセージのリスト）
    reputation_messages: HashMap<String, Vec<ReputationMessage>>,
}

impl ThirdPartyInfluence {
    /// 新しいThirdPartyInfluence戦略を作成
    pub fn new(cooperation_threshold: f64) -> Self {
        Self {
            cooperation_threshold,
            observations: HashMap::new(),
            reputation_messages: HashMap::new(),
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
    
    /// 評判メッセージを受信
    pub fn receive_reputation_message(&mut self, message: ReputationMessage) {
        let subject_messages = self.reputation_messages.entry(message.subject_id.clone()).or_insert_with(Vec::new);
        subject_messages.push(message);
    }
    
    /// 特定の対象に関する評判メッセージを取得
    pub fn get_reputation_messages(&self, subject_id: &str) -> Option<&Vec<ReputationMessage>> {
        self.reputation_messages.get(subject_id)
    }
    
    /// 評判メッセージから対象の評判スコアを計算
    fn calculate_reputation_score(&self, subject_id: &str) -> f64 {
        let messages = match self.get_reputation_messages(subject_id) {
            Some(msgs) => msgs,
            None => return 0.5, // 評判情報がない場合はニュートラル
        };
        
        if messages.is_empty() {
            return 0.5;
        }
        
        let total_influence: f64 = messages.iter().map(|msg| msg.influence).sum();
        if total_influence == 0.0 {
            return 0.5;
        }
        
        let weighted_score: f64 = messages.iter()
            .map(|msg| msg.reputation_score * msg.influence)
            .sum();
        
        weighted_score / total_influence
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

    #[test]
    fn test_reputation_message_creation() {
        let message = ReputationMessage::new(
            "sender1".to_string(),
            "subject1".to_string(),
            0.8,
            0.9
        );
        assert_eq!(message.sender_id, "sender1");
        assert_eq!(message.subject_id, "subject1");
        assert_eq!(message.reputation_score, 0.8);
        assert_eq!(message.influence, 0.9);
    }

    #[test]
    fn test_receive_reputation_message() {
        let mut strategy = ThirdPartyInfluence::default();
        
        let message = ReputationMessage::new(
            "sender1".to_string(),
            "subject1".to_string(),
            0.7,
            0.8
        );
        
        strategy.receive_reputation_message(message.clone());
        
        let messages = strategy.get_reputation_messages("subject1").unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], message);
    }

    #[test]
    fn test_multiple_reputation_messages_same_subject() {
        let mut strategy = ThirdPartyInfluence::default();
        
        let msg1 = ReputationMessage::new(
            "sender1".to_string(),
            "subject1".to_string(),
            0.8,
            0.7
        );
        
        let msg2 = ReputationMessage::new(
            "sender2".to_string(),
            "subject1".to_string(),
            0.3,
            0.6
        );
        
        strategy.receive_reputation_message(msg1.clone());
        strategy.receive_reputation_message(msg2.clone());
        
        let messages = strategy.get_reputation_messages("subject1").unwrap();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0], msg1);
        assert_eq!(messages[1], msg2);
    }

    #[test]
    fn test_reputation_messages_different_subjects() {
        let mut strategy = ThirdPartyInfluence::default();
        
        let msg1 = ReputationMessage::new(
            "sender1".to_string(),
            "subject1".to_string(),
            0.8,
            0.7
        );
        
        let msg2 = ReputationMessage::new(
            "sender1".to_string(),
            "subject2".to_string(),
            0.3,
            0.6
        );
        
        strategy.receive_reputation_message(msg1);
        strategy.receive_reputation_message(msg2);
        
        assert_eq!(strategy.get_reputation_messages("subject1").unwrap().len(), 1);
        assert_eq!(strategy.get_reputation_messages("subject2").unwrap().len(), 1);
        assert!(strategy.get_reputation_messages("subject3").is_none());
    }

    #[test]
    fn test_calculate_reputation_score_no_messages() {
        let strategy = ThirdPartyInfluence::default();
        assert_eq!(strategy.calculate_reputation_score("unknown"), 0.5);
    }

    #[test]
    fn test_calculate_reputation_score_single_message() {
        let mut strategy = ThirdPartyInfluence::default();
        
        strategy.receive_reputation_message(ReputationMessage::new(
            "sender1".to_string(),
            "subject1".to_string(),
            0.8,
            1.0
        ));
        
        assert_eq!(strategy.calculate_reputation_score("subject1"), 0.8);
    }

    #[test]
    fn test_calculate_reputation_score_weighted_average() {
        let mut strategy = ThirdPartyInfluence::default();
        
        // 評判スコア 0.8、影響力 0.6
        strategy.receive_reputation_message(ReputationMessage::new(
            "sender1".to_string(),
            "subject1".to_string(),
            0.8,
            0.6
        ));
        
        // 評判スコア 0.2、影響力 0.4
        strategy.receive_reputation_message(ReputationMessage::new(
            "sender2".to_string(),
            "subject1".to_string(),
            0.2,
            0.4
        ));
        
        // 重み付き平均 = (0.8 * 0.6 + 0.2 * 0.4) / (0.6 + 0.4) = (0.48 + 0.08) / 1.0 = 0.56
        assert_eq!(strategy.calculate_reputation_score("subject1"), 0.56);
    }
}