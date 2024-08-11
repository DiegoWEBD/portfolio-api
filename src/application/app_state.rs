use std::sync::Arc;
use super::{document::document_services::DocumentServices, project::project_services::ProjectServices};

#[derive(Clone)]
pub struct AppState {
    pub project_services: Arc<ProjectServices>,
    pub document_services: DocumentServices
}

impl AppState {
    
    pub fn new(project_services: ProjectServices, document_services: DocumentServices) -> Self {
        AppState {
            project_services: Arc::new(project_services),
            document_services
        }
    }
}
