use crate::application::errors::AppError;

pub fn convert_pg_error(error: tokio_postgres::Error) -> AppError {
    AppError::InternalServerError(error.to_string())
}