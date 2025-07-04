/// 簡略化された戦略モジュール
/// 
/// 現在は基本的な戦略のみ提供。
/// 詳細な実装は後で追加予定。

use crate::simulation::environment::Choice;

/// 戦略の基本トレイト
pub trait Strategy {
    /// 戦略の名前を取得
    fn name(&self) -> &str;
    
    /// 戦略の説明を取得
    fn description(&self) -> &str;
    
    /// 次の行動を決定
    fn decide(&self, history: &[(Choice, Choice)], round: usize) -> Choice;
}

/// 基本的な戦略実装
#[derive(Clone, Debug)]
pub enum BasicStrategy {
    /// 常に協力
    AlwaysCooperate,
    /// 常に裏切り
    AlwaysDefect,
    /// しっぺ返し戦略
    TitForTat,
    /// ランダム戦略
    Random,
}

impl Strategy for BasicStrategy {
    fn name(&self) -> &str {
        match self {
            BasicStrategy::AlwaysCooperate => "always-cooperate",
            BasicStrategy::AlwaysDefect => "always-defect",
            BasicStrategy::TitForTat => "tit-for-tat",
            BasicStrategy::Random => "random",
        }
    }
    
    fn description(&self) -> &str {
        match self {
            BasicStrategy::AlwaysCooperate => "常に協力する戦略",
            BasicStrategy::AlwaysDefect => "常に裏切る戦略",
            BasicStrategy::TitForTat => "相手の前回の行動を真似る戦略",
            BasicStrategy::Random => "ランダムに行動を選択する戦略",
        }
    }
    
    fn decide(&self, history: &[(Choice, Choice)], _round: usize) -> Choice {
        use crate::core::random::utils;
        
        match self {
            BasicStrategy::AlwaysCooperate => Choice::Cooperate,
            BasicStrategy::AlwaysDefect => Choice::Defect,
            BasicStrategy::TitForTat => {
                if history.is_empty() {
                    Choice::Cooperate // 初回は協力
                } else {
                    history.last().unwrap().1 // 相手の前回の行動を真似る
                }
            }
            BasicStrategy::Random => {
                if utils::random_bool(0.5).unwrap_or(false) {
                    Choice::Cooperate
                } else {
                    Choice::Defect
                }
            }
        }
    }
}

/// 戦略セレクター（簡略版）
pub struct StrategySelector;

impl StrategySelector {
    /// 利用可能な戦略のリストを取得
    pub fn available_strategies() -> Vec<BasicStrategy> {
        vec![
            BasicStrategy::AlwaysCooperate,
            BasicStrategy::AlwaysDefect,
            BasicStrategy::TitForTat,
            BasicStrategy::Random,
        ]
    }
    
    /// 名前から戦略を取得
    pub fn get_strategy(name: &str) -> Option<BasicStrategy> {
        Self::available_strategies()
            .into_iter()
            .find(|s| s.name() == name)
    }
}
