use thiserror::Error;

pub type Result<T> = std::result::Result<T, CliError>;

#[derive(Error, Debug)]
pub enum CliError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::error::Error),
    #[error("{0}")]
    Generic(String),
}

impl CliError {
    pub fn generic(msg: &str) -> CliError {
        CliError::Generic(msg.to_string())
    }
}
