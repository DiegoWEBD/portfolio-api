use actix_web::web::ServiceConfig;
use super::routes;

pub struct ProjectRouter;

impl ProjectRouter {

    pub fn config(cfg: &mut ServiceConfig) {
        cfg
            .service(routes::get_projects)
            .service(routes::add_project);
    }
}