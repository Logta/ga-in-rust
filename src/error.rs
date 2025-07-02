use std::fmt;

#[derive(Debug, Clone)]
pub enum GAError {
    EmptyPopulation,
    InvalidDna(String),
    InvalidChoice(String),
    SelectionError(String),
    GameError(String),
    ConfigError(crate::config::ConfigError),
}

impl fmt::Display for GAError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GAError::EmptyPopulation => write!(f, "Population cannot be empty"),
            GAError::InvalidDna(msg) => write!(f, "Invalid DNA: {}", msg),
            GAError::InvalidChoice(msg) => write!(f, "Invalid choice: {}", msg),
            GAError::SelectionError(msg) => write!(f, "Selection error: {}", msg),
            GAError::GameError(msg) => write!(f, "Game error: {}", msg),
            GAError::ConfigError(err) => write!(f, "Configuration error: {}", err),
        }
    }
}

impl std::error::Error for GAError {}

impl From<crate::config::ConfigError> for GAError {
    fn from(err: crate::config::ConfigError) -> Self {
        GAError::ConfigError(err)
    }
}

pub type GAResult<T> = Result<T, GAError>;
