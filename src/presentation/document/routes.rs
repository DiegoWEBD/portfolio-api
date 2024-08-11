use actix_web::{get, web::Data, HttpResponse, Responder};

use crate::{application::app_state::AppState, presentation::http::http_error::CustomHttpError};

#[get("/documents")]
async fn get_documents(data: Data<AppState>) -> impl Responder {
    
    match data.document_services.get_documents().await {
        Ok(documents) => {
            return HttpResponse::Ok().json(documents);
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(CustomHttpError::new(500, error.to_string()));
        }
    };
}