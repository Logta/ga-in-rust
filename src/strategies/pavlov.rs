use crate::simulation::environment::Choice;
use super::Strategy;

/// Pavlov戦略（勝てば続ける、負ければ変える）
/// 
/// 良い結果（相互協力または相手を出し抜く）なら同じ行動を続け、
/// 悪い結果（出し抜かれるまたは相互裏切り）なら行動を変更する
#[derive(Clone, Debug)]
pub struct Pavlov;

impl Strategy for Pavlov {
    fn name(&self) -> &str {
        "pavlov"
    }
    
    fn description(&self) -> &str {
        "勝てば続ける、負ければ変える戦略"
    }
    
    fn decide(&self, history: &[(Choice, Choice)], _round: usize) -> Choice {
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pavlov_first_round_cooperates() {
        let pavlov = Pavlov;
        let history = vec![];
        
        // 初回は協力
        assert_eq!(pavlov.decide(&history, 0), Choice::Cooperate);
    }

    #[test]
    fn test_pavlov_continues_after_good_outcome() {
        let pavlov = Pavlov;
        
        // 前回協力して相手も協力（良い結果）→ 協力を続ける
        let history = vec![(Choice::Cooperate, Choice::Cooperate)];
        assert_eq!(pavlov.decide(&history, 1), Choice::Cooperate);
        
        // 前回裏切って相手が協力（良い結果）→ 裏切りを続ける
        let history = vec![(Choice::Defect, Choice::Cooperate)];
        assert_eq!(pavlov.decide(&history, 1), Choice::Defect);
    }

    #[test]
    fn test_pavlov_changes_after_bad_outcome() {
        let pavlov = Pavlov;
        
        // 前回協力したが相手が裏切り（悪い結果）→ 裏切りに変更
        let history = vec![(Choice::Cooperate, Choice::Defect)];
        assert_eq!(pavlov.decide(&history, 1), Choice::Defect);
        
        // 前回裏切ったが相手も裏切り（悪い結果）→ 協力に変更
        let history = vec![(Choice::Defect, Choice::Defect)];
        assert_eq!(pavlov.decide(&history, 1), Choice::Cooperate);
    }

    #[test]
    fn test_pavlov_strategy_name() {
        let pavlov = Pavlov;
        assert_eq!(pavlov.name(), "pavlov");
    }
}