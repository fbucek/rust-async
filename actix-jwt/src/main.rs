#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer};
use diesel::prelude::*;

// TODO: Move to lib crate
mod api;
mod db;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = diesel::r2d2::ConnectionManager::<SqliteConnection>::new(database_url);
    let pool: db::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let url = "127.0.0.1:8080";

    println!("http://{}", url);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/users", web::get().to(api::users::get_users))
            .route("/users/{id}", web::get().to(api::users::get_user_by_id))
            .route("/users", web::post().to(api::users::add_user))
            .route("/users/{id}", web::delete().to(api::users::delete_user))
    })
    .bind(url)?
    .run()
    .await
}
