use crate::application::errors::AppError;

use super::Project;
use async_trait::async_trait;

#[async_trait]
pub trait ProjectRepository {
    async fn get_all(&self) -> Result<Vec<Project>, AppError>;
    async fn add(&self, new_project: Project) -> Result<Project, AppError>;
    async fn find_by_name(&self , name: &String) -> Result<Option<Project>, AppError>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Project>, AppError>;
}