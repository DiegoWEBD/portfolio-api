use async_trait::async_trait;

use crate::{domain::project::{project_repository::ProjectRepository, Project}, infrastructure::{database::database_connection::DatabaseConnection, utils::convert_pg_error}};

pub struct PostgresProjectRepository {
    db_connection: DatabaseConnection
}

impl PostgresProjectRepository {
    pub async fn new(db_connection: DatabaseConnection) -> Self {
        PostgresProjectRepository { db_connection }
    }
}

#[async_trait]
impl ProjectRepository for PostgresProjectRepository {
    
    async fn get_all(&self) -> Result<Vec<Project>, std::io::Error>  {
        
        let rows = self.db_connection.client.query("select * from project;", &[])
            .await
            .map_err(convert_pg_error)?;

        let projects: Vec<Project> = rows.into_iter().map(|row| {
            Project::new(row.get("id"), row.get("name"), row.get("description"))
        }).collect();

        Ok(projects)
    }

    async fn add(&self, project: Project) -> Result<Project, std::io::Error> {
        self.db_connection.client.execute("insert into project (name, description) values ({}, {});", &[&project.get_name(), &project.get_description()])
            .await
            .map_err(convert_pg_error)?;

        let rows = self.db_connection.client.query("select * from project where name = '{}';", &[&project.get_name()])
            .await
            .map_err(convert_pg_error)?;

        let projects: Vec<Project> = rows.into_iter().map(|row| {
                Project::new(row.get("id"), row.get("name"), row.get("description"))
            }).collect();
    
        let proj: &Project;

        match projects.get(0) {
            Some(p) => {
                proj = p;
            }
            None => {
                panic!("Error ocurred")
            }
        }

        Ok(proj.clone())
    }
}