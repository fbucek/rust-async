use crate::db::users::InputUser;
use crate::db::{self, Pool};
use actix_web::{post, web, Error, HttpResponse};

pub fn config_app(cfg: &mut web::ServiceConfig) {
    log::info!("Actix user config");
    cfg.service(web::scope("api/auth").service(login));
}

#[post("/signup")]
pub async fn signup(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let user = InputUser {
        username: "Test".to_string(),
        password: "strong".to_string(),
        email: "test@test.com".to_string(),
    };

    HttpResponse::Ok().json(user).await
    // HttpResponse::InternalServerError().await
}

#[post("/login")]
pub async fn login(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let user = InputUser {
        username: "Test".to_string(),
        password: "strong".to_string(),
        email: "test@test.com".to_string(),
    };

    HttpResponse::Ok().json(user).await
    // HttpResponse::InternalServerError().await
}
