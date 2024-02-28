use crate::application::{
    command::ProjectUser,
    port::{Signal, SignalPub, UserRepository},
};
use forgen::*;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast::{self, Receiver, Sender};

pub struct Tokio {
    sender: Mutex<Sender<Signal>>,
}

impl Default for Tokio {
    fn default() -> Self {
        let (sender, _) = broadcast::channel(100); // Channel with buffer size 100
        Self {
            sender: Mutex::new(sender),
        }
    }
}

#[async_trait]
impl SignalPub for Tokio {
    async fn publish(&self, signal: Signal) -> () {
        let sender = self.sender.lock().unwrap();
        // Ignore the error if there are no subscribers
        let _ = sender.send(signal);
    }
}

impl Tokio {
    pub fn init<R>(&self, runtime: Arc<R>) -> ()
    where
        R: SignalPub + UserRepository,
        R: Send + Sync + 'static,
    {
        let mut receiver = self.subscribe();
        tokio::spawn(async move {
            while let Ok(signal) = receiver.recv().await {
                if let Signal::EventsEmitted {
                    events,
                    user_id,
                    transaction_id,
                } = signal
                {
                    if let Some(user_id) = user_id {
                        ProjectUser {
                            events,
                            user_id,
                            transaction_id: transaction_id.clone(),
                        }
                        .execute(runtime.as_ref())
                        .await
                        .unwrap();
                    }
                }
            }
        });
    }

    pub fn subscribe(&self) -> Receiver<Signal> {
        self.sender.lock().unwrap().subscribe()
    }
}
