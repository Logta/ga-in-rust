/// 戦略モジュール
/// 
/// 囚人のジレンマゲームにおける各種戦略の実装
/// 各戦略は個別のファイルに分割されている

use crate::simulation::environment::Choice;

// 個別の戦略モジュール
mod always_cooperate;
mod always_defect;
mod tit_for_tat;
mod random;
mod pavlov;
mod generalized_reciprocity;

// 公開する型
pub use always_cooperate::AlwaysCooperate;
pub use always_defect::AlwaysDefect;
pub use tit_for_tat::TitForTat;
pub use random::Random;
pub use pavlov::Pavlov;
pub use generalized_reciprocity::GeneralizedReciprocity;

/// 戦略の基本トレイト
pub trait Strategy {
    /// 戦略の名前を取得
    fn name(&self) -> &str;
    
    /// 戦略の説明を取得
    fn description(&self) -> &str;
    
    /// 次の行動を決定
    fn decide(&self, history: &[(Choice, Choice)], round: usize) -> Choice;
}

/// 基本的な戦略の列挙型（後方互換性のため維持）
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
    /// Pavlov戦略（勝てば続ける、負ければ変える）
    Pavlov,
    /// 一般化互恵戦略
    GeneralizedReciprocity,
}

impl Strategy for BasicStrategy {
    fn name(&self) -> &str {
        match self {
            BasicStrategy::AlwaysCooperate => "always-cooperate",
            BasicStrategy::AlwaysDefect => "always-defect",
            BasicStrategy::TitForTat => "tit-for-tat",
            BasicStrategy::Random => "random",
            BasicStrategy::Pavlov => "pavlov",
            BasicStrategy::GeneralizedReciprocity => "generalized-reciprocity",
        }
    }
    
    fn description(&self) -> &str {
        match self {
            BasicStrategy::AlwaysCooperate => "常に協力する戦略",
            BasicStrategy::AlwaysDefect => "常に裏切る戦略",
            BasicStrategy::TitForTat => "相手の前回の行動を真似る戦略",
            BasicStrategy::Random => "ランダムに行動を選択する戦略",
            BasicStrategy::Pavlov => "勝てば続ける、負ければ変える戦略",
            BasicStrategy::GeneralizedReciprocity => "グループ全体への貢献と恩恵を考慮する一般化互恵戦略",
        }
    }
    
    fn decide(&self, history: &[(Choice, Choice)], round: usize) -> Choice {
        match self {
            BasicStrategy::AlwaysCooperate => AlwaysCooperate.decide(history, round),
            BasicStrategy::AlwaysDefect => AlwaysDefect.decide(history, round),
            BasicStrategy::TitForTat => TitForTat.decide(history, round),
            BasicStrategy::Random => Random.decide(history, round),
            BasicStrategy::Pavlov => Pavlov.decide(history, round),
            BasicStrategy::GeneralizedReciprocity => GeneralizedReciprocity::default().decide(history, round),
        }
    }
}

/// 戦略セレクター
pub struct StrategySelector;

impl StrategySelector {
    /// 利用可能な戦略のリストを取得
    pub fn available_strategies() -> Vec<BasicStrategy> {
        vec![
            BasicStrategy::AlwaysCooperate,
            BasicStrategy::AlwaysDefect,
            BasicStrategy::TitForTat,
            BasicStrategy::Random,
            BasicStrategy::Pavlov,
            BasicStrategy::GeneralizedReciprocity,
        ]
    }
    
    /// 名前から戦略を取得
    pub fn get_strategy(name: &str) -> Option<BasicStrategy> {
        Self::available_strategies()
            .into_iter()
            .find(|s| s.name() == name)
    }
}