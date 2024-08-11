use std::sync::Arc;

use crate::{application::errors::AppError, domain::project::{project_repository::ProjectRepository, Project}};


pub struct ProjectServices {
    repository: Arc<dyn ProjectRepository + Send + Sync>
}

impl ProjectServices {

    pub fn new(repository: Arc<dyn ProjectRepository + Send + Sync>) -> Self {
        ProjectServices { repository }
    }

    pub async fn get_project(&self, id: i32) -> Result<Project, AppError> {
        
        match self.repository.find_by_id(id).await? {
            Some(found_project) => {
                Ok(found_project)     
            }
            None => {
                Err(AppError::NotFoundError(format!("Project with id {} doesn't exist.", id)))
            }
        }
    }

    pub async fn get_projects(&self) -> Result<Vec<Project>, AppError> {
        Ok(self.repository.get_all().await?)
    }

    pub async fn add_project(&self, name: &String, description: &String, image_name: &String) -> Result<Project, AppError> {
        if let Some(_) = self.repository.find_by_name(name).await? {
            return Err(AppError::AlreadyExistsError(format!("Project with name '{}' already exists.", name)));
        }

        let new_project: Project = Project::new(None, name.to_string(), description.to_string(), image_name.to_string());
        let created_project = self.repository.add(new_project).await?;

        Ok(created_project)
    }
}