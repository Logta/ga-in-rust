/// 遺伝的アルゴリズムの設定管理モジュール
/// 
/// このモジュールでは、遺伝的アルゴリズムの実行に必要な全ての設定パラメータを
/// 管理します。設定の妥当性検証、デフォルト値の提供、ビルダーパターンによる
/// 柔軟な設定構築などの機能を提供します。

use crate::core::types::*;
use std::fmt;

/// 遺伝的アルゴリズムシミュレーションの設定構造体
/// 
/// シミュレーションの実行に必要な全てのパラメータを含みます。
/// 各パラメータは遺伝的アルゴリズムの動作に直接影響するため、
/// 適切な値を設定することが重要です。
/// 
/// # フィールド
/// * `generations` - 実行する世代数
/// * `population` - 各世代の個体数
/// * `mutation_rate` - 突然変異率（0.0-1.0）
/// * `rounds_per_generation` - 世代あたりのゲームラウンド数
/// * `dna_length` - DNA（戦略）の長さ
/// * `report_interval` - 進捗報告の間隔
/// * `elite_size` - エリート保存する個体数
#[derive(Debug, Clone)]
pub struct Config {
    /// 実行する世代数
    /// 
    /// 進化のサイクル回数を決定します。多いほど長時間の進化を観察できますが、
    /// 計算時間も増加します。
    pub generations: usize,
    
    /// 各世代の個体数
    /// 
    /// 遺伝的多様性と計算効率のバランスを決定します。
    /// 多いほど多様性が保たれますが、計算コストが増加します。
    pub population: usize,
    
    /// 突然変異率（0.0-1.0）
    /// 
    /// 新しい遺伝的変異を導入する確率です。
    /// 高すぎると収束が遅く、低すぎると局所最適解に陥りやすくなります。
    pub mutation_rate: f64,
    
    /// 世代あたりのゲームラウンド数
    /// 
    /// 各世代で個体が対戦するゲームの回数です。
    /// 多いほど個体の適応度評価が安定しますが、計算時間が増加します。
    pub rounds_per_generation: usize,
    
    /// DNA（戦略）の長さ
    /// 
    /// 個体の戦略を表現する遺伝子の長さです。
    /// 長いほど複雑な戦略を表現できますが、探索空間が指数的に増加します。
    pub dna_length: usize,
    
    /// 進捗報告の間隔
    /// 
    /// 何世代ごとに進捗状況を出力するかを決定します。
    pub report_interval: usize,
    
    /// エリート保存する個体数
    /// 
    /// 各世代で確実に次世代に引き継がれる優秀な個体の数です。
    /// 多すぎると多様性が失われ、少なすぎると良い解が失われる可能性があります。
    pub elite_size: usize,
}

impl Config {
    /// デフォルト設定で新しいConfigを作成
    /// 
    /// 標準的な実験に適したデフォルト値を使用して設定を初期化します。
    /// より詳細な設定が必要な場合は、ConfigBuilderを使用することを推奨します。
    /// 
    /// # 戻り値
    /// デフォルト値で初期化された新しいConfigインスタンス
    pub fn new() -> Self {
        Self {
            generations: DEFAULT_GENERATIONS,
            population: DEFAULT_POPULATION,
            mutation_rate: DEFAULT_MUTATION_RATE,
            rounds_per_generation: 1,
            dna_length: DEFAULT_DNA_LENGTH,
            report_interval: DEFAULT_REPORT_INTERVAL,
            elite_size: DEFAULT_ELITE_SIZE,
        }
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.population == 0 {
            return Err(ConfigError::InvalidPopulation);
        }
        if self.mutation_rate < 0.0 || self.mutation_rate > 1.0 {
            return Err(ConfigError::InvalidMutationRate);
        }
        if self.dna_length == 0 {
            return Err(ConfigError::InvalidDnaLength);
        }
        if self.elite_size >= self.population {
            return Err(ConfigError::InvalidEliteSize);
        }
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum ConfigError {
    InvalidPopulation,
    InvalidMutationRate,
    InvalidDnaLength,
    InvalidEliteSize,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InvalidPopulation => write!(f, "Population must be greater than 0"),
            ConfigError::InvalidMutationRate => {
                write!(f, "Mutation rate must be between 0.0 and 1.0")
            }
            ConfigError::InvalidDnaLength => write!(f, "DNA length must be greater than 0"),
            ConfigError::InvalidEliteSize => {
                write!(f, "Elite size must be less than population size")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: Config::new(),
        }
    }

    pub fn generations(mut self, generations: usize) -> Self {
        self.config.generations = generations;
        self
    }

    pub fn population(mut self, population: usize) -> Self {
        self.config.population = population;
        self
    }

    pub fn mutation_rate(mut self, rate: f64) -> Self {
        self.config.mutation_rate = rate;
        self
    }

    pub fn dna_length(mut self, length: usize) -> Self {
        self.config.dna_length = length;
        self
    }

    pub fn report_interval(mut self, interval: usize) -> Self {
        self.config.report_interval = interval;
        self
    }

    pub fn elite_size(mut self, size: usize) -> Self {
        self.config.elite_size = size;
        self
    }

    pub fn build(self) -> Result<Config, ConfigError> {
        self.config.validate()?;
        Ok(self.config)
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
