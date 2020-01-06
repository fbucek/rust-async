use std::sync::{Arc, Mutex};
use std::*;

#[macro_use]
extern crate log;

#[derive(Debug)]
pub enum Message {
    RunCheck,
    Terminate,
}

#[derive(Debug)]
pub struct ServiceController {
    receiver: tokio::sync::mpsc::Receiver<Message>,
    //sender: Arc<Mutex<tokio::sync::mpsc::Sender<Message>>>,
}

impl ServiceController {
    pub fn new(receiver: tokio::sync::mpsc::Receiver<Message>) -> Self {
        // pub fn new(receiver: tokio::sync::mpsc::Receiver<Message>, sender: Arc<Mutex<tokio::sync::mpsc::Sender<Message>>>) -> Self {
        //pub fn new(sender: Arc<Mutex<tokio::sync::mpsc::Sender<Message>>>) -> Self {
        ServiceController { receiver }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // let receiver = Arc::clone(&self.receiver);
        // let receiver = self.
        test().await;
        // tokio::spawn(async move {
            // while let Ok(message) = receiver.lock().unwrap().try_recv() {
            loop {
                let message = self.receiver.recv().await
                    .expect("ServiceController: Not possible to receive message");
                // let message = receiver.lock().unwrap().recv().await.unwrap();
                trace!("ServiceController: message received {:?}", &message);
                match message {
                    Message::RunCheck => {
                        // test().await;
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

async fn test() {
    info!("test function");
}