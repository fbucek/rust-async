use actix_web::{get, App, HttpServer};
use std::sync::Arc;
use futures::lock::Mutex;

use std::*;

#[macro_use]
extern crate log;

use actixcomplex::controller;
use actixcomplex::webserver;



#[get("/")]
async fn index() -> &'static str {
    "Hello World!"
}

#[derive(Default, Debug)]
struct Check {
    ip: String,
    port: String,
}

// #[tokio::main]
#[actix_rt::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "debug,actix_async=trace");
    env_logger::init();

    // Create sender and receiver to communicate with loop
    let (sender, receiver) = tokio::sync::mpsc::channel(10);
    let sender = Arc::new(Mutex::new(sender)); // <-- Actix loop

    let control_future = tokio::spawn(async move {
        let mut service_controller = controller::ServiceController::new(receiver);
        service_controller
            .run()
            .await
    });

    info!("Starting web server");
    info!("paste into web browser to test: 127.0.0.1:8080/api/run");
    // async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_future = HttpServer::new(move || {
        App::new()
            .service(webserver::handlers::index_id_name)
            .service(index)
            .service(webserver::api::api_run)
            .data(Arc::clone(&sender))
    })
    .bind("127.0.0.1:8080").expect("Not possible to bind to address")
    .run();

    let res = futures::join!(server_future, control_future);
    info!("Server finished");
    res.0
}
