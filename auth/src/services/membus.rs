use crate::application::{
    event::Event,
    port::{EventBus, EventStream},
};
use futures::channel::mpsc::{self, UnboundedSender};
use futures::stream::StreamExt;
use std::sync::{Arc, Mutex};

pub struct MemBus {
    subscribers: Arc<Mutex<Vec<UnboundedSender<Vec<Event>>>>>,
}

impl MemBus {
    pub fn new() -> Self {
        MemBus {
            subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl EventBus for MemBus {
    fn publish(&self, events: Vec<Event>) {
        let subscribers = self.subscribers.lock().unwrap();
        for sub in subscribers.iter() {
            // Here, we clone the event for each subscriber. This is a simple approach but might not be the most efficient for complex event types or a large number of subscribers.
            let _ = sub.unbounded_send(events.clone()); // Ignore send errors
        }
    }

    fn subscribe(&self) -> EventStream {
        let (tx, rx) = mpsc::unbounded();
        {
            let mut subscribers = self.subscribers.lock().unwrap();
            subscribers.push(tx);
        }
        rx.boxed()
    }
}
