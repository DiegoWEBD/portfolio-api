use std::sync::Arc;

use async_trait::async_trait;
use crate::{application::errors::AppError, domain::project::{project_repository::ProjectRepository, Project}, infrastructure::{database::database_connection::DatabaseConnection, utils::convert_pg_error}};

pub struct PostgresProjectRepository {
    db_connection: Arc<DatabaseConnection>
}

impl PostgresProjectRepository {
    pub async fn new(db_connection: Arc<DatabaseConnection> ) -> Self {
        PostgresProjectRepository { db_connection }
    }
}

#[async_trait]
impl ProjectRepository for PostgresProjectRepository {
    
    async fn get_all(&self) -> Result<Vec<Project>, AppError>  {
        
        let rows = self.db_connection.client.query("select * from project;", &[])
            .await
            .map_err(convert_pg_error)?;

        let projects: Vec<Project> = rows.into_iter().map(|row| {
            Project::new(row.get("id"), row.get("name"), row.get("description"), row.get("image_name"))
        }).collect();

        Ok(projects)
    }

    async fn add(&self, new_project: Project) -> Result<Project, AppError> {

        let row = self.db_connection.client.query_one("insert into project (name, description, image_name) values ($1, $2, $3) returning id;", &[&new_project.get_name(), &new_project.get_description(), &new_project.get_image_name()])
            .await
            .map_err(convert_pg_error)?;

        let id: i32 = row.get("id");
        let mut created_project = new_project;
        created_project.set_id(id);

        Ok(created_project)
    }

    async fn find_by_name(&self, name: &String) -> Result<Option<Project>, AppError> {
        
        match self.db_connection.client.query_opt("select * from project p where p.name = $1;", &[&name])
            .await
            .map_err(convert_pg_error)? {
                Some(row) => {
                    Ok(Some(Project::new(row.get("id"), row.get("name"), row.get("description"), row.get("image_name"))))
                }
                None => {
                    Ok(None)
                }
            }
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Project>, AppError> {

        match self.db_connection.client.query_opt("select * from project p where p.id = $1", &[&id])
            .await
            .map_err(convert_pg_error)? {
                Some(row) => {
                    Ok(Some(Project::new(row.get("id"), row.get("name"), row.get("description"), row.get("image_name"))))
                }
                None => {
                    Ok(None)
                }
            }
    }
}