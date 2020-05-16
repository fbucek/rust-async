use actix_web::{App, HttpServer};
use diesel::prelude::*;

use actixjwt::{api, db};
use actixjwt::api::validator::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Not possible to load .env file");
    std::env::set_var("RUST_LOG", "actixjwt=trace, actix_web=debug");
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
        // let auth = HttpAuthentication::bearer(auth_validator);
        App::new()
            .data(pool.clone())
            // .wrap(auth)
            .configure(api::auth::config_app)
            .configure(api::users::config_app)
    })
    .bind(url)?
    .run()
    .await
}
