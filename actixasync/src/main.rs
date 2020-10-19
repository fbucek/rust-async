use actix_web::{get, web, App, Error as ActixError, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, Mutex};
use std::*;

#[macro_use]
extern crate log;

use actix_async::*;

#[get("/{id}/{name}/index.html")]
async fn index_id_name(info: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = info.into_inner();
    format!("Hello {}! id:{}\n", name, id)
}

#[get("/api/run")]
async fn api_run(
    sender: web::Data<Arc<Mutex<tokio::sync::mpsc::Sender<Message>>>>,
) -> Result<HttpResponse, ActixError> {
    // trace!("{:?}", sender);
    let sender = sender.lock().unwrap();
    sender.send(Message::RunCheck).await.unwrap_or_else(|err| {
        error!(
            "Not possible to send message -> RunCheck - error: {:?}",
            err
        )
    });
    // if let Err(err) = sender.send(Message::RunCheck).unwrap() {
    //     error!("Not possible to send message -> RunCheck");
    // }
    let text = "Send message using sender to start service runner\n".to_string();
    Ok(HttpResponse::Ok().body(text))
}

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
#[actix_web::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "debug,actix_async=trace");
    env_logger::init();

    // Create sender and receiver to communicate with loop
    let (sender, receiver) = tokio::sync::mpsc::channel(10);
    let sender = Arc::new(Mutex::new(sender)); // <-- Actix loop
    let sender_exit = sender.clone(); //Arc::clone(&sender); // <-- Ctrl+C handler

    let (sender2, receiver2) = tokio::sync::mpsc::channel(10);
    let sender2 = Arc::new(Mutex::new(sender2)); // <-- Actix loop
    let sender_exit2 = Arc::clone(&sender2); // <-- Ctrl+C handler

    // Gracefull shutdown -> SIGTERM received -> send message terminate
    ctrlc::set_handler(move || {
        let sender = sender_exit.lock().expect("not possible to lock");
        for _ in 0..4 {
            info!("sending terminate mesage");
            sender
                .try_send(Message::Terminate)
                .expect("not possible to send terminate message");
        }
        let sender = sender_exit2.lock().expect("not possible to lock");
        for _ in 0..4 {
            info!("sending terminate mesage");
            sender
                .try_send(Message::Terminate)
                .expect("not possible to send terminate message");
        }
    })
    .expect("Error setting Ctrl+C handler");

    let builder = std::thread::Builder::new().name("second thread".into()); // into() -> to_string()
    let handler = builder.spawn(move || {
        let runtime = tokio::runtime::Builder::new_multi_thread() //Runtime::new().unwrap();
            .worker_threads(4)
            .enable_all()
            .build()
            .unwrap();
        trace!("Starging block_on");
        runtime.block_on(async move {
            tokio::spawn(async move {
                let mut service_controller = ServiceController::new(receiver);
                service_controller
                    .run()
                    .await
                    .expect("Not possible to run service controller");
            })
            .await
            .expect("Not possible to finish tokio::spawn gracefully");
        });
        trace!("End block on");
    });

    let control_future = tokio::spawn(async move {
        let mut service_controller = ServiceController::new(receiver2);
        service_controller.run().await
    });

    info!("Starting web server");
    info!("http://127.0.0.1:8080/api/run");
    // async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_future = HttpServer::new(move || {
        App::new()
            .service(index_id_name)
            .service(index)
            .service(api_run)
            .data(Arc::clone(&sender))
    })
    .bind("127.0.0.1:8080")
    .expect("Not possible to bind to address")
    .run();

    let res = futures::join!(server_future, control_future);
    handler
        .expect("Not possible to join 'second thread'")
        .join()
        .unwrap();
    info!("Server finished");
    res.0
}
