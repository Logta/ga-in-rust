/// シミュレーションエンジンモジュール
///
/// 遺伝的アルゴリズムによる囚人のジレンマシミュレーションの実行エンジン

pub mod engine;
pub mod environment;
pub mod runner;
pub mod stats;

pub use engine::Simulation;
pub use environment::{Environment, Choice};
pub use crate::config::schema::PayoffMatrix;
pub use runner::{SimulationRunner, ParameterValue, ComparisonResult};
pub use stats::{SimulationStats, GenerationStats, BestIndividualInfo, ConvergenceInfo, PerformanceInfo};