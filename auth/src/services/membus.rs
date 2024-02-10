use crate::application::{
    event::Event,
    port::{EventBus, TransactionBus},
    transaction::Transaction,
};
use framework::*;
use futures::channel::mpsc::{self, UnboundedSender};
use futures::stream::StreamExt;
use std::sync::{Arc, Mutex};

pub struct MemBus {
    event_subscribers: Arc<Mutex<Vec<UnboundedSender<(TransactionId, Vec<Event>)>>>>,
    transaction_subscribers: Arc<Mutex<Vec<UnboundedSender<Transaction>>>>,
}

impl Default for MemBus {
    fn default() -> Self {
        MemBus {
            event_subscribers: Arc::new(Mutex::new(Vec::new())),
            transaction_subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl EventBus for MemBus {
    fn publish(&self, events: Vec<Event>) -> TransactionId {
        let transaction_id = TransactionId::default();
        let subscribers = self.event_subscribers.lock().unwrap();
        for sub in subscribers.iter() {
            // Here, we clone the event for each subscriber. This is a simple approach but might not be the most efficient for complex event types or a large number of subscribers.
            let _ = sub.unbounded_send((transaction_id.clone(), events.clone()));
            // Ignore send errors
        }
        transaction_id
    }

    fn subscribe(&self) -> EventStream<Event> {
        let (tx, rx) = mpsc::unbounded();
        {
            let mut subscribers = self.event_subscribers.lock().unwrap();
            subscribers.push(tx);
        }
        rx.boxed()
    }
}

impl TransactionBus for MemBus {
    fn publish(&self, transaction: Transaction) {
        let subscribers = self.transaction_subscribers.lock().unwrap();
        for sub in subscribers.iter() {
            // Here, we clone the event for each subscriber. This is a simple approach but might not be the most efficient for complex event types or a large number of subscribers.
            let _ = sub.unbounded_send(transaction.clone()); // Ignore send errors
        }
    }

    fn subscribe(&self) -> TransactionStream<Transaction> {
        let (tx, rx) = mpsc::unbounded();
        {
            let mut subscribers = self.transaction_subscribers.lock().unwrap();
            subscribers.push(tx);
        }
        rx.boxed()
    }
}
