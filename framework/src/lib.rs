mod short_backtrace;

pub extern crate auto_delegate;
pub extern crate thiserror;

pub use async_trait::async_trait;
pub use auto_delegate::*;
pub use thiserror::*;
pub use tracing::{debug, error, info, trace, warn};

use short_backtrace::ShortBacktrace;

/// Should be implemented on Vec<Event>
pub trait Dispatch {
    type Error: std::error::Error;
    type Event;
    type Message;
    fn dispatch(&self, message: &Self::Message) -> Result<Vec<Self::Event>, Self::Error>;
}

/// Should be implemented on Projections
pub trait Project: Default {
    type Event;
    fn apply(&mut self, event: &Self::Event) -> &mut Self;
    fn apply_all(&mut self, events: &[Self::Event]) -> &mut Self {
        for event in events {
            self.apply(event);
        }
        self
    }
    fn project(events: &[Self::Event]) -> Self {
        let mut value = Self::default();
        value.apply_all(events);
        value
    }
}

/// Should be implemented on Snapshots
pub trait Rewind: Project {
    type Error: std::error::Error;
    fn rewind(&self) -> Result<Vec<Self::Event>, Self::Error>;
}

/// Should be implemented on Commands
#[async_trait]
pub trait Execute<R>
where
    R: Send + Sync,
{
    type Error: std::error::Error;
    async fn execute(self, runtime: &R) -> Result<(), Self::Error>;
}

/// Should be implemented on Queries
#[async_trait]
pub trait Fetch<R>
where
    R: Send + Sync,
{
    type Output;
    type Error: std::error::Error;
    async fn fetch(self, runtime: &R) -> Result<Self::Output, Self::Error>;
}

#[async_trait]
pub trait Framework: Send + Sync + Sized {
    async fn execute<T>(&self, command: T) -> Result<(), T::Error>
    where
        T: Execute<Self> + Send + Sync,
    {
        command.execute(self).await
    }

    async fn fetch<T>(&self, query: T) -> Result<T::Output, T::Error>
    where
        T: Fetch<Self> + Send + Sync,
    {
        query.fetch(self).await
    }
}

/// Generic Error
#[derive(Error, Debug)]
#[error("An unexpected error occurred: {message}")]
pub struct UnexpectedError {
    message: String,
}

impl UnexpectedError {
    pub fn from<T: std::error::Error>(err: T) -> Self {
        error!(
            error = %err,
            backtrace = ?ShortBacktrace::new(),
            "Unexpected Error"
        );
        UnexpectedError {
            message: err.to_string(),
        }
    }
}
