/// 簡単な評判ベース戦略（プロトタイプ）
/// 
/// 間接互恵の実装例：
/// - 相手の過去の行動から評判スコアを計算
/// - 評判が良ければ協力、悪ければ裏切り
use crate::simulation::environment::Choice;
use crate::strategies::Strategy;

/// 簡単な評判ベース戦略
#[derive(Debug, Clone)]
pub struct SimpleReputation {
    /// 協力の閾値（この値以上なら協力）
    cooperation_threshold: f64,
}

impl SimpleReputation {
    /// 新しいSimpleReputation戦略を作成
    pub fn new(cooperation_threshold: f64) -> Self {
        Self {
            cooperation_threshold,
        }
    }
    
    /// デフォルトの閾値（0.5）でSimpleReputation戦略を作成
    pub fn default() -> Self {
        Self::new(0.5)
    }
    
    /// 履歴から評判スコアを計算
    /// 協力の割合を評判スコアとして使用
    fn calculate_reputation(&self, history: &[(Choice, Choice)]) -> f64 {
        if history.is_empty() {
            return 0.5; // デフォルト評判
        }
        
        let cooperation_count = history.iter()
            .map(|(_, opponent_choice)| match opponent_choice {
                Choice::Cooperate => 1.0,
                Choice::Defect => 0.0,
            })
            .sum::<f64>();
        
        cooperation_count / history.len() as f64
    }
}

impl Strategy for SimpleReputation {
    fn name(&self) -> &str {
        "simple-reputation"
    }
    
    fn description(&self) -> &str {
        "相手の過去の協力率に基づいて行動を決定する評判ベース戦略"
    }
    
    fn decide(&self, history: &[(Choice, Choice)], _round: usize) -> Choice {
        let reputation = self.calculate_reputation(history);
        
        if reputation >= self.cooperation_threshold {
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
    fn test_simple_reputation_creation() {
        let strategy = SimpleReputation::new(0.6);
        assert_eq!(strategy.name(), "simple-reputation");
        assert_eq!(strategy.cooperation_threshold, 0.6);
    }

    #[test]
    fn test_simple_reputation_default() {
        let strategy = SimpleReputation::default();
        assert_eq!(strategy.cooperation_threshold, 0.5);
    }

    #[test]
    fn test_reputation_calculation() {
        let strategy = SimpleReputation::default();
        
        // 空の履歴
        let history = vec![];
        assert_eq!(strategy.calculate_reputation(&history), 0.5);
        
        // 全て協力
        let history = vec![
            (Choice::Cooperate, Choice::Cooperate),
            (Choice::Defect, Choice::Cooperate),
        ];
        assert_eq!(strategy.calculate_reputation(&history), 1.0);
        
        // 半々
        let history = vec![
            (Choice::Cooperate, Choice::Cooperate),
            (Choice::Cooperate, Choice::Defect),
        ];
        assert_eq!(strategy.calculate_reputation(&history), 0.5);
        
        // 全て裏切り
        let history = vec![
            (Choice::Cooperate, Choice::Defect),
            (Choice::Defect, Choice::Defect),
        ];
        assert_eq!(strategy.calculate_reputation(&history), 0.0);
    }

    #[test]
    fn test_decision_making() {
        let strategy = SimpleReputation::new(0.6);
        
        // 良い評判（協力率0.75 > 0.6）-> 協力
        let history = vec![
            (Choice::Cooperate, Choice::Cooperate),
            (Choice::Cooperate, Choice::Cooperate),
            (Choice::Cooperate, Choice::Cooperate),
            (Choice::Cooperate, Choice::Defect),
        ];
        assert_eq!(strategy.decide(&history, 0), Choice::Cooperate);
        
        // 悪い評判（協力率0.25 < 0.6）-> 裏切り
        let history = vec![
            (Choice::Cooperate, Choice::Cooperate),
            (Choice::Cooperate, Choice::Defect),
            (Choice::Cooperate, Choice::Defect),
            (Choice::Cooperate, Choice::Defect),
        ];
        assert_eq!(strategy.decide(&history, 0), Choice::Defect);
    }

    #[test]
    fn test_edge_cases() {
        let strategy = SimpleReputation::new(0.5);
        
        // 空の履歴（初回）
        assert_eq!(strategy.decide(&[], 0), Choice::Cooperate);
        
        // 閾値ちょうど
        let history = vec![
            (Choice::Cooperate, Choice::Cooperate),
            (Choice::Cooperate, Choice::Defect),
        ];
        assert_eq!(strategy.decide(&history, 0), Choice::Cooperate); // 0.5 >= 0.5
    }
}