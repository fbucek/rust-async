use super::api;
use super::handlers;

pub fn api_config(_cfg: &mut actix_web::web::ServiceConfig) {
    _cfg.service(api::api_run);
}

pub fn www_config(_cfg: &mut actix_web::web::ServiceConfig) {
    _cfg.service(handlers::index_id_name)
        .service(handlers::index)
        .service(handlers::password);
}
