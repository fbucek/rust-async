use crate::db::users::InputUser;
use crate::db::{self, Pool};
use actix_web::{web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

pub async fn get_users(dbconn: web::Data<Pool>) -> Result<HttpResponse, Error> {
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
    Ok(
        web::block(move || db::users::db_get_user_by_id(db.into_inner(), user_id.into_inner()))
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
    Ok(
        web::block(move || db::users::add_single_user(db.into_inner(), item.into_inner()))
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
    Ok(
        web::block(move || db::users::delete_single_user(db.into_inner(), user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}
