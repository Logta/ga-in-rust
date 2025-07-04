
/// 利用可能な戦略の列挙型
#[derive(Clone, Debug)]
pub enum StrategyType {
    /// 既存の戦略
    RouletteSelection,
    ThresholdSelection,
    
    /// 直接互恵戦略
    TitForTat,
    GenerousTitForTat { forgiveness_rate: f64 },
    Pavlov,
    
    /// 間接互恵戦略
    ReputationBased { cooperation_threshold: f64, update_rate: f64 },
    ImageScoring { cooperation_threshold: f64, initial_cooperation_prob: f64, update_rate: f64 },
    Standing { good_standing_threshold: f64, justify_defection: bool, update_rate: f64 },
}

/// 戦略セレクター
pub struct StrategySelector;

impl StrategySelector {
    /// デフォルトの戦略パラメータを取得
    pub fn get_default_strategy(name: &str) -> Option<StrategyType> {
        match name.to_lowercase().as_str() {
            "roulette" => Some(StrategyType::RouletteSelection),
            "threshold" => Some(StrategyType::ThresholdSelection),
            "tft" | "tit-for-tat" => Some(StrategyType::TitForTat),
            "gtft" | "generous-tft" => Some(StrategyType::GenerousTitForTat { forgiveness_rate: 0.1 }),
            "pavlov" => Some(StrategyType::Pavlov),
            "reputation" => Some(StrategyType::ReputationBased { 
                cooperation_threshold: 0.0, 
                update_rate: 0.2 
            }),
            "image-scoring" => Some(StrategyType::ImageScoring { 
                cooperation_threshold: 0.0, 
                initial_cooperation_prob: 0.5, 
                update_rate: 0.2 
            }),
            "standing" => Some(StrategyType::Standing { 
                good_standing_threshold: 0.0, 
                justify_defection: true, 
                update_rate: 0.2 
            }),
            _ => None,
        }
    }
    
    /// 利用可能な戦略の一覧を取得
    pub fn available_strategies() -> Vec<&'static str> {
        vec![
            "roulette",
            "threshold",
            "tft",
            "gtft",
            "pavlov",
            "reputation",
            "image-scoring",
            "standing",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_strategy_lookup() {
        assert!(StrategySelector::get_default_strategy("tft").is_some());
        assert!(StrategySelector::get_default_strategy("pavlov").is_some());
        assert!(StrategySelector::get_default_strategy("reputation").is_some());
        assert!(StrategySelector::get_default_strategy("unknown").is_none());
    }

    #[test]
    fn test_available_strategies() {
        let strategies = StrategySelector::available_strategies();
        assert!(strategies.contains(&"tft"));
        assert!(strategies.contains(&"pavlov"));
        assert!(strategies.contains(&"reputation"));
        assert!(strategies.contains(&"roulette"));
    }
}