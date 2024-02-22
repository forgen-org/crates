use std::sync::Mutex;

use crate::application::{port::SignalPub, signal::Signal};
use tokio::sync::broadcast::{self, Receiver, Sender};

pub struct TokioBus {
    sender: Mutex<Sender<Signal>>,
}

impl Default for TokioBus {
    fn default() -> Self {
        let (sender, _) = broadcast::channel(100); // Channel with buffer size 100
        Self {
            sender: Mutex::new(sender),
        }
    }
}

impl SignalPub for TokioBus {
    fn publish(&self, signal: Signal) -> () {
        let sender = self.sender.lock().unwrap();
        // Ignore the error if there are no subscribers
        let _ = sender.send(signal);
    }
}

impl TokioBus {
    pub fn subscribe(&self) -> Receiver<Signal> {
        self.sender.lock().unwrap().subscribe()
    }
}
