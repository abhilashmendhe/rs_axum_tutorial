use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyAppError {

    #[error("IOerror: {0}")]
    IOError(#[from] std::io::Error)
}