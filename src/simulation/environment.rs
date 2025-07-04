/// シミュレーション環境の管理
///
/// 囚人のジレンマゲームのルールとペイオフ行列を管理
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use crate::core::validation::*;
use crate::config::schema::PayoffMatrix;

/// 囚人のジレンマの選択
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Choice {
    /// 協力
    Cooperate,
    /// 裏切り
    Defect,
}

impl Choice {
    /// 文字列から選択を作成
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "c" | "cooperate" | "協力" => Ok(Choice::Cooperate),
            "d" | "defect" | "裏切り" => Ok(Choice::Defect),
            _ => anyhow::bail!("無効な選択です: '{}'. 'C'（協力）または'D'（裏切り）を指定してください", s),
        }
    }

    /// 選択を文字に変換
    pub fn to_char(&self) -> char {
        match self {
            Choice::Cooperate => 'C',
            Choice::Defect => 'D',
        }
    }

    /// 選択を日本語文字列に変換
    pub fn to_japanese(&self) -> &'static str {
        match self {
            Choice::Cooperate => "協力",
            Choice::Defect => "裏切り",
        }
    }
}


/// ゲーム環境の設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    /// ペイオフ行列
    pub payoff_matrix: PayoffMatrix,
    
    /// 1試合あたりのラウンド数
    pub rounds_per_match: usize,
    
    /// ランダム性のレベル（0.0-1.0）
    pub noise_level: f64,
    
    /// 情報の完全性（0.0-1.0、1.0=完全情報）
    pub information_completeness: f64,
}

impl Environment {
    /// 新しい環境を作成
    pub fn new(
        payoff_matrix: PayoffMatrix,
        rounds_per_match: usize,
        noise_level: f64,
        information_completeness: f64,
    ) -> Result<Self> {
        validate_game_config(rounds_per_match)
            .context("ゲーム設定の検証に失敗しました")?;

        if !(0.0..=1.0).contains(&noise_level) {
            anyhow::bail!("ノイズレベルは0.0から1.0の間である必要があります: {}", noise_level);
        }

        if !(0.0..=1.0).contains(&information_completeness) {
            anyhow::bail!("情報完全性は0.0から1.0の間である必要があります: {}", information_completeness);
        }

        Ok(Self {
            payoff_matrix,
            rounds_per_match,
            noise_level,
            information_completeness,
        })
    }

    /// 標準的な環境を作成
    pub fn standard() -> Self {
        Self {
            payoff_matrix: PayoffMatrix::standard(),
            rounds_per_match: 10,
            noise_level: 0.0,
            information_completeness: 1.0,
        }
    }

    /// ノイズのある環境を作成
    pub fn with_noise(noise_level: f64) -> Result<Self> {
        let mut env = Self::standard();
        env.noise_level = noise_level;
        env.validate()?;
        Ok(env)
    }

    /// 部分情報の環境を作成
    pub fn with_partial_information(information_completeness: f64) -> Result<Self> {
        let mut env = Self::standard();
        env.information_completeness = information_completeness;
        env.validate()?;
        Ok(env)
    }

    /// 環境設定の妥当性を検証
    pub fn validate(&self) -> Result<()> {
        self.payoff_matrix.validate()
            .context("ペイオフ行列の検証に失敗しました")?;

        validate_game_config(self.rounds_per_match)
            .context("ゲーム設定の検証に失敗しました")?;

        if !(0.0..=1.0).contains(&self.noise_level) {
            anyhow::bail!("ノイズレベルは0.0から1.0の間である必要があります: {}", self.noise_level);
        }

        if !(0.0..=1.0).contains(&self.information_completeness) {
            anyhow::bail!("情報完全性は0.0から1.0の間である必要があります: {}", self.information_completeness);
        }

        Ok(())
    }

    /// 環境の複雑度を計算（0.0-1.0）
    pub fn complexity(&self) -> f64 {
        let payoff_complexity = 1.0 - self.payoff_matrix.cooperation_incentive();
        let rounds_complexity = (self.rounds_per_match as f64 / 100.0).min(1.0);
        let noise_complexity = self.noise_level;
        let info_complexity = 1.0 - self.information_completeness;
        
        (payoff_complexity + rounds_complexity + noise_complexity + info_complexity) / 4.0
    }

    /// 環境の説明を生成
    pub fn description(&self) -> String {
        format!(
            "ゲーム環境:\n\
             {}\n\
             ラウンド数: {} rounds/match\n\
             ノイズレベル: {:.1}%\n\
             情報完全性: {:.1}%\n\
             環境複雑度: {:.1}%\n\
             協力インセンティブ: {:.1}%",
            self.payoff_matrix.description(),
            self.rounds_per_match,
            self.noise_level * 100.0,
            self.information_completeness * 100.0,
            self.complexity() * 100.0,
            self.payoff_matrix.cooperation_incentive() * 100.0
        )
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::standard()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choice_conversions() {
        assert_eq!(Choice::from_str("C").unwrap(), Choice::Cooperate);
        assert_eq!(Choice::from_str("cooperate").unwrap(), Choice::Cooperate);
        assert_eq!(Choice::from_str("協力").unwrap(), Choice::Cooperate);
        
        assert_eq!(Choice::from_str("D").unwrap(), Choice::Defect);
        assert_eq!(Choice::from_str("defect").unwrap(), Choice::Defect);
        assert_eq!(Choice::from_str("裏切り").unwrap(), Choice::Defect);
        
        assert!(Choice::from_str("invalid").is_err());
    }

    #[test]
    fn test_choice_display() {
        assert_eq!(Choice::Cooperate.to_char(), 'C');
        assert_eq!(Choice::Defect.to_char(), 'D');
        
        assert_eq!(Choice::Cooperate.to_japanese(), "協力");
        assert_eq!(Choice::Defect.to_japanese(), "裏切り");
    }

    #[test]
    fn test_payoff_matrix_standard() {
        let matrix = PayoffMatrix::standard();
        matrix.validate().unwrap();
        
        assert_eq!(matrix.payoff(Choice::Cooperate, Choice::Cooperate), (3, 3));
        assert_eq!(matrix.payoff(Choice::Cooperate, Choice::Defect), (0, 5));
        assert_eq!(matrix.payoff(Choice::Defect, Choice::Cooperate), (5, 0));
        assert_eq!(matrix.payoff(Choice::Defect, Choice::Defect), (1, 1));
    }

    #[test]
    fn test_payoff_matrix_validation() {
        // 有効なマトリックス
        let valid = PayoffMatrix::new(3, 5, 0, 1);
        assert!(valid.is_ok());
        
        // 無効なマトリックス（T <= R）
        let invalid1 = PayoffMatrix::new(5, 3, 0, 1);
        assert!(invalid1.is_err());
        
        // 無効なマトリックス（R <= P）
        let invalid2 = PayoffMatrix::new(1, 5, 0, 3);
        assert!(invalid2.is_err());
        
        // 無効なマトリックス（P <= S）
        let invalid3 = PayoffMatrix::new(3, 5, 2, 1);
        assert!(invalid3.is_err());
        
        // 無効なマトリックス（2R <= T + S）
        let invalid4 = PayoffMatrix::new(3, 10, 0, 1);
        assert!(invalid4.is_err());
    }

    #[test]
    fn test_payoff_matrix_variants() {
        let cooperative = PayoffMatrix::cooperative();
        let competitive = PayoffMatrix::competitive();
        
        cooperative.validate().unwrap();
        competitive.validate().unwrap();
        
        // 協力的マトリックスの方が協力インセンティブが高いはず
        assert!(cooperative.cooperation_incentive() > competitive.cooperation_incentive());
    }

    #[test]
    fn test_environment_creation() {
        let env = Environment::standard();
        assert!(env.validate().is_ok());
        
        let env_with_noise = Environment::with_noise(0.1).unwrap();
        assert_eq!(env_with_noise.noise_level, 0.1);
        
        let env_partial = Environment::with_partial_information(0.8).unwrap();
        assert_eq!(env_partial.information_completeness, 0.8);
    }

    #[test]
    fn test_environment_validation() {
        // 無効なノイズレベル
        let result = Environment::new(
            PayoffMatrix::standard(),
            10,
            1.5, // 無効
            1.0,
        );
        assert!(result.is_err());
        
        // 無効な情報完全性
        let result = Environment::new(
            PayoffMatrix::standard(),
            10,
            0.0,
            -0.1, // 無効
        );
        assert!(result.is_err());
        
        // 無効なラウンド数
        let result = Environment::new(
            PayoffMatrix::standard(),
            0, // 無効
            0.0,
            1.0,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_environment_complexity() {
        let simple = Environment::standard();
        let complex = Environment::new(
            PayoffMatrix::competitive(),
            100,
            0.2,
            0.7,
        ).unwrap();
        
        // 複雑な環境の方が複雑度が高いはず
        assert!(complex.complexity() > simple.complexity());
    }

    #[test]
    fn test_descriptions() {
        let matrix = PayoffMatrix::standard();
        let description = matrix.description();
        assert!(description.contains("ペイオフ行列"));
        assert!(description.contains("R(報酬)=3"));
        
        let env = Environment::standard();
        let env_description = env.description();
        assert!(env_description.contains("ゲーム環境"));
        assert!(env_description.contains("ラウンド数: 10"));
    }
}