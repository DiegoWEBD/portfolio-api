use std::sync::Arc;

use crate::{domain::project::{project_repository::ProjectRepository, Project}, infrastructure::project::postgres_project_repository::PostgresProjectRepository};


pub struct ProjectServices {
    project_repository: Arc<dyn ProjectRepository + Send + Sync>
}

impl ProjectServices {

    pub fn new(project_repository: PostgresProjectRepository) -> Self {
        ProjectServices { 
            project_repository: Arc::new(project_repository)
        }
    }

    pub async fn get_projects(&self) -> Result<Vec<Project>, std::io::Error> {
        let projects = self.project_repository.get_all().await?;
        Ok(projects)
    }

    
}