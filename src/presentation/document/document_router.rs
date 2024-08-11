use actix_web::web::ServiceConfig;

use super::routes;

pub struct DocumentRouter;

impl DocumentRouter {

    pub fn config(cfg: &mut ServiceConfig) {
        cfg
            .service(routes::get_documents);
    }
}