use crate::models::model::Model;
use crate::strategies::utils::{RouletteSelectionStrategy, ThresholdSelectionStrategy, StrategyOperation};
use crate::strategies::direct_reciprocity::{TitForTatStrategy, GenerousTitForTatStrategy, PavlovStrategy};
use crate::strategies::indirect_reciprocity::{ReputationBasedStrategy, ImageScoringStrategy, StandingStrategy};
use crate::strategies::strategy_selector::StrategyType;

/// 動的戦略セレクター
/// 実行時に戦略を切り替えることができるラッパー
#[derive(Clone)]
pub enum DynamicStrategy {
    RouletteSelection(RouletteSelectionStrategy),
    ThresholdSelection(ThresholdSelectionStrategy),
    TitForTat(TitForTatStrategy),
    GenerousTitForTat(GenerousTitForTatStrategy),
    Pavlov(PavlovStrategy),
    ReputationBased(ReputationBasedStrategy),
    ImageScoring(ImageScoringStrategy),
    Standing(StandingStrategy),
}

impl DynamicStrategy {
    /// 戦略タイプから動的戦略を作成
    pub fn from_strategy_type(strategy_type: &StrategyType) -> Self {
        match strategy_type {
            StrategyType::RouletteSelection => {
                DynamicStrategy::RouletteSelection(RouletteSelectionStrategy {})
            }
            StrategyType::ThresholdSelection => {
                DynamicStrategy::ThresholdSelection(ThresholdSelectionStrategy {})
            }
            StrategyType::TitForTat => {
                DynamicStrategy::TitForTat(TitForTatStrategy::new())
            }
            StrategyType::GenerousTitForTat { forgiveness_rate } => {
                DynamicStrategy::GenerousTitForTat(GenerousTitForTatStrategy::new_with_forgiveness(*forgiveness_rate))
            }
            StrategyType::Pavlov => {
                DynamicStrategy::Pavlov(PavlovStrategy::new())
            }
            StrategyType::ReputationBased { cooperation_threshold, update_rate } => {
                DynamicStrategy::ReputationBased(ReputationBasedStrategy::new(*cooperation_threshold, *update_rate))
            }
            StrategyType::ImageScoring { cooperation_threshold, initial_cooperation_prob, update_rate } => {
                DynamicStrategy::ImageScoring(ImageScoringStrategy::new(*cooperation_threshold, *initial_cooperation_prob, *update_rate))
            }
            StrategyType::Standing { good_standing_threshold, justify_defection, update_rate } => {
                DynamicStrategy::Standing(StandingStrategy::new(*good_standing_threshold, *justify_defection, *update_rate))
            }
        }
    }
}

impl<T> StrategyOperation<T> for DynamicStrategy
where
    T: Model,
{
    fn play_match(&self, agent1: &T, agent2: &T) -> (T, T) {
        match self {
            DynamicStrategy::RouletteSelection(s) => s.play_match(agent1, agent2),
            DynamicStrategy::ThresholdSelection(s) => s.play_match(agent1, agent2),
            DynamicStrategy::TitForTat(s) => s.play_match(agent1, agent2),
            DynamicStrategy::GenerousTitForTat(s) => s.play_match(agent1, agent2),
            DynamicStrategy::Pavlov(s) => s.play_match(agent1, agent2),
            DynamicStrategy::ReputationBased(s) => s.play_match(agent1, agent2),
            DynamicStrategy::ImageScoring(s) => s.play_match(agent1, agent2),
            DynamicStrategy::Standing(s) => s.play_match(agent1, agent2),
        }
    }
}