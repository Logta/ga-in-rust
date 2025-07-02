/// 遺伝的アルゴリズムエラー処理モジュール
/// 
/// このモジュールでは、遺伝的アルゴリズムの実行中に発生する可能性のある
/// 全てのエラーを定義し、適切なエラーハンドリングを提供しています。

use std::fmt;

/// 遺伝的アルゴリズムで発生する可能性のあるエラー列挙型
/// 
/// Rustのベストプラクティスに従い、エラーの種類を細かく分類し、
/// 各エラーに適切なコンテキスト情報を付与しています。
#[derive(Debug, Clone, PartialEq)]
pub enum GAError {
    // ## 個体群関連エラー
    
    /// 個体群が空の場合のエラー
    /// 
    /// 遺伝的アルゴリズムを実行するためには少なくとも1つの個体が必要
    EmptyPopulation,
    
    /// 無効な個体群サイズエラー
    /// 
    /// 個体群サイズが0または設定可能な範囲外の場合に発生
    InvalidPopulationSize(usize),

    // ## DNA関連エラー
    
    /// 無効なDNA形式エラー
    /// 
    /// DNAに無効な文字が含まれている場合など
    InvalidDna(String),
    
    /// 無効なDNA長エラー
    /// 
    /// DNAの長さが期待される範囲外の場合に発生
    InvalidDnaLength(usize),
    
    /// DNAフォーマットエラー
    /// 
    /// DNAの形式が期待される形式と異なる場合に発生
    InvalidDnaFormat(String),

    // ## 設定関連エラー
    
    /// 無効な突然変異率エラー
    /// 
    /// 突然変異率が0.0-1.0の範囲外の場合に発生
    InvalidMutationRate(f64),
    
    /// 無効な交叉点エラー
    /// 
    /// 交叉点がDNAの長さを超えている場合などに発生
    InvalidCrossoverPoint(usize),
    
    /// 無効なエリートサイズエラー
    /// 
    /// エリートサイズが個体群サイズを超えている場合などに発生
    InvalidEliteSize(usize),
    
    /// 無効な世代数エラー
    /// 
    /// 世代数が無効な値の場合に発生
    InvalidGenerationCount(usize),

    // ## ゲーム関連エラー
    
    /// ゲーム初期化エラー
    /// 
    /// 囚人のジレンマゲームの初期化に失敗した場合
    GameInitializationError(String),
    
    /// ゲーム実行エラー
    /// 
    /// ゲームの実行中に予期しないエラーが発生した場合
    GameExecutionError(String),
    
    /// 無効なゲーム状態エラー
    /// 
    /// ゲームが不正な状態になった場合
    InvalidGameState(String),

    // ## 選択関連エラー
    
    /// 選択処理エラー
    /// 
    /// 個体選択処理中にエラーが発生した場合
    SelectionError(String),
    
    /// 候補不足エラー
    /// 
    /// 選択に必要な候補数が不足している場合
    InsufficientCandidates(usize),

    // ## I/O関連エラー
    
    /// 設定ファイルエラー
    /// 
    /// 設定ファイルの読み込み・書き込みに失敗した場合
    ConfigurationFileError(String),
    
    /// 出力エラー
    /// 
    /// 結果の出力処理に失敗した場合
    OutputError(String),

    // ## 汎用エラー
    
    /// 内部エラー
    /// 
    /// 予期しない内部エラーが発生した場合
    InternalError(String),
    
    /// バリデーションエラー
    /// 
    /// 入力値の検証に失敗した場合
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
