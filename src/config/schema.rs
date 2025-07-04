use anyhow::{anyhow, bail, ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// アプリケーション全体の設定
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// シミュレーション設定
    #[serde(default)]
    pub simulation: SimulationConfig,
    
    /// 遺伝的アルゴリズム設定
    #[serde(default)]
    pub genetic: GeneticConfig,
    
    /// 出力設定
    #[serde(default)]
    pub output: OutputConfig,
    
    /// パフォーマンス設定
    #[serde(default)]
    pub performance: PerformanceConfig,
    
    /// カスタム戦略設定
    #[serde(default)]
    pub strategies: HashMap<String, StrategyConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            simulation: SimulationConfig::default(),
            genetic: GeneticConfig::default(),
            output: OutputConfig::default(),
            performance: PerformanceConfig::default(),
            strategies: HashMap::new(),
        }
    }
}

impl Config {
    /// 設定ファイルから読み込み
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| anyhow!("ファイル拡張子が不明: {}", path.display()))?;
        
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("設定ファイルの読み込みに失敗: {}", path.display()))?;
        
        let config: Self = match extension {
            "toml" => toml::from_str(&content)
                .context("TOML形式の設定ファイルのパースに失敗")?,
            "yaml" | "yml" => serde_yaml::from_str(&content)
                .context("YAML形式の設定ファイルのパースに失敗")?,
            "json" => serde_json::from_str(&content)
                .context("JSON形式の設定ファイルのパースに失敗")?,
            _ => bail!("サポートされていないファイル形式: {}", extension),
        };
        
        config.validate()
            .context("設定ファイルの検証に失敗")?;
        
        Ok(config)
    }
    
    /// 設定の妥当性を検証
    pub fn validate(&self) -> Result<()> {
        // 遺伝的パラメータの検証
        self.validate_genetic_params()
            .context("遺伝的パラメータの検証エラー")?;
        
        // シミュレーションパラメータの検証
        self.validate_simulation_params()
            .context("シミュレーションパラメータの検証エラー")?;
        
        // 出力設定の検証
        self.validate_output_params()
            .context("出力設定の検証エラー")?;
        
        Ok(())
    }
    
    fn validate_genetic_params(&self) -> Result<()> {
        ensure!(
            self.genetic.population_size >= 2,
            "個体数は最低2以上必要です（現在: {}）",
            self.genetic.population_size
        );
        
        ensure!(
            self.genetic.generations > 0,
            "世代数は1以上である必要があります"
        );
        
        ensure!(
            (0.0..=1.0).contains(&self.genetic.mutation_rate),
            "突然変異率は0.0から1.0の間である必要があります（現在: {}）",
            self.genetic.mutation_rate
        );
        
        ensure!(
            self.genetic.elite_count < self.genetic.population_size,
            "エリート数({})は個体数({})より小さくなければなりません",
            self.genetic.elite_count,
            self.genetic.population_size
        );
        
        ensure!(
            self.genetic.dna_length > 0 && self.genetic.dna_length <= 64,
            "DNA長は1から64の間である必要があります（現在: {}）",
            self.genetic.dna_length
        );
        
        Ok(())
    }
    
    fn validate_simulation_params(&self) -> Result<()> {
        ensure!(
            self.simulation.rounds_per_match > 0,
            "対戦ラウンド数は1以上である必要があります"
        );
        
        // 戦略の存在確認
        if !self.is_valid_strategy(&self.simulation.default_strategy) {
            bail!(
                "デフォルト戦略 '{}' が見つかりません",
                self.simulation.default_strategy
            );
        }
        
        Ok(())
    }
    
    fn validate_output_params(&self) -> Result<()> {
        ensure!(
            self.output.report_interval > 0,
            "レポート間隔は1以上である必要があります"
        );
        
        if self.output.report_interval > self.genetic.generations {
            bail!(
                "レポート間隔({})が総世代数({})を超えています",
                self.output.report_interval,
                self.genetic.generations
            );
        }
        
        Ok(())
    }
    
    fn is_valid_strategy(&self, name: &str) -> bool {
        // 組み込み戦略
        const BUILTIN_STRATEGIES: &[&str] = &[
            "random", "always-cooperate", "always-defect",
            "tit-for-tat", "generous-tft", "pavlov",
            "reputation", "image-scoring", "standing"
        ];
        
        BUILTIN_STRATEGIES.contains(&name) || self.strategies.contains_key(name)
    }
}

/// シミュレーション設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    /// デフォルトで使用する戦略
    #[serde(default = "default_strategy")]
    pub default_strategy: String,
    
    /// 1対戦あたりのラウンド数
    #[serde(default = "default_rounds")]
    pub rounds_per_match: usize,
    
    /// ペイオフ行列
    #[serde(default)]
    pub payoff_matrix: PayoffMatrix,
    
    /// トーナメント形式
    #[serde(default)]
    pub tournament_type: TournamentType,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            default_strategy: default_strategy(),
            rounds_per_match: default_rounds(),
            payoff_matrix: PayoffMatrix::default(),
            tournament_type: TournamentType::RoundRobin,
        }
    }
}

/// 遺伝的アルゴリズム設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticConfig {
    /// 個体数
    #[serde(default = "default_population")]
    pub population_size: usize,
    
    /// 世代数
    #[serde(default = "default_generations")]
    pub generations: usize,
    
    /// 突然変異率
    #[serde(default = "default_mutation_rate")]
    pub mutation_rate: f64,
    
    /// エリート保存数
    #[serde(default = "default_elite_count")]
    pub elite_count: usize,
    
    /// DNA長
    #[serde(default = "default_dna_length")]
    pub dna_length: usize,
    
    /// 交叉タイプ
    #[serde(default)]
    pub crossover_type: CrossoverType,
    
    /// 選択方式
    #[serde(default)]
    pub selection_method: SelectionMethod,
}

impl GeneticConfig {
    /// 設定の妥当性を検証
    pub fn validate(&self) -> anyhow::Result<()> {
        anyhow::ensure!(self.population_size > 0, "個体数は0より大きくなければなりません");
        anyhow::ensure!(self.generations > 0, "世代数は0より大きくなければなりません");
        anyhow::ensure!(self.mutation_rate >= 0.0 && self.mutation_rate <= 1.0, "突然変異率は0.0から1.0の間でなければなりません");
        anyhow::ensure!(self.elite_count <= self.population_size, "エリート数は個体数以下でなければなりません");
        anyhow::ensure!(self.dna_length > 0, "DNA長は0より大きくなければなりません");
        Ok(())
    }
}

impl Default for GeneticConfig {
    fn default() -> Self {
        Self {
            population_size: default_population(),
            generations: default_generations(),
            mutation_rate: default_mutation_rate(),
            elite_count: default_elite_count(),
            dna_length: default_dna_length(),
            crossover_type: CrossoverType::SinglePoint,
            selection_method: SelectionMethod::Tournament(2),
        }
    }
}

/// 出力設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// レポート間隔（世代数）
    #[serde(default = "default_report_interval")]
    pub report_interval: usize,
    
    /// 詳細ログを出力
    #[serde(default)]
    pub verbose: bool,
    
    /// 結果の自動保存
    #[serde(default)]
    pub auto_save: bool,
    
    /// 保存先ディレクトリ
    #[serde(default = "default_output_dir")]
    pub output_dir: PathBuf,
    
    /// 出力形式
    #[serde(default)]
    pub format: OutputFormat,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            report_interval: default_report_interval(),
            verbose: false,
            auto_save: false,
            output_dir: default_output_dir(),
            format: OutputFormat::Json,
        }
    }
}

/// パフォーマンス設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// 並列処理を有効化
    #[serde(default)]
    pub parallel: bool,
    
    /// スレッド数（0 = 自動）
    #[serde(default)]
    pub num_threads: usize,
    
    /// バッチサイズ
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
    
    /// メモリ使用量の制限（MB）
    #[serde(default)]
    pub memory_limit_mb: Option<usize>,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            parallel: false,
            num_threads: 0,
            batch_size: default_batch_size(),
            memory_limit_mb: None,
        }
    }
}

/// カスタム戦略設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfig {
    /// 戦略の説明
    pub description: String,
    
    /// 戦略パラメータ
    #[serde(default)]
    pub parameters: HashMap<String, serde_json::Value>,
}

/// ペイオフ行列
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoffMatrix {
    /// 両者協力
    #[serde(default = "default_reward")]
    pub reward: i32,
    
    /// 裏切り成功
    #[serde(default = "default_temptation")]
    pub temptation: i32,
    
    /// 裏切られ
    #[serde(default = "default_sucker")]
    pub sucker: i32,
    
    /// 両者裏切り
    #[serde(default = "default_punishment")]
    pub punishment: i32,
}

impl PayoffMatrix {
    /// 2つの選択に基づいてペイオフを計算
    pub fn payoff(&self, player1_choice: crate::simulation::environment::Choice, player2_choice: crate::simulation::environment::Choice) -> (i32, i32) {
        use crate::simulation::environment::Choice;
        match (player1_choice, player2_choice) {
            (Choice::Cooperate, Choice::Cooperate) => (self.reward, self.reward),
            (Choice::Cooperate, Choice::Defect) => (self.sucker, self.temptation),
            (Choice::Defect, Choice::Cooperate) => (self.temptation, self.sucker),
            (Choice::Defect, Choice::Defect) => (self.punishment, self.punishment),
        }
    }

    /// 囚人のジレンマの標準的なペイオフ行列を作成
    pub fn standard() -> Self {
        Self {
            reward: 3,      // 両者協力: 中程度の報酬
            temptation: 5,  // 裏切り成功: 高い報酬
            sucker: 0,      // 裏切られ: 報酬なし
            punishment: 1,  // 両者裏切り: 低い報酬
        }
    }

    /// ペイオフ行列の妥当性を検証
    pub fn validate(&self) -> anyhow::Result<()> {
        // 囚人のジレンマの基本条件: T > R > P > S
        anyhow::ensure!(
            self.temptation > self.reward,
            "誘惑値({})は報酬値({})より大きくなければなりません",
            self.temptation, self.reward
        );

        anyhow::ensure!(
            self.reward > self.punishment,
            "報酬値({})は処罰値({})より大きくなければなりません",
            self.reward, self.punishment
        );

        anyhow::ensure!(
            self.punishment > self.sucker,
            "処罰値({})は愚か者報酬({})より大きくなければなりません",
            self.punishment, self.sucker
        );

        // 追加条件: 2R > T + S (相互協力が交互裏切りより有利)
        anyhow::ensure!(
            2 * self.reward > self.temptation + self.sucker,
            "相互協力の利得(2×{} = {})は交互裏切りの利得({} + {} = {})より大きくなければなりません",
            self.reward, 2 * self.reward,
            self.temptation, self.sucker, self.temptation + self.sucker
        );

        Ok(())
    }

    /// ペイオフ行列の説明を生成
    pub fn description(&self) -> String {
        format!(
            "ペイオフ行列:\n\
             相手    協力  裏切り\n\
             自分 協力  {:2}    {:2}\n\
             　　 裏切り {:2}    {:2}\n\
             \n\
             R(報酬)={}, T(誘惑)={}, S(愚か者)={}, P(処罰)={}",
            self.reward, self.sucker,
            self.temptation, self.punishment,
            self.reward, self.temptation, self.sucker, self.punishment
        )
    }

    /// 協力インセンティブを計算（0.0-1.0、高いほど協力有利）
    pub fn cooperation_incentive(&self) -> f64 {
        let max_gain = self.temptation.max(self.reward).max(self.punishment).max(self.sucker) as f64;
        let min_gain = self.temptation.min(self.reward).min(self.punishment).min(self.sucker) as f64;
        let range = max_gain - min_gain;
        
        if range <= 0.0 {
            return 0.5; // 全て同じ場合は中立
        }
        
        // 協力による期待利得の相対的位置
        ((self.reward as f64 - min_gain) / range).clamp(0.0, 1.0)
    }
}

impl Default for PayoffMatrix {
    fn default() -> Self {
        Self {
            reward: default_reward(),
            temptation: default_temptation(),
            sucker: default_sucker(),
            punishment: default_punishment(),
        }
    }
}

/// トーナメント形式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TournamentType {
    /// 総当たり戦
    RoundRobin,
    /// ランダムペアリング
    RandomPairing(usize),
    /// スイス式
    Swiss,
}

impl Default for TournamentType {
    fn default() -> Self {
        Self::RoundRobin
    }
}

/// 交叉タイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CrossoverType {
    /// 一点交叉
    SinglePoint,
    /// 二点交叉
    TwoPoint,
    /// 一様交叉
    Uniform(f64),
}

impl Default for CrossoverType {
    fn default() -> Self {
        Self::SinglePoint
    }
}

/// 選択方式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SelectionMethod {
    /// ルーレット選択
    Roulette,
    /// トーナメント選択
    Tournament(usize),
    /// ランク選択
    Rank,
    /// エリート選択
    Elite,
}

impl Default for SelectionMethod {
    fn default() -> Self {
        Self::Tournament(2)
    }
}

/// 出力形式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Json,
    Csv,
    Yaml,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Json
    }
}

// デフォルト値関数
fn default_strategy() -> String { "tit-for-tat".to_string() }
fn default_rounds() -> usize { 10 }
fn default_population() -> usize { 100 }
fn default_generations() -> usize { 1000 }
fn default_mutation_rate() -> f64 { 0.01 }
fn default_elite_count() -> usize { 2 }
fn default_dna_length() -> usize { 8 }
fn default_report_interval() -> usize { 100 }
fn default_output_dir() -> PathBuf { PathBuf::from("output") }
fn default_batch_size() -> usize { 100 }
fn default_reward() -> i32 { 3 }
fn default_temptation() -> i32 { 5 }
fn default_sucker() -> i32 { 0 }
fn default_punishment() -> i32 { 1 }