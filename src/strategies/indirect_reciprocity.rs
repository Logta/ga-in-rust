use crate::models::model::{AgentId, Model};
use crate::strategies::utils::{calculate_payoff, Choice, StrategyOperation};
use std::collections::HashMap;

/// 間接互恵戦略の実装
/// 
/// ## 概要
/// 間接互恵とは、直接の相互作用相手ではなく、第三者の評判や
/// 社会的な評価に基づいて協力/裏切りを決定する戦略です。
/// 
/// ## 実装される戦略
/// - **ReputationBased**: 評判スコアに基づく協力判定
/// - **ImageScoring**: 観察による評判管理
/// - **Standing**: 文脈を考慮した道徳的判断
/// 
/// ## 理論的背景
/// 間接互恵は人間社会における協力行動の重要な基盤の一つとされており、
/// 「良い人には親切に、悪い人には厳しく」という社会規範を表現します。

#[derive(Clone)]
pub struct IndirectReciprocityContext {
    /// エージェントの評判スコア
    /// 
    /// # 値域
    /// -1.0（非常に悪い評判）から 1.0（非常に良い評判）
    /// 初期値は 0.0（中立的な評判）
    reputation_scores: HashMap<AgentId, f64>,
    
    /// 評判の更新率（新しい行動の重み）
    /// 
    /// # 値域
    /// 0.0（更新しない）から 1.0（即座に完全更新）
    /// 高い値ほど最新の行動を重視する
    update_rate: f64,
}

impl IndirectReciprocityContext {
    pub fn new(update_rate: f64) -> Self {
        Self {
            reputation_scores: HashMap::new(),
            update_rate: update_rate.clamp(0.0, 1.0),
        }
    }

    /// エージェントの評判を取得
    /// 
    /// # 引数
    /// * `agent_id` - 評判を取得するエージェントのID
    /// 
    /// # 戻り値
    /// 評判スコア（初期値は0.0）
    fn get_reputation(&self, agent_id: AgentId) -> f64 {
        self.reputation_scores.get(&agent_id).copied().unwrap_or(0.0)
    }

    /// 行動に基づいて評判を更新
    /// 
    /// # 引数
    /// * `agent_id` - 評判を更新するエージェントのID
    /// * `choice` - エージェントの選択
    /// * `opponent_reputation` - 相手の評判（行動の評価に影響）
    /// 
    /// # 更新ルール
    /// - 協力: 基本的に良い評価（+1.0）、ただし悪い相手への協力は減点（+0.5）
    /// - 裏切り: 基本的に悪い評価（-1.0）、ただし悪い相手への裏切りは中立（0.0）
    fn update_reputation(&mut self, agent_id: AgentId, choice: &Choice, opponent_reputation: f64) {
        let current_reputation = self.get_reputation(agent_id);
        
        // 行動の評価
        let action_value = match choice {
            Choice::Cooperate => {
                // 協力は基本的に良い評価だが、悪い相手への協力は少し減点
                if opponent_reputation < -0.3 {
                    0.5  // 悪い相手への協力
                } else {
                    1.0  // 通常の協力
                }
            }
            Choice::Defect => {
                // 裏切りは基本的に悪い評価だが、悪い相手への裏切りは正当化される
                if opponent_reputation < -0.3 {
                    0.0   // 悪い相手への裏切り（中立的）
                } else {
                    -1.0  // 良い相手への裏切り
                }
            }
        };
        
        // 指数移動平均で評判を更新
        let new_reputation = current_reputation * (1.0 - self.update_rate) + action_value * self.update_rate;
        self.reputation_scores.insert(agent_id, new_reputation.clamp(-1.0, 1.0));
    }
}

/// 評判ベース戦略
/// 
/// ## 概要
/// 相手の評判スコアに基づいて協力するかどうかを決定する基本的な間接互恵戦略です。
/// 
/// ## 動作原理
/// 1. 相手の評判スコアを確認
/// 2. 評判が閾値以上なら協力、未満なら裏切り
/// 3. 行動後に両者の評判を更新
/// 
/// ## パラメータ
/// - `cooperation_threshold`: 協力する最小評判スコア
/// - `update_rate`: 評判更新の学習率
/// 
/// ## 特徴
/// - **単純明快**: 理解しやすいルール
/// - **社会的**: 評判システムを活用
/// - **適応的**: 評判は動的に更新される
#[derive(Clone)]
pub struct ReputationBasedStrategy {
    context: IndirectReciprocityContext,
    cooperation_threshold: f64,
}

impl ReputationBasedStrategy {
    pub fn new(cooperation_threshold: f64, update_rate: f64) -> Self {
        Self {
            context: IndirectReciprocityContext::new(update_rate),
            cooperation_threshold: cooperation_threshold.clamp(-1.0, 1.0),
        }
    }

    fn get_choice(&self, opponent_id: AgentId) -> Choice {
        let opponent_reputation = self.context.get_reputation(opponent_id);
        
        if opponent_reputation >= self.cooperation_threshold {
            Choice::Cooperate
        } else {
            Choice::Defect
        }
    }
}

impl<T> StrategyOperation<T> for ReputationBasedStrategy
where
    T: Model,
{
    fn play_match(&self, agent1: &T, agent2: &T) -> (T, T) {
        let agent1_id = agent1.get_id();
        let agent2_id = agent2.get_id();

        // 相手の評判に基づいて選択
        let choice1 = self.get_choice(agent2_id);
        let choice2 = self.get_choice(agent1_id);

        let points1 = calculate_payoff(&choice1, &choice2);
        let points2 = calculate_payoff(&choice2, &choice1);

        // 評判を更新
        let mut new_strategy = self.clone();
        let agent1_rep = new_strategy.context.get_reputation(agent1_id);
        let agent2_rep = new_strategy.context.get_reputation(agent2_id);
        
        new_strategy.context.update_reputation(agent1_id, &choice1, agent2_rep);
        new_strategy.context.update_reputation(agent2_id, &choice2, agent1_rep);

        (
            agent1.with_points(agent1.get_points() + points1),
            agent2.with_points(agent2.get_points() + points2),
        )
    }
}

/// イメージスコアリング戦略
/// 自分の観察に基づいて他者の評判を管理
#[derive(Clone)]
pub struct ImageScoringStrategy {
    context: IndirectReciprocityContext,
    /// 協力するための最小評判スコア
    cooperation_threshold: f64,
    /// 初期協力確率（評判が未知の相手に対して）
    initial_cooperation_prob: f64,
}

impl ImageScoringStrategy {
    pub fn new(cooperation_threshold: f64, initial_cooperation_prob: f64, update_rate: f64) -> Self {
        Self {
            context: IndirectReciprocityContext::new(update_rate),
            cooperation_threshold: cooperation_threshold.clamp(-1.0, 1.0),
            initial_cooperation_prob: initial_cooperation_prob.clamp(0.0, 1.0),
        }
    }

    fn get_choice(&self, opponent_id: AgentId) -> Choice {
        // 評判が未知の場合は初期協力確率に従う
        if !self.context.reputation_scores.contains_key(&opponent_id) {
            let mut rng = rand::thread_rng();
            if rand::Rng::gen::<f64>(&mut rng) < self.initial_cooperation_prob {
                return Choice::Cooperate;
            } else {
                return Choice::Defect;
            }
        }

        let opponent_reputation = self.context.get_reputation(opponent_id);
        
        if opponent_reputation >= self.cooperation_threshold {
            Choice::Cooperate
        } else {
            Choice::Defect
        }
    }
}

impl<T> StrategyOperation<T> for ImageScoringStrategy
where
    T: Model,
{
    fn play_match(&self, agent1: &T, agent2: &T) -> (T, T) {
        let agent1_id = agent1.get_id();
        let agent2_id = agent2.get_id();

        let choice1 = self.get_choice(agent2_id);
        let choice2 = self.get_choice(agent1_id);

        let points1 = calculate_payoff(&choice1, &choice2);
        let points2 = calculate_payoff(&choice2, &choice1);

        // 評判を更新（より単純な更新ルール）
        let mut new_strategy = self.clone();
        
        // 協力したら評判上昇、裏切ったら評判低下
        let update1 = if matches!(choice1, Choice::Cooperate) { 1.0 } else { -1.0 };
        let update2 = if matches!(choice2, Choice::Cooperate) { 1.0 } else { -1.0 };
        
        let current1 = new_strategy.context.get_reputation(agent1_id);
        let current2 = new_strategy.context.get_reputation(agent2_id);
        
        let new_rep1 = current1 * (1.0 - new_strategy.context.update_rate) + update1 * new_strategy.context.update_rate;
        let new_rep2 = current2 * (1.0 - new_strategy.context.update_rate) + update2 * new_strategy.context.update_rate;
        
        new_strategy.context.reputation_scores.insert(agent1_id, new_rep1.clamp(-1.0, 1.0));
        new_strategy.context.reputation_scores.insert(agent2_id, new_rep2.clamp(-1.0, 1.0));

        (
            agent1.with_points(agent1.get_points() + points1),
            agent2.with_points(agent2.get_points() + points2),
        )
    }
}

/// スタンディング戦略
/// 行動の文脈を考慮した評判管理
#[derive(Clone)]
pub struct StandingStrategy {
    context: IndirectReciprocityContext,
    /// 良い評判と見なす閾値
    good_standing_threshold: f64,
    /// 悪い相手への裏切りを正当化するかどうか
    justify_defection: bool,
}

impl StandingStrategy {
    pub fn new(good_standing_threshold: f64, justify_defection: bool, update_rate: f64) -> Self {
        Self {
            context: IndirectReciprocityContext::new(update_rate),
            good_standing_threshold: good_standing_threshold.clamp(-1.0, 1.0),
            justify_defection,
        }
    }

    fn is_good_standing(&self, agent_id: AgentId) -> bool {
        self.context.get_reputation(agent_id) >= self.good_standing_threshold
    }

    fn get_choice(&self, opponent_id: AgentId) -> Choice {
        if self.is_good_standing(opponent_id) {
            Choice::Cooperate
        } else {
            Choice::Defect
        }
    }
}

impl<T> StrategyOperation<T> for StandingStrategy
where
    T: Model,
{
    fn play_match(&self, agent1: &T, agent2: &T) -> (T, T) {
        let agent1_id = agent1.get_id();
        let agent2_id = agent2.get_id();

        let choice1 = self.get_choice(agent2_id);
        let choice2 = self.get_choice(agent1_id);

        let points1 = calculate_payoff(&choice1, &choice2);
        let points2 = calculate_payoff(&choice2, &choice1);

        // 評判を更新（文脈を考慮）
        let mut new_strategy = self.clone();
        
        // エージェント1の評判更新
        let reputation_change1 = match (&choice1, new_strategy.is_good_standing(agent2_id)) {
            (Choice::Cooperate, _) => 1.0,  // 協力は常に良い
            (Choice::Defect, false) if new_strategy.justify_defection => 0.0,  // 悪い相手への裏切りは中立
            (Choice::Defect, _) => -1.0,  // その他の裏切りは悪い
        };
        
        // エージェント2の評判更新
        let reputation_change2 = match (&choice2, new_strategy.is_good_standing(agent1_id)) {
            (Choice::Cooperate, _) => 1.0,
            (Choice::Defect, false) if new_strategy.justify_defection => 0.0,
            (Choice::Defect, _) => -1.0,
        };
        
        let current1 = new_strategy.context.get_reputation(agent1_id);
        let current2 = new_strategy.context.get_reputation(agent2_id);
        
        let new_rep1 = current1 * (1.0 - new_strategy.context.update_rate) + reputation_change1 * new_strategy.context.update_rate;
        let new_rep2 = current2 * (1.0 - new_strategy.context.update_rate) + reputation_change2 * new_strategy.context.update_rate;
        
        new_strategy.context.reputation_scores.insert(agent1_id, new_rep1.clamp(-1.0, 1.0));
        new_strategy.context.reputation_scores.insert(agent2_id, new_rep2.clamp(-1.0, 1.0));

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
    fn test_reputation_based_initial_behavior() {
        let strategy = ReputationBasedStrategy::new(0.0, 0.5);
        let agent1 = Agent::new(1, "1111".to_string());
        let agent2 = Agent::new(2, "0000".to_string());

        // 初期評判は0.0なので、閾値0.0なら協力する
        let (updated1, updated2) = strategy.play_match(&agent1, &agent2);

        assert_eq!(updated1.get_points(), 3);
        assert_eq!(updated2.get_points(), 3);
    }

    #[test]
    fn test_image_scoring_unknown_agents() {
        let strategy = ImageScoringStrategy::new(0.0, 1.0, 0.5); // 初期協力率100%
        let agent1 = Agent::new(1, "1111".to_string());
        let agent2 = Agent::new(2, "0000".to_string());

        let (updated1, updated2) = strategy.play_match(&agent1, &agent2);

        // 未知の相手に対して初期協力率100%なので両方協力
        assert_eq!(updated1.get_points(), 3);
        assert_eq!(updated2.get_points(), 3);
    }

    #[test]
    fn test_standing_strategy() {
        let strategy = StandingStrategy::new(0.0, true, 0.5);
        let agent1 = Agent::new(1, "1111".to_string());
        let agent2 = Agent::new(2, "0000".to_string());

        let (updated1, updated2) = strategy.play_match(&agent1, &agent2);

        // 初期評判は0.0で閾値も0.0なので、両方協力
        assert_eq!(updated1.get_points(), 3);
        assert_eq!(updated2.get_points(), 3);
    }
}