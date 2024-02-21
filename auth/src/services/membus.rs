use crate::application::{event::Event, port::EventBus};
use crate::port::EventStream;
use crate::scalar::TransactionId;
use futures::channel::mpsc::{self, UnboundedSender};
use futures::stream::StreamExt;
use std::sync::{Arc, Mutex};

pub struct MemBus {
    event_subscribers: Arc<Mutex<Vec<UnboundedSender<(Vec<Event>, Option<TransactionId>)>>>>,
}

impl Default for MemBus {
    fn default() -> Self {
        MemBus {
            event_subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl EventBus for MemBus {
    async fn publish(&self, events: Vec<Event>, transaction_id: Option<TransactionId>) -> () {
        let subscribers = self.event_subscribers.lock().unwrap();
        for sub in subscribers.iter() {
            // Here, we clone the event for each subscriber. This is a simple approach but might not be the most efficient for complex event types or a large number of subscribers.
            let _ = sub.unbounded_send((events.clone(), transaction_id.clone()));
            // Ignore send errors
        }
    }

    fn subscribe(&self) -> EventStream {
        let (tx, rx) = mpsc::unbounded();
        {
            let mut subscribers = self.event_subscribers.lock().unwrap();
            subscribers.push(tx);
        }
        rx.boxed()
    }
}
