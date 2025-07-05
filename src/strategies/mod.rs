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
    /// Pavlov戦略（勝てば続ける、負ければ変える）
    Pavlov,
}

impl Strategy for BasicStrategy {
    fn name(&self) -> &str {
        match self {
            BasicStrategy::AlwaysCooperate => "always-cooperate",
            BasicStrategy::AlwaysDefect => "always-defect",
            BasicStrategy::TitForTat => "tit-for-tat",
            BasicStrategy::Random => "random",
            BasicStrategy::Pavlov => "pavlov",
        }
    }
    
    fn description(&self) -> &str {
        match self {
            BasicStrategy::AlwaysCooperate => "常に協力する戦略",
            BasicStrategy::AlwaysDefect => "常に裏切る戦略",
            BasicStrategy::TitForTat => "相手の前回の行動を真似る戦略",
            BasicStrategy::Random => "ランダムに行動を選択する戦略",
            BasicStrategy::Pavlov => "勝てば続ける、負ければ変える戦略",
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
            BasicStrategy::Pavlov => {
                if history.is_empty() {
                    Choice::Cooperate // 初回は協力
                } else {
                    let (my_last, opponent_last) = history.last().unwrap();
                    
                    // 結果を評価（良い結果なら同じ行動、悪い結果なら変更）
                    let good_outcome = match (my_last, opponent_last) {
                        (Choice::Cooperate, Choice::Cooperate) => true, // 相互協力
                        (Choice::Defect, Choice::Cooperate) => true,    // 相手を出し抜く
                        (Choice::Cooperate, Choice::Defect) => false,   // 出し抜かれる
                        (Choice::Defect, Choice::Defect) => false,      // 相互裏切り
                    };
                    
                    if good_outcome {
                        *my_last // 前回と同じ行動を続ける
                    } else {
                        match my_last {
                            Choice::Cooperate => Choice::Defect, // 協力から裏切りに変更
                            Choice::Defect => Choice::Cooperate,  // 裏切りから協力に変更
                        }
                    }
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
            BasicStrategy::Pavlov,
        ]
    }
    
    /// 名前から戦略を取得
    pub fn get_strategy(name: &str) -> Option<BasicStrategy> {
        Self::available_strategies()
            .into_iter()
            .find(|s| s.name() == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::environment::Choice;

    #[test]
    fn test_pavlov_first_round_cooperates() {
        let pavlov = BasicStrategy::Pavlov;
        let history = vec![];
        
        // 初回は協力
        assert_eq!(pavlov.decide(&history, 0), Choice::Cooperate);
    }

    #[test]
    fn test_pavlov_continues_after_good_outcome() {
        let pavlov = BasicStrategy::Pavlov;
        
        // 前回協力して相手も協力（良い結果）→ 協力を続ける
        let history = vec![(Choice::Cooperate, Choice::Cooperate)];
        assert_eq!(pavlov.decide(&history, 1), Choice::Cooperate);
        
        // 前回裏切って相手が協力（良い結果）→ 裏切りを続ける
        let history = vec![(Choice::Defect, Choice::Cooperate)];
        assert_eq!(pavlov.decide(&history, 1), Choice::Defect);
    }

    #[test]
    fn test_pavlov_changes_after_bad_outcome() {
        let pavlov = BasicStrategy::Pavlov;
        
        // 前回協力したが相手が裏切り（悪い結果）→ 裏切りに変更
        let history = vec![(Choice::Cooperate, Choice::Defect)];
        assert_eq!(pavlov.decide(&history, 1), Choice::Defect);
        
        // 前回裏切ったが相手も裏切り（悪い結果）→ 協力に変更
        let history = vec![(Choice::Defect, Choice::Defect)];
        assert_eq!(pavlov.decide(&history, 1), Choice::Cooperate);
    }

    #[test]
    fn test_pavlov_strategy_name() {
        let pavlov = BasicStrategy::Pavlov;
        assert_eq!(pavlov.name(), "pavlov");
    }
}
