/// 遺伝的アルゴリズム囚人のジレンマライブラリ
///
/// このライブラリは、遺伝的アルゴリズムを使用して囚人のジレンマゲームの戦略を
/// 進化させるシミュレーションフレームワークを提供します。
///
/// ## アーキテクチャ
///
/// ライブラリは以下の層構造で設計されています：
///
/// * **Core**: 基本的な型、トレイト、エラーハンドリング
/// * **Engine**: 遺伝的アルゴリズムのエンジンとコンポーネント
/// * **Domain**: ドメイン固有のモデルとロジック
/// * **Infrastructure**: 設定、ログ、I/Oなどのインフラ機能
/// * **Interface**: CLI、API等のユーザーインターフェース
///
/// ## 使用例
///
/// ```rust
/// use ga_prisoners_dilemma::infrastructure::config::Config;
/// use ga_prisoners_dilemma::domain::simulation::Simulation;
///
/// # fn main() -> ga_prisoners_dilemma::core::errors::GAResult<()> {
/// let config = Config::new();
/// let simulation = Simulation::new(config)?;
/// let _result = simulation.run()?;
/// # Ok(())
/// # }
/// ```
/// 基本的な型、トレイト、エラーハンドリング
///
/// 遺伝的アルゴリズムの基盤となる型定義、共通トレイト、
/// エラー処理機能を提供します。
pub mod core;

/// 遺伝的アルゴリズムエンジンとコンポーネント
///
/// 選択戦略、交叉操作、個体群管理など、遺伝的アルゴリズムの
/// 核となる機能を実装します。
pub mod engine;

/// ドメイン固有のモデルとロジック
///
/// 囚人のジレンマゲームとシミュレーション実行に関する
/// ドメインロジックを提供します。
pub mod domain;

/// インフラストラクチャコンポーネント
///
/// 設定管理、ログ出力、ファイルI/Oなど、アプリケーションの
/// 基盤機能を提供します。
pub mod infrastructure;

/// ユーザーインターフェース
///
/// CLI、Web API等のユーザーとのインタラクション機能を
/// 提供します。
pub mod interface;

// Legacy modules for backward compatibility
#[deprecated(note = "Use infrastructure::config instead")]
pub mod config {
    pub use crate::infrastructure::config::*;
}

#[deprecated(note = "Use core::errors instead")]
pub mod error {
    pub use crate::core::errors::*;
}

#[deprecated(note = "Use interface::cli instead")]
pub mod cli {
    pub use crate::interface::cli::*;
}

#[deprecated(note = "Use domain::simulation instead")]
pub mod simulation {
    pub use crate::domain::simulation::*;
}

// Legacy modules that still need to be moved
pub mod ga;
pub mod models;
pub mod strategies;

// Re-export commonly used items
pub use core::{errors::GAResult, traits::*, types::*};
pub use engine::{Population, RankSelection, RouletteSelection, TournamentSelection};
pub use infrastructure::config::Config;
