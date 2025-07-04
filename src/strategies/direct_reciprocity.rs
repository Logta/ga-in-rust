use crate::models::model::Model;
use crate::strategies::utils::{calculate_payoff, Choice, StrategyOperation};
use std::collections::HashMap;

/// 直接互恵戦略の実装
/// 
/// 直接互恵とは、同じ相手との繰り返し相互作用において、
/// 相手の過去の行動に基づいて自分の行動を決定する戦略です。
/// このモジュールでは以下の直接互恵戦略を実装しています：
/// 
/// - **Tit-for-Tat (TFT)**: 相手の前回の行動を真似る
/// - **Generous Tit-for-Tat (GTFT)**: TFTに寛容性を追加
/// - **Pavlov**: 前回の結果に基づいて行動を変更する

#[derive(Clone)]
pub struct DirectReciprocityContext {
    /// エージェント間の相互作用履歴を記録
    /// 
    /// # データ構造
    /// - キー: (エージェント1のID, エージェント2のID)
    /// - 値: (エージェント1の選択履歴, エージェント2の選択履歴)
    /// 
    /// # 注意
    /// キーは常に小さいIDを最初にソートして格納されます
    history: HashMap<(u64, u64), (Vec<Choice>, Vec<Choice>)>,
}

impl DirectReciprocityContext {
    pub fn new() -> Self {
        Self {
            history: HashMap::new(),
        }
    }

    /// 2つのエージェント間の履歴を取得
    /// 
    /// # 引数
    /// * `agent1_id` - エージェント1のID
    /// * `agent2_id` - エージェント2のID
    /// 
    /// # 戻り値
    /// Some((agent1の履歴, agent2の履歴)) または None（履歴が存在しない場合）
    fn get_history(&self, agent1_id: u64, agent2_id: u64) -> Option<(&Vec<Choice>, &Vec<Choice>)> {
        if let Some((hist1, hist2)) = self.history.get(&(agent1_id, agent2_id)) {
            Some((hist1, hist2))
        } else if let Some((hist2, hist1)) = self.history.get(&(agent2_id, agent1_id)) {
            Some((hist1, hist2))
        } else {
            None
        }
    }

    /// 相互作用の履歴を記録
    /// 
    /// # 引数
    /// * `agent1_id` - エージェント1のID
    /// * `agent2_id` - エージェント2のID
    /// * `choice1` - エージェント1の選択
    /// * `choice2` - エージェント2の選択
    fn record_interaction(&mut self, agent1_id: u64, agent2_id: u64, choice1: Choice, choice2: Choice) {
        let key = if agent1_id < agent2_id {
            (agent1_id, agent2_id)
        } else {
            (agent2_id, agent1_id)
        };

        let (hist1, hist2) = self.history.entry(key).or_insert((Vec::new(), Vec::new()));
        if agent1_id < agent2_id {
            hist1.push(choice1);
            hist2.push(choice2);
        } else {
            hist1.push(choice2);
            hist2.push(choice1);
        }
    }
}

/// Tit-for-Tat戦略
/// 
/// ## 概要
/// Tit-for-Tat（しっぺ返し戦略）は直接互恵の最も基本的な戦略です。
/// 
/// ## 動作原理
/// 1. 初回の相互作用では必ず協力する
/// 2. 2回目以降は相手の前回の行動を真似る
///    - 相手が協力したら協力する
///    - 相手が裏切ったら裏切る
/// 
/// ## 特徴
/// - **親切**: 初回は協力する
/// - **応報性**: 裏切りには裏切りで対応
/// - **寛容性**: 相手が協力に戻れば即座に協力
/// - **明快性**: 戦略が理解しやすい
#[derive(Clone)]
pub struct TitForTatStrategy {
    context: DirectReciprocityContext,
}

impl TitForTatStrategy {
    pub fn new() -> Self {
        Self {
            context: DirectReciprocityContext::new(),
        }
    }

    /// エージェントの次の選択を決定
    /// 
    /// # 引数
    /// * `agent_id` - 行動するエージェントのID
    /// * `opponent_id` - 相手エージェントのID
    /// 
    /// # 戻り値
    /// Choice::Cooperate または Choice::Defect
    fn get_choice(&self, agent_id: u64, opponent_id: u64) -> Choice {
        match self.context.get_history(agent_id, opponent_id) {
            Some((my_hist, opp_hist)) => {
                // 相手の最後の選択を真似る
                if agent_id < opponent_id {
                    opp_hist.last().copied().unwrap_or(Choice::Cooperate)
                } else {
                    my_hist.last().copied().unwrap_or(Choice::Cooperate)
                }
            }
            None => Choice::Cooperate, // 初回は協力
        }
    }
}

impl<T> StrategyOperation<T> for TitForTatStrategy
where
    T: Model,
{
    fn play_match(&self, agent1: &T, agent2: &T) -> (T, T) {
        let agent1_id = agent1.get_id();
        let agent2_id = agent2.get_id();

        let choice1 = self.get_choice(agent1_id, agent2_id);
        let choice2 = self.get_choice(agent2_id, agent1_id);

        let points1 = calculate_payoff(&choice1, &choice2);
        let points2 = calculate_payoff(&choice2, &choice1);

        // 履歴を記録
        let mut new_strategy = self.clone();
        new_strategy.context.record_interaction(agent1_id, agent2_id, choice1, choice2);

        (
            agent1.with_points(agent1.get_points() + points1),
            agent2.with_points(agent2.get_points() + points2),
        )
    }
}

/// Generous Tit-for-Tat戦略
/// 
/// ## 概要
/// Generous Tit-for-Tat（寛容なしっぺ返し戦略）はTFTに寛容性を追加した戦略です。
/// 
/// ## 動作原理
/// 1. 基本的にはTFTと同じ動作
/// 2. 相手が裏切った場合、一定確率で許して協力する
/// 3. これにより相互裏切りの連鎖を断ち切る
/// 
/// ## 利点
/// - **エラー回復**: 誤解やノイズから回復しやすい
/// - **協力促進**: 相互協力状態に戻りやすい
/// - **頑健性**: 不完全な情報下でも有効
/// 
/// ## パラメータ
/// - `forgiveness_rate`: 裏切りを許す確率（0.0〜1.0）
#[derive(Clone)]
pub struct GenerousTitForTatStrategy {
    context: DirectReciprocityContext,
    forgiveness_rate: f64,
}

impl GenerousTitForTatStrategy {
    pub fn new_with_forgiveness(forgiveness_rate: f64) -> Self {
        Self {
            context: DirectReciprocityContext::new(),
            forgiveness_rate,
        }
    }

    fn get_choice(&self, agent_id: u64, opponent_id: u64) -> Choice {
        match self.context.get_history(agent_id, opponent_id) {
            Some((my_hist, opp_hist)) => {
                let opponent_last = if agent_id < opponent_id {
                    opp_hist.last().copied().unwrap_or(Choice::Cooperate)
                } else {
                    my_hist.last().copied().unwrap_or(Choice::Cooperate)
                };

                match opponent_last {
                    Choice::Cooperate => Choice::Cooperate,
                    Choice::Defect => {
                        // 一定確率で許す
                        let mut rng = rand::thread_rng();
                        if rand::Rng::gen::<f64>(&mut rng) < self.forgiveness_rate {
                            Choice::Cooperate
                        } else {
                            Choice::Defect
                        }
                    }
                }
            }
            None => Choice::Cooperate, // 初回は協力
        }
    }
}

impl<T> StrategyOperation<T> for GenerousTitForTatStrategy
where
    T: Model,
{
    fn play_match(&self, agent1: &T, agent2: &T) -> (T, T) {
        let agent1_id = agent1.get_id();
        let agent2_id = agent2.get_id();

        let choice1 = self.get_choice(agent1_id, agent2_id);
        let choice2 = self.get_choice(agent2_id, agent1_id);

        let points1 = calculate_payoff(&choice1, &choice2);
        let points2 = calculate_payoff(&choice2, &choice1);

        // 履歴を記録
        let mut new_strategy = self.clone();
        new_strategy.context.record_interaction(agent1_id, agent2_id, choice1, choice2);

        (
            agent1.with_points(agent1.get_points() + points1),
            agent2.with_points(agent2.get_points() + points2),
        )
    }
}

/// Pavlov戦略（Win-Stay, Lose-Shift）
/// 
/// ## 概要
/// Pavlov戦略は前回の結果に基づいて行動を決定する学習型戦略です。
/// 心理学のパブロフの犬の実験にちなんで名付けられました。
/// 
/// ## 動作原理
/// 1. 前回の対戦結果を評価する
/// 2. 結果が良ければ（高得点）同じ行動を継続（Win-Stay）
/// 3. 結果が悪ければ（低得点）行動を変更（Lose-Shift）
/// 
/// ## 判定基準
/// - 得点が3以上（相互協力または一方的搾取）→ 同じ行動を継続
/// - 得点が3未満（相互裏切りまたは一方的被搾取）→ 行動を変更
/// 
/// ## 特徴
/// - **適応性**: 環境に応じて学習する
/// - **自己修正**: 悪い結果から自動的に回復
/// - **シンプル**: 理解しやすいルール
#[derive(Clone)]
pub struct PavlovStrategy {
    context: DirectReciprocityContext,
}

impl PavlovStrategy {
    pub fn new() -> Self {
        Self {
            context: DirectReciprocityContext::new(),
        }
    }

    fn get_choice(&self, agent_id: u64, opponent_id: u64) -> Choice {
        match self.context.get_history(agent_id, opponent_id) {
            Some((my_hist, opp_hist)) => {
                let (my_last, opp_last) = if agent_id < opponent_id {
                    (
                        my_hist.last().copied().unwrap_or(Choice::Cooperate),
                        opp_hist.last().copied().unwrap_or(Choice::Cooperate),
                    )
                } else {
                    (
                        opp_hist.last().copied().unwrap_or(Choice::Cooperate),
                        my_hist.last().copied().unwrap_or(Choice::Cooperate),
                    )
                };

                // 前回の利得を計算
                let last_payoff = calculate_payoff(&my_last, &opp_last);

                // 利得が3以上なら同じ選択を、それ以下なら変更
                if last_payoff >= 3 {
                    my_last
                } else {
                    match my_last {
                        Choice::Cooperate => Choice::Defect,
                        Choice::Defect => Choice::Cooperate,
                    }
                }
            }
            None => Choice::Cooperate, // 初回は協力
        }
    }
}

impl<T> StrategyOperation<T> for PavlovStrategy
where
    T: Model,
{
    fn play_match(&self, agent1: &T, agent2: &T) -> (T, T) {
        let agent1_id = agent1.get_id();
        let agent2_id = agent2.get_id();

        let choice1 = self.get_choice(agent1_id, agent2_id);
        let choice2 = self.get_choice(agent2_id, agent1_id);

        let points1 = calculate_payoff(&choice1, &choice2);
        let points2 = calculate_payoff(&choice2, &choice1);

        // 履歴を記録
        let mut new_strategy = self.clone();
        new_strategy.context.record_interaction(agent1_id, agent2_id, choice1, choice2);

        (
            agent1.with_points(agent1.get_points() + points1),
            agent2.with_points(agent2.get_points() + points2),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::model::Agent;

    #[test]
    fn test_tit_for_tat_initial_cooperation() {
        let strategy = TitForTatStrategy::new();
        let agent1 = Agent::new(1, "1111".to_string());
        let agent2 = Agent::new(2, "0000".to_string());

        let (updated1, updated2) = strategy.play_match(&agent1, &agent2);

        // 初回は両方協力するので、両者3ポイント
        assert_eq!(updated1.get_points(), 3);
        assert_eq!(updated2.get_points(), 3);
    }

    #[test]
    fn test_generous_tft_forgiveness() {
        let strategy = GenerousTitForTatStrategy::new_with_forgiveness(1.0); // 100%許す
        let agent1 = Agent::new(1, "1111".to_string());
        let agent2 = Agent::new(2, "0000".to_string());

        let (updated1, updated2) = strategy.play_match(&agent1, &agent2);

        // 初回は両方協力
        assert_eq!(updated1.get_points(), 3);
        assert_eq!(updated2.get_points(), 3);
    }

    #[test]
    fn test_pavlov_strategy() {
        let strategy = PavlovStrategy::new();
        let agent1 = Agent::new(1, "1111".to_string());
        let agent2 = Agent::new(2, "0000".to_string());

        let (updated1, updated2) = strategy.play_match(&agent1, &agent2);

        // 初回は両方協力
        assert_eq!(updated1.get_points(), 3);
        assert_eq!(updated2.get_points(), 3);
    }
}