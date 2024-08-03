use actix_web::{get, post, web::{Data, Json, Path}, HttpResponse, Responder};
use crate::{application::app_state::AppState, domain::project::Project, presentation::http::http_error::CustomHttpError};

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

#[get("/projects/{id}")]
async fn get_project(data: Data<AppState>, id: Path<i32>) -> impl Responder {

    match data.project_services.get_project(*id).await {
        Ok(found_project) => {
            return HttpResponse::Ok().json(found_project);
        }
        Err(error) => {
            return HttpResponse::NotFound().json(CustomHttpError::new(404, error.to_string()));
        }
    }
}

#[post("/projects")]
async fn add_project(data: Data<AppState>, new_project_json: Json<Project>) -> impl Responder {
    let new_project = new_project_json.into_inner();
    
    match data.project_services.add_project(&new_project.get_name(), &new_project.get_description()).await {
        Ok(created_project) => {
            return HttpResponse::Created().json(created_project);
        } 
        Err(error) => {
            return HttpResponse::InternalServerError().json(CustomHttpError::new(500, error.to_string()));
        }
    }
}