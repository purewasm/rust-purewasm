use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error(transparent)]
    StdError(#[from] std::io::Error),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    #[error("Wasm error: {0}")]
    WasmError(String)
}