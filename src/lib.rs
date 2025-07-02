/// Core types, traits, and error handling
pub mod core;

/// Genetic algorithm engine and components
pub mod engine;

/// Domain-specific models and logic
pub mod domain;

/// Infrastructure components (config, logging, etc.)
pub mod infrastructure;

/// User interfaces (CLI, API, etc.)
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
