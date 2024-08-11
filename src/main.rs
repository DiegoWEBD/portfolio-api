mod domain;
mod infrastructure;
mod presentation;
mod application;

use std::{io::Result, sync::Arc};
use application::{app_state::AppState, document::document_services::DocumentServices, project::project_services::ProjectServices};
use domain::{document::document_repository::DocumentRepository, project::project_repository::ProjectRepository};
use infrastructure::{database::database_connection::DatabaseConnection, document::postgres_document_repository::PostgresDocumentRepository, project::postgres_project_repository::PostgresProjectRepository};
use presentation::api::Api;


#[actix_web::main]
async fn main() -> Result<()> {
    let db_connection: Arc<DatabaseConnection>;

    match DatabaseConnection::new().await {
        Ok(connection) => {
            db_connection = Arc::new(connection);
        }
        Err(error) => {
            panic!("Couldn't connect to database. Error: {}", error);
        }
    }

    let proj_repository: Arc<dyn ProjectRepository + Send + Sync> = Arc::new(PostgresProjectRepository::new(Arc::clone(&db_connection)));
    let proj_services: ProjectServices = ProjectServices::new(proj_repository);

    let docs_repository: Arc<dyn DocumentRepository + Send + Sync> = Arc::new(PostgresDocumentRepository::new(Arc::clone(&db_connection)));
    let docs_services: DocumentServices = DocumentServices::new(docs_repository);

    let app_data: AppState = AppState::new(proj_services, docs_services);
    let api: Api = Api::new(app_data);
    let port: u16 = 8080;

    api.serve(port).await
}