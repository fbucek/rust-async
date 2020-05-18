use crate::db::users::InputUser;
use crate::db::{self, Pool};
use crate::utils::errors::ServiceError;
use crate::utils::response::ResponseBody;
use actix_web::{delete, get, post, web, Error, HttpResponse};

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg
        .service(add_user)
        .service(get_user_by_id)
        .service(delete_user)
        .service(get_users); // ("/users", web::post().to(add_user))
}

#[get("/users")]
pub async fn get_users(dbconn: web::Data<Pool>) -> Result<HttpResponse, Error> {
    log::trace!("Getting users");
    let conn = dbconn.into_inner();
    // .expect("Not possible to unwrap arc");
    Ok(web::block(move || db::users::get_all_users(conn))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| ServiceError::DbError("Not possible to get users from database".to_string()))?
    )
}

#[get("/users/{id}")]
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
            .map_err(|_| ServiceError::DbError(format!("Not possible get user by id: {}", user_id)))?
    )
}

/// Handler for POST /users
///
/// returns `201` status
#[post("/users")]
pub async fn add_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    log::trace!("Adding user {:?}", &item);
    Ok(
        web::block(move || db::users::signup_user(db.into_inner(), &item.into_inner()))
            .await
            .map(|user| HttpResponse::Created().json(user)) // status 201
            .map_err(|err| ServiceError::DbError(format!("Not possible to add user to database: {:?}", err)))?
    )
}

// Handler for DELETE /users/{id}
#[delete("/users/{id}")]
pub async fn delete_user(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    log::trace!("Deletings user {:?}", &user_id);
    Ok(
        web::block(move || db::users::delete_single_user(db.into_inner(), user_id.into_inner()))
            .await
            .map(|count| {
                let text = format!("Users deleted: {}", count);
                HttpResponse::Ok().json(ResponseBody::new(&text, ""))
            })
            .map_err(|err| ServiceError::DbError(format!("Not possible to delete user: {:?}", err)))?
    )
}
