use std::{borrow::Borrow, sync::Arc};

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
        let projects: Vec<Project> = self.project_repository.get_all().await?;
        Ok(projects)
    }

    pub async fn add_project(&self, name: &String, description: &String) -> Result<Project, std::io::Error> {
        let result: Option<Project> = self.project_repository.find_by_name(name).await?;

        match result {
            Some(_) => {
                panic!("Project with the given name already exists.")
            }
            None => {
                println!("Ok")
            }
        }

        let new_project: Project = Project::new(1, name.to_string(), description.to_string());
        self.project_repository.add(&new_project).await?;

        Ok(new_project)
    }
}