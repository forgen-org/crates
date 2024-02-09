mod short_backtrace;

pub extern crate auto_delegate;
pub extern crate thiserror;

pub use async_trait::async_trait;
pub use auto_delegate::*;
pub use thiserror::*;
pub use tracing::{debug, error, info, trace, warn};

use short_backtrace::ShortBacktrace;

/// Should be implemented on Messages
pub trait Dispatch<E> {
    type Error: std::error::Error;
    fn dispatch(&self, events: &[E]) -> Result<Vec<E>, Self::Error>;
}

/// Automatically implemented on Vec<Event>
pub trait Story<E> {
    fn dispatch<T>(&self, message: &T) -> Result<Vec<E>, T::Error>
    where
        T: Dispatch<E>;
}

impl<E> Story<E> for Vec<E> {
    fn dispatch<T>(&self, message: &T) -> Result<Vec<E>, T::Error>
    where
        T: Dispatch<E>,
    {
        message.dispatch(self)
    }
}

/// Should be implemented on Projections
pub trait Project<E>: Default {
    fn apply(&mut self, event: &E) -> &mut Self;
    fn apply_all(&mut self, events: &[E]) -> &mut Self {
        for event in events {
            self.apply(event);
        }
        self
    }
    fn project(events: &[E]) -> Self {
        let mut value = Self::default();
        value.apply_all(events);
        value
    }
}

/// Should be implemented on Snapshots
pub trait Rewind<E>: Project<E> {
    type Error: std::error::Error;
    fn rewind(&self) -> Result<Vec<E>, Self::Error>;
}

/// Should be implemented on Commands
#[async_trait]
pub trait Execute<R>
where
    R: Send + Sync,
{
    type Error: std::error::Error;
    async fn execute(&self, runtime: &R) -> Result<(), Self::Error>;
}

/// Should be implemented on Queries
#[async_trait]
pub trait Fetch<R>
where
    R: Send + Sync,
{
    type Output;
    type Error: std::error::Error;
    async fn fetch(&self, runtime: &R) -> Result<Self::Output, Self::Error>;
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
