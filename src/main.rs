mod domain;
mod infrastructure;
mod presentation;
mod application;

use std::io::Result;
use application::{app_state::AppState, project::project_services::ProjectServices};
use infrastructure::{database::database_connection::DatabaseConnection, project::postgres_project_repository::PostgresProjectRepository};
use presentation::api::Api;


#[actix_web::main]
async fn main() -> Result<()> {
    let db_connection: DatabaseConnection;

    match DatabaseConnection::new().await {
        Ok(connection) => {
            db_connection = connection;
        }
        Err(error) => {
            panic!("Couldn't connect to database. Error: {}", error);
        }
    }

    let proj_repository: PostgresProjectRepository = PostgresProjectRepository::new(db_connection).await;
    let project_services: ProjectServices = ProjectServices::new(proj_repository);
    let app_data: AppState = AppState::new(project_services);
    let api: Api = Api::new(app_data);
    let port: u16 = 8080;

    api.serve(port).await
}