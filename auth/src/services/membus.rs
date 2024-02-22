use crate::application::{port::SignalBus, signal::Signal};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};

pub struct MemBus {
    senders: Arc<Mutex<Vec<Sender<Signal>>>>,
}

impl Default for MemBus {
    fn default() -> Self {
        MemBus {
            senders: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl SignalBus for MemBus {
    fn publish(&self, signal: Signal) -> () {
        let senders = self.senders.lock().unwrap();
        for sender in senders.iter() {
            sender.send(signal.clone()).unwrap();
        }
    }

    fn subscribe<F>(&self, handler: F)
    where
        F: Fn(Signal) + Send + 'static,
    {
        let (sender, receiver) = channel();
        let mut senders = self.senders.lock().unwrap();
        senders.push(sender);
        drop(senders);

        std::thread::spawn(move || loop {
            let event = receiver.recv().unwrap();
            handler(event);
        });
    }
}
