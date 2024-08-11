use std::sync::Arc;

use async_trait::async_trait;

use crate::{application::errors::AppError, domain::document::{document_repository::DocumentRepository, Document}, infrastructure::{database::database_connection::DatabaseConnection, utils::convert_pg_error}};

pub struct PostgresDocumentRepository {
    db_connection: Arc<DatabaseConnection>
}

impl PostgresDocumentRepository {

    pub fn new(db_connection: Arc<DatabaseConnection>) -> Self {
        PostgresDocumentRepository { db_connection }
    }
}

#[async_trait]
impl DocumentRepository for PostgresDocumentRepository {
    
    async fn get_all(&self) -> Result<Vec<Document>, AppError> {

        let rows = self.db_connection.client.query("select * from document;", &[])
            .await
            .map_err(convert_pg_error)?;

        let documents: Vec<Document> = rows.into_iter()
            .map(|row| {
                Document::new(row.get("id"), row.get("title"), row.get("description"), row.get("download_url"))
            })
            .collect();

        Ok(documents)
    }
}