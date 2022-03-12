use actix_web::{get, web, Error as ActixError, HttpResponse};

// Synchronization
use futures::lock::Mutex;
use std::sync::Arc;

use crate::controller;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(api_get_json)
        .service(api_run)
        .service(increment)
        .service(decrement);
}

#[get("/api/get_json")]
async fn api_get_json() -> Result<HttpResponse, ActixError> {
    let data = r#"
        {
            "first_name": "John",
            "last_name": "Doe",
            "description": "Normal user"
        }"#;
    Ok(HttpResponse::Ok().body(data))
}

#[get("/api/run")]
async fn api_run(
    sender: web::Data<Arc<Mutex<tokio::sync::mpsc::Sender<controller::Message>>>>,
) -> Result<HttpResponse, ActixError> {
    // trace!("{:?}", sender);
    let mut sender = sender.lock().await;
    sender
        .send(controller::Message::RunCheck)
        .await
        .unwrap_or_else(|err| {
            error!(
                "Not possible to send message -> RunCheck - error: {:?}",
                err
            )
        });
    // if let Err(err) = sender.send(controller::Message::RunCheck).unwrap() {
    //     error!("Not possible to send message -> RunCheck");
    // }
    let text = "Send message using sender to start service runner\n".to_string();
    Ok(HttpResponse::Ok().body(text))
}

#[get("/api/increment")]
async fn increment(
    counter: web::Data<Arc<Mutex<i32>>>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut counter = counter.lock().await;
    *counter += 1;
    Ok(HttpResponse::Ok().body(format!("counter: {}", *counter)))
}

#[get("/api/decrement")]
async fn decrement(
    counter: web::Data<Arc<Mutex<i32>>>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut counter = counter.lock().await;
    *counter -= 1;
    Ok(HttpResponse::Ok().body(format!("counter: {}", *counter)))
}
