use actix_web::{App, HttpServer};
use diesel::prelude::*;

use actixjwt::{api, db};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_jwt=trace, actix_web=debug");
    env_logger::init();
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
            .configure(api::users::config_app)
    })
    .bind(url)?
    .run()
    .await
}
