use actix_web::{get, web, App, Error as ActixError, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, Mutex};
use std::*;

#[macro_use]
extern crate log;

#[get("/{id}/{name}/index.html")]
async fn index_id_name(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}\n", info.1, info.0)
}

#[get("/api/stop")]
async fn api_stop(
    sender: web::Data<Arc<Mutex<sync::mpsc::Sender<Message>>>>,
) -> Result<HttpResponse, ActixError> {
    trace!("{:?}", sender);
    let sender = sender.lock().unwrap();
    sender.send(Message::RunCheck).unwrap_or_else(|err| {
        error!(
            "Not possible to send message -> RunCheck - error: {:?}",
            err
        )
    });
    // if let Err(err) = sender.send(Message::RunCheck).unwrap() {
    //     error!("Not possible to send message -> RunCheck");
    // }
    let text = "Shoud stop task\n".to_string();
    Ok(HttpResponse::Ok().body(text))
}

#[get("/index.html")]
async fn index() -> &'static str {
    "hello\n"
}


#[derive(Debug)]
pub enum Message {
    RunCheck,
    Terminate,
}

#[derive(Debug)]
pub struct ServiceController {
    receiver: Arc<Mutex<tokio::sync::mpsc::Receiver<Message>>>,
    //sender: Arc<Mutex<tokio::sync::mpsc::Sender<Message>>>,
}

impl ServiceController {
    pub fn new(receiver: Arc<Mutex<tokio::sync::mpsc::Receiver<Message>>>) -> Self {
        // pub fn new(receiver: tokio::sync::mpsc::Receiver<Message>, sender: Arc<Mutex<tokio::sync::mpsc::Sender<Message>>>) -> Self {
        //pub fn new(sender: Arc<Mutex<tokio::sync::mpsc::Sender<Message>>>) -> Self {
        ServiceController { receiver }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let receiver = Arc::clone(&self.receiver);
        // let receiver = self.
        // tokio::spawn(async move {
            while let Some(message) = receiver.lock().unwrap().recv().await {
                // let message = receiver.lock().unwrap().recv().await.unwrap();
                trace!("ServiceController: message received {:?}", &message);
                match message {
                    Message::RunCheck => {
                        test();
                        info!("ServiceController: now should be able to run task");
                    }
                    Message::Terminate => {
                        info!("ServiceController: now terminating project");
                        break; // loop
                    }
                }
            }
            // trace!("ServiceController: tokio loop finishes");
        // });

        Ok(())
    }
}

impl Drop for ServiceController {
    fn drop(&mut self) {
        trace!("dropping service controller");
        // self.sender.lock().unwrap().send(Message::Terminate).unwrap();
    }
}

fn test() {
    info!("test function");
}

#[derive(Default, Debug)]
struct Check {
    ip: String,
    port: String,
}




// #[tokio::main]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug,actix_async=trace");
    env_logger::init();

    // Create sender and receiver to communicate with loop
    let (sender, receiver) = tokio::sync::mpsc::channel(10);
    let sender = Arc::new(Mutex::new(sender)); // <-- Actix loop
    let sender_exit = Arc::clone(&sender); // <-- Ctrl+C handler
    let receiver = Arc::new(Mutex::new(receiver));
    let receiver_tokio = Arc::clone(&receiver);
    let receiver_tokio2 = Arc::clone(&receiver);
    let receiver_tokio3 = Arc::clone(&receiver);

    // Gracefull shutdown -> SIGTERM received -> send message terminate
    ctrlc::set_handler(move || {
        let mut sender = sender_exit.lock().expect("not possible to lock");
        for _ in 0..4 {
            info!("sending terminate mesage");
            sender.try_send(Message::Terminate).expect("not possible to send terminate message");
        }
    })
    .expect("Error setting Ctrl+C handler");

    let mut service_controller = ServiceController::new(Arc::clone(&receiver));
    service_controller
        .run()
        .await
        .expect("Not possible to run thread loop");

    tokio::spawn(async move {
        let mut service_controller = ServiceController::new(receiver_tokio);
        service_controller
            .run()
            .await
            .expect("Not possible to run thread loop");
    });

    tokio::spawn(async move {
        let mut service_controller = ServiceController::new(receiver_tokio2);
        service_controller
            .run()
            .await
            .expect("Not possible to run thread loop");
    });

 // TODO: try this: https://users.rust-lang.org/t/new-with-async-how-to-structure-application/35233/4
    // tokio::spawn(async move {
    //     loop {
    //         let message = receiver.lock().unwrap().recv().await.unwrap();
    //         trace!("message received {:?}", &message);
    //         match message {
    //             Message::RunCheck => {
    //                 info!("separtate tokio::spawn: now should be able to run task");
    //             }
    //             Message::Terminate => {
    //                 info!("separtate tokio::spawn: now terminating project");
    //                 break; // loop
    //             }
    //         }
    //     }
    //     trace!("tokio loop finishes");
    // });

    info!("Starting web server");
    // async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = HttpServer::new(move || {
        App::new()
            .service(index_id_name)
            .service(index)
            .service(api_stop)
            .data(Arc::clone(&sender))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await;

    info!("Server finished");
    res
}
