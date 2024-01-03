use thiserror::Error;
use serde::{Serialize, Deserialize};

#[derive(Error, Debug, Serialize, Deserialize)]
#[error("{code}: {message:?}")]
pub struct  WasmError {
    pub code: String,
    pub message: Option<String>,
}

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error(transparent)]
    StdError(#[from] std::io::Error),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    #[error(transparent)]
    WasmError(#[from] WasmError)
}