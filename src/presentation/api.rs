use std::io::Result;
use actix_cors::Cors;
use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use crate::{application::app_state::AppState, presentation::http::default_response::DefaultResponse};
use super::{document::document_router::DocumentRouter, project::project_router::ProjectRouter};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(DefaultResponse::new())
}

pub struct Api {
    app_state: Data<AppState>
}

impl Api {

    pub fn new(app_state: AppState) -> Self {
        Api { app_state: Data::new(app_state) }
    }

    pub async fn serve(&self, port: u16) -> Result<()> {
        let app_state = self.app_state.clone();

        HttpServer::new(move || {
            App::new()
                .app_data(app_state.clone())
                .service(index)
                .configure(ProjectRouter::config)
                .configure(DocumentRouter::config)
                .wrap(
                    Cors::default()
                            .allowed_origin("http://localhost:5173")   
                )
        })
        .bind(("127.0.0.1", port))?
        .run()
        .await
    }
}