use std::sync::Arc;
use super::project::project_services::ProjectServices;

#[derive(Clone)]
pub struct AppState {
    pub project_services: Arc<ProjectServices>
}

impl AppState {
    
    pub fn new(project_services: ProjectServices) -> Self {
        AppState {
            project_services: Arc::new(project_services)
        }
    }
}
