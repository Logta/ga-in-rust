/// 遺伝的アルゴリズム囚人のジレンマライブラリ
///
/// このライブラリは、遺伝的アルゴリズムを使用して囚人のジレンマゲームの戦略を
/// 進化させるシミュレーションフレームワークを提供します。
///
/// ## アーキテクチャ
///
/// ライブラリは以下のモジュール構造で設計されています：
///
/// * **core**: 基本的な型、トレイト、バリデーション、乱数生成、ログ
/// * **cli**: コマンドラインインターフェースと出力機能
/// * **config**: 設定管理とファイル読み込み
/// * **simulation**: シミュレーションエンジンと統計情報
/// * **genetic**: 遺伝的アルゴリズムの実装
/// * **strategies**: 囚人のジレンマ戦略の定義
///
/// ## 使用例
///
/// ```rust,no_run
/// use ga_prisoners_dilemma::config::Config;
/// use ga_prisoners_dilemma::simulation::Simulation;
/// use anyhow::Result;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let config = Config::default();
///     let mut simulation = Simulation::new(config, None)?;
///     let result = simulation.run().await?;
///     println!("{}", result.summary());
///     Ok(())
/// }
/// ```

/// 基本的な型、トレイト、バリデーション、乱数生成、ログ
pub mod core;

/// コマンドラインインターフェースと出力機能
pub mod cli;

/// 設定管理とファイル読み込み
pub mod config;

/// シミュレーションエンジンと統計情報
pub mod simulation;

/// 遺伝的アルゴリズムの実装
pub mod genetic;

/// 囚人のジレンマ戦略の定義
pub mod strategies;

// よく使用されるアイテムの再エクスポート
pub use core::{traits::*, types::*, validation::*, random::*, logging::*};
pub use config::{Config, ConfigLoader};
pub use simulation::{Simulation, SimulationStats, Environment, Choice, PayoffMatrix};
pub use genetic::{Individual, Population, GeneticAlgorithm};
pub use strategies::*;