use actix_web::{get, web::Data, HttpResponse, Responder};

use crate::{application::app_state::AppState, presentation::http::http_error::CustomHttpError};

#[get("/projects")]
async fn get_projects(data: Data<AppState>) -> impl Responder {
    
    match data.project_services.get_projects().await {
        Ok(projects) => {
            return HttpResponse::Ok().json(projects);
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(CustomHttpError::new(500, error.to_string()));
        }
    };
}