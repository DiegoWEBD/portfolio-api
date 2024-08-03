use std::sync::Arc;
use crate::{application::errors::AppError, domain::project::{project_repository::ProjectRepository, Project}, infrastructure::project::postgres_project_repository::PostgresProjectRepository};


pub struct ProjectServices {
    project_repository: Arc<dyn ProjectRepository + Send + Sync>
}

impl ProjectServices {

    pub fn new(project_repository: PostgresProjectRepository) -> Self {
        ProjectServices { 
            project_repository: Arc::new(project_repository)
        }
    }

    pub async fn get_project(&self, id: i32) -> Result<Project, AppError> {
        
        match self.project_repository.find_by_id(id).await? {
            Some(found_project) => {
                Ok(found_project)     
            }
            None => {
                Err(AppError::NotFoundError(format!("Project with id {} doesn't exist.", id)))
            }
        }
    }

    pub async fn get_projects(&self) -> Result<Vec<Project>, AppError> {
        let projects: Vec<Project> = self.project_repository.get_all().await?;
        Ok(projects)
    }

    pub async fn add_project(&self, name: &String, description: &String) -> Result<Project, AppError> {
        if let Some(_) = self.project_repository.find_by_name(name).await? {
            return Err(AppError::AlreadyExistsError(format!("Project with name '{}' already exists.", name)));
        }

        let new_project: Project = Project::new(None, name.to_string(), description.to_string());
        let created_project = self.project_repository.add(new_project).await?;

        Ok(created_project)
    }
}