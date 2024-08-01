use super::Project;
use async_trait::async_trait;

#[async_trait]
pub trait ProjectRepository {
    async fn get_all(&self) -> Result<Vec<Project>, std::io::Error>;
    async fn add(&self, project: Project) -> Result<Project, std::io::Error>;
}