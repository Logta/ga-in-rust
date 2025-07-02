/// Configuration management
pub mod config;

/// Logging infrastructure
pub mod logging;

// Re-export commonly used items
pub use config::{Config, ConfigBuilder};
