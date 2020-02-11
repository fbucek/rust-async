type ResultSend<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[derive(Debug)]
pub enum Message {
    RunCheck,
    Terminate,
}

#[derive(Debug)]
pub struct ServiceController {
    receiver: tokio::sync::mpsc::Receiver<Message>,
}

impl ServiceController {
    pub fn new(receiver: tokio::sync::mpsc::Receiver<Message>) -> Self {
        ServiceController { receiver }
    }

    pub async fn run(&mut self) -> ResultSend<()> {
        loop {
            let message = self
                .receiver
                .recv()
                .await
                .expect("ServiceController: Not possible to receive message");
            // let message = receiver.lock().unwrap().recv().await.unwrap();
            trace!("ServiceController: message received {:?}", &message);
            match message {
                Message::RunCheck => {
                    test().await;
                    info!("ServiceController: now should be able to run task");
                }
                Message::Terminate => {
                    info!("ServiceController: now terminating project");
                    break; // loop
                }
            }
        }
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
