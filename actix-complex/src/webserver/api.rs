use actix_web::{get, web, HttpResponse, Error as ActixError};
use actix_http;

// Synchronization
use std::sync::Arc;
use futures::lock::Mutex;

use crate::controller;

#[get("/api/run")]
async fn api_run(
    sender: web::Data<Arc<Mutex<tokio::sync::mpsc::Sender<controller::Message>>>>,
) -> Result<HttpResponse, ActixError> {
    // trace!("{:?}", sender);
    let mut sender = sender.lock().await;
    sender.send(controller::Message::RunCheck).await.unwrap_or_else(|err| {
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
pub async fn increment(
    counter: web::Data<Arc<Mutex<i32>>>,
) -> Result<actix_http::Response, actix_web::Error> {
    let mut counter = counter.lock().await;
    *counter += 1;
    Ok(HttpResponse::Ok().body(format!("counter: {}", *counter)))
}

#[get("/api/decrement")]
pub async fn decrement(
    counter: web::Data<Arc<Mutex<i32>>>,
) -> Result<actix_http::Response, actix_web::Error> {
    let mut counter = counter.lock().await;
    *counter -= 1;
    Ok(HttpResponse::Ok().body(format!("counter: {}", *counter)))
}
