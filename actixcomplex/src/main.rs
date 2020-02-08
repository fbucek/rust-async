use actix_web::{App, HttpServer};
use futures::lock::Mutex;
use std::sync::Arc;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use std::*;

#[macro_use]
extern crate log;

use actixcomplex::controller;
use actixcomplex::webserver;

// #[tokio::main]
#[actix_rt::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "debug,actixcomplex=trace");
    env_logger::init();

    // Enabled SSL
    let certificate = "actixcomplex/keys/actixcomplex.crt";
    let private_key = "actixcomplex/keys/actixcomplex.key";
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(&private_key, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(&certificate).unwrap();

    // Create sender and receiver to communicate with loop
    let (sender, receiver) = tokio::sync::mpsc::channel(10);
    let sender = Arc::new(Mutex::new(sender)); // <-- Actix loop
    let sender_exit = sender.clone();

    // Gracefull shutdown -> SIGTERM received -> send message terminate
    ctrlc::set_handler(move || loop {
        if let Some(mut sender) = sender_exit.try_lock() {
            sender
                .try_send(controller::Message::Terminate)
                .expect("not possible to send terminate message");
            break;
        }
    })
    .expect("Error setting Ctrl+C handler");

    let control_future = tokio::spawn(async move {
        let mut service_controller = controller::ServiceController::new(receiver);
        service_controller.run().await
    });

    info!("Starting web server");
    info!("https://localhost:8088/api/run");
    info!("https://localhost:8088/private/test");
    info!("https://localhost:8088/password/41/filip");
    // async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_future = HttpServer::new(move || {
        App::new()
            .configure(webserver::handlers_api::config)
            .configure(webserver::handlers_www::config)
            .data(Arc::clone(&sender))
    })
    .bind_openssl("127.0.0.1:8088", builder)?
    // .bind("127.0.0.1:8080")
    // .expect("Not possible to bind to address")
    .run();

    let res = futures::join!(server_future, control_future);
    info!("Server finished");
    res.0
}
