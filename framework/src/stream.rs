use crate::TransactionId;
use futures::Stream;
use std::pin::Pin;

pub type EventStream<E> = Pin<Box<dyn Stream<Item = (TransactionId, Vec<E>)> + Send>>;
pub type TransactionStream<T> = Pin<Box<dyn Stream<Item = T> + Send>>;
