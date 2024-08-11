use std::sync::Arc;

use crate::{application::errors::AppError, domain::document::{document_repository::DocumentRepository, Document}};

#[derive(Clone)]
pub struct DocumentServices {
    repository: Arc<dyn DocumentRepository + Send + Sync>
}

impl DocumentServices {

    pub fn new(repository: Arc<dyn DocumentRepository + Send + Sync>) -> Self {
        DocumentServices { repository }
    }

    pub async fn get_documents(&self) -> Result<Vec<Document>, AppError> {
        Ok(self.repository.get_all().await?)
    }
}