/// Core types and constants
pub mod types;

/// Core traits and interfaces
pub mod traits;

/// Error types and validation
pub mod errors;

// Re-export commonly used items
pub use errors::{GAError, GAResult};
pub use traits::*;
pub use types::*;
