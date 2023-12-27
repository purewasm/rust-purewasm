use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error(transparent)]
    StdError(#[from] std::io::Error),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    #[error("None error")]
    NoneError,
    #[error("{0}")]
    Other(String),
}