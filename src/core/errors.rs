/// 遺伝的アルゴリズムのエラー処理
/// 
/// 実行中に発生する可能性のある全てのエラーを定義

use std::fmt;

/// 遺伝的アルゴリズムのエラー型
#[derive(Debug, Clone, PartialEq)]
pub enum GAError {
    /// 個体群が空
    EmptyPopulation,
    /// 無効な個体群サイズ
    InvalidPopulationSize(usize),

    /// 無効なDNA形式
    InvalidDna(String),
    
    /// 無効なDNA長
    InvalidDnaLength(usize),
    /// 無効なDNAフォーマット
    InvalidDnaFormat(String),

    /// 無効な突然変異率
    InvalidMutationRate(f64),
    /// 無効な交叉点
    InvalidCrossoverPoint(usize),
    /// 無効なエリートサイズ
    InvalidEliteSize(usize),
    /// 無効な世代数
    InvalidGenerationCount(usize),

    /// ゲーム初期化エラー
    GameInitializationError(String),
    /// ゲーム実行エラー
    GameExecutionError(String),
    /// 無効なゲーム状態
    InvalidGameState(String),

    /// 選択処理エラー
    SelectionError(String),
    /// 候補不足
    InsufficientCandidates(usize),

    /// 設定ファイルエラー
    ConfigurationFileError(String),
    /// 出力エラー
    OutputError(String),

    /// 内部エラー
    InternalError(String),
    /// バリデーションエラー
    ValidationError(String),
}

impl From<crate::infrastructure::config::ConfigError> for GAError {
    fn from(err: crate::infrastructure::config::ConfigError) -> Self {
        GAError::ValidationError(err.to_string())
    }
}

impl fmt::Display for GAError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GAError::EmptyPopulation => {
                write!(f, "Population cannot be empty")
            }
            GAError::InvalidPopulationSize(size) => {
                write!(f, "Invalid population size: {} (must be > 0)", size)
            }
            GAError::InvalidDna(dna) => {
                write!(f, "Invalid DNA string: '{}'", dna)
            }
            GAError::InvalidDnaLength(length) => {
                write!(f, "Invalid DNA length: {} (must be > 0)", length)
            }
            GAError::InvalidDnaFormat(msg) => {
                write!(f, "Invalid DNA format: {}", msg)
            }
            GAError::InvalidMutationRate(rate) => {
                write!(
                    f,
                    "Invalid mutation rate: {} (must be between 0.0 and 1.0)",
                    rate
                )
            }
            GAError::InvalidCrossoverPoint(point) => {
                write!(f, "Invalid crossover point: {}", point)
            }
            GAError::InvalidEliteSize(size) => {
                write!(
                    f,
                    "Invalid elite size: {} (must be less than population size)",
                    size
                )
            }
            GAError::InvalidGenerationCount(count) => {
                write!(f, "Invalid generation count: {} (must be > 0)", count)
            }
            GAError::GameInitializationError(msg) => {
                write!(f, "Game initialization error: {}", msg)
            }
            GAError::GameExecutionError(msg) => {
                write!(f, "Game execution error: {}", msg)
            }
            GAError::InvalidGameState(msg) => {
                write!(f, "Invalid game state: {}", msg)
            }
            GAError::SelectionError(msg) => {
                write!(f, "Selection error: {}", msg)
            }
            GAError::InsufficientCandidates(count) => {
                write!(f, "Insufficient candidates for selection: {}", count)
            }
            GAError::ConfigurationFileError(msg) => {
                write!(f, "Configuration file error: {}", msg)
            }
            GAError::OutputError(msg) => {
                write!(f, "Output error: {}", msg)
            }
            GAError::InternalError(msg) => {
                write!(f, "Internal error: {}", msg)
            }
            GAError::ValidationError(msg) => {
                write!(f, "Validation error: {}", msg)
            }
        }
    }
}

impl std::error::Error for GAError {}

pub type GAResult<T> = Result<T, GAError>;

/// Utility functions for error handling
pub mod validation {
    use super::*;
    use crate::core::types::*;

    pub fn validate_population_size(size: Population) -> GAResult<()> {
        if size == 0 {
            Err(GAError::InvalidPopulationSize(size))
        } else {
            Ok(())
        }
    }

    pub fn validate_mutation_rate(rate: MutationRate) -> GAResult<()> {
        if rate < 0.0 || rate > 1.0 {
            Err(GAError::InvalidMutationRate(rate))
        } else {
            Ok(())
        }
    }

    pub fn validate_dna(dna: &str) -> GAResult<()> {
        if dna.is_empty() {
            return Err(GAError::InvalidDna("DNA cannot be empty".to_string()));
        }

        if !dna.chars().all(|c| c == '0' || c == '1') {
            return Err(GAError::InvalidDnaFormat(
                "DNA must contain only '0' and '1' characters".to_string(),
            ));
        }

        Ok(())
    }

    pub fn validate_elite_size(elite_size: usize, population_size: Population) -> GAResult<()> {
        if elite_size >= population_size {
            Err(GAError::InvalidEliteSize(elite_size))
        } else {
            Ok(())
        }
    }

    pub fn validate_crossover_point(point: CrossoverPoint, dna_length: usize) -> GAResult<()> {
        if point >= dna_length {
            Err(GAError::InvalidCrossoverPoint(point))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::validation::*;
    use super::*;

    #[test]
    fn test_validate_population_size() {
        assert!(validate_population_size(10).is_ok());
        assert!(validate_population_size(0).is_err());
    }

    #[test]
    fn test_validate_mutation_rate() {
        assert!(validate_mutation_rate(0.5).is_ok());
        assert!(validate_mutation_rate(0.0).is_ok());
        assert!(validate_mutation_rate(1.0).is_ok());
        assert!(validate_mutation_rate(-0.1).is_err());
        assert!(validate_mutation_rate(1.1).is_err());
    }

    #[test]
    fn test_validate_dna() {
        assert!(validate_dna("101010").is_ok());
        assert!(validate_dna("000000").is_ok());
        assert!(validate_dna("111111").is_ok());
        assert!(validate_dna("").is_err());
        assert!(validate_dna("102010").is_err());
        assert!(validate_dna("abcdef").is_err());
    }

    #[test]
    fn test_validate_elite_size() {
        assert!(validate_elite_size(2, 10).is_ok());
        assert!(validate_elite_size(0, 10).is_ok());
        assert!(validate_elite_size(10, 10).is_err());
        assert!(validate_elite_size(15, 10).is_err());
    }

    #[test]
    fn test_validate_crossover_point() {
        assert!(validate_crossover_point(3, 6).is_ok());
        assert!(validate_crossover_point(0, 6).is_ok());
        assert!(validate_crossover_point(5, 6).is_ok());
        assert!(validate_crossover_point(6, 6).is_err());
        assert!(validate_crossover_point(10, 6).is_err());
    }

    #[test]
    fn test_error_display() {
        let error = GAError::InvalidPopulationSize(0);
        assert!(error.to_string().contains("Invalid population size: 0"));

        let error = GAError::InvalidDna("abc".to_string());
        assert!(error.to_string().contains("Invalid DNA string: 'abc'"));
    }

    #[test]
    fn test_error_equality() {
        let error1 = GAError::EmptyPopulation;
        let error2 = GAError::EmptyPopulation;
        let error3 = GAError::InvalidPopulationSize(0);

        assert_eq!(error1, error2);
        assert_ne!(error1, error3);
    }
}
