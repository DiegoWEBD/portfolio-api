use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    
    #[error("{0}")]
    NotFoundError(String),
    
    #[error("{0}")]
    AlreadyExistsError(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),
}