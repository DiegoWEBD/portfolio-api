use async_trait::async_trait;

use crate::application::errors::AppError;

use super::Document;

#[async_trait]
pub trait DocumentRepository {
    async fn get_all(&self) -> Result<Vec<Document>, AppError>;
}