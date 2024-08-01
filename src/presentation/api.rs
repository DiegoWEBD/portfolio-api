use std::io::Result;
use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use crate::{application::app_state::AppState, presentation::http::default_response::DefaultResponse};
use super::project::project_router::ProjectRouter;

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
        })
        .bind(("127.0.0.1", port))?
        .run()
        .await
    }
}