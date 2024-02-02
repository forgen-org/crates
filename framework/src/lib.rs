pub extern crate auto_delegate;
pub extern crate thiserror;

pub use async_trait::async_trait;
pub use auto_delegate::*;
use serde::{Deserialize, Serialize};
pub use thiserror::*;

pub trait Event: Serialize + for<'de> Deserialize<'de> {}

pub trait Message<T, E>
where
    T: Event,
{
    fn send(self, events: &[T]) -> Result<Vec<T>, E>;
}

pub trait Snapshot: Projection {
    type Error;
    fn restore(&self) -> Result<Vec<Self::Event>, Self::Error>;
}

pub trait Projection: Serialize + for<'de> serde::Deserialize<'de> {
    type Event: Event;
    fn apply(&mut self, events: &[Self::Event]);
}

pub trait Runtime: Send + Sync {}

#[async_trait]
pub trait Command<R, E>
where
    R: Runtime,
{
    async fn execute(self, runtime: &R) -> Result<(), E>;
}

#[async_trait]
pub trait Query<R, T, E>
where
    R: Runtime,
{
    async fn execute(self, runtime: &R) -> Result<T, E>;
}
