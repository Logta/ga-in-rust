/// 設定管理モジュール
/// 
/// configクレートを活用した高度な設定管理システム

pub mod loader;
pub mod schema;
pub mod validation;

pub use loader::ConfigLoader;
pub use schema::{Config, SimulationConfig, GeneticConfig, OutputConfig, PerformanceConfig};
pub use validation::validate_config;