use actix_web::{web, App, HttpServer};

mod handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    println!("http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .route("/users", web::get().to(handlers::get_users))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

