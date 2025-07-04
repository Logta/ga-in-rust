/// 遺伝的アルゴリズムのコア機能
/// 
/// 基本的な型定義、トレイト、バリデーション機能を提供

/// 基本的な型定義と定数
pub mod types;

/// コアトレイトとインターフェース
pub mod traits;

/// バリデーション関数
pub mod validation;

/// SFMT高速乱数生成器
pub mod random;

/// ロギングシステム
pub mod logging;

// よく使用されるアイテムの再エクスポート
pub use traits::*;
pub use types::*;
pub use validation::*;
pub use random::{RandomGenerator, default_rng, init_default_rng};
pub use logging::{LogConfig, LogLevel, init_logging};
