use crate::db::models::{InputUser};

use crate::db::{self, Pool};
use actix_web::{web, Error, HttpResponse};

pub fn config_app(cfg: &mut web::ServiceConfig) {
    log::info!("Actix user config");
    cfg
        .route("/users", web::get().to(get_users))
        .route("/users/{id}", web::get().to(get_user_by_id))
        .route("/users", web::post().to(add_user))
        .route("/users/{id}", web::delete().to(delete_user));
}


pub async fn get_users(dbconn: web::Data<Pool>) -> Result<HttpResponse, Error> {
    log::trace!("Getting users");
    let conn = dbconn.into_inner();
    // .expect("Not possible to unwrap arc");
    Ok(web::block(move || db::users::get_all_users(conn))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn get_user_by_id(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let user_id = user_id.into_inner();
    log::trace!("Getting user by id: {}", &user_id);
    Ok(
        web::block(move || db::users::db_get_user_by_id(db.into_inner(), user_id))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

// Handler for POST /users
pub async fn add_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    log::trace!("Adding user {:?}", &item);
    Ok(
        web::block(move || db::users::add_single_user(db.into_inner(), &item.into_inner()))
            .await
            .map(|user| HttpResponse::Created().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

// Handler for DELETE /users/{id}
pub async fn delete_user(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    log::trace!("Deletings user {:?}", &user_id);
    Ok(
        web::block(move || db::users::delete_single_user(db.into_inner(), user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}
