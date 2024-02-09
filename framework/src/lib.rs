pub extern crate auto_delegate;
pub extern crate thiserror;

pub use async_trait::async_trait;
pub use auto_delegate::*;
pub use thiserror::*;

/// Should be implemented on Vec<Event>
pub trait Dispatch {
    type Error;
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
    type Error;
    fn rewind(&self) -> Result<Vec<Self::Event>, Self::Error>;
}

/// Should be implemented on Commands
#[async_trait]
pub trait Execute<R>
where
    R: Send + Sync,
{
    type Error;
    async fn execute(self, runtime: &R) -> Result<(), Self::Error>;
}

/// Should be implemented on Queries
#[async_trait]
pub trait Fetch<R>
where
    R: Send + Sync,
{
    type Output;
    type Error;
    async fn fetch(self, runtime: &R) -> Result<Self::Output, Self::Error>;
}

// #[derive(Clone)]
// pub struct Framework<R> {
//     runtime: R,
// }

// impl<R> Framework<R>
// where
//     R: Send + Sync,
// {
//     pub fn new(runtime: R) -> Self {
//         Framework { runtime }
//     }

//     pub async fn execute<T>(&self, command: T) -> Result<(), T::Error>
//     where
//         T: Execute<R>,
//     {
//         command.execute(&self.runtime).await
//     }

//     pub async fn fetch<T>(&self, query: T) -> Result<T::Output, T::Error>
//     where
//         T: Fetch<R>,
//     {
//         query.fetch(&self.runtime).await
//     }
// }

#[async_trait]
pub trait Framework: Send + Sync + Sized {
    async fn execute<T>(&self, command: T) -> Result<(), T::Error>
    where
        T: Execute<Self> + Send + Sync,
    {
        command.execute(&self).await
    }

    async fn fetch<T>(&self, query: T) -> Result<T::Output, T::Error>
    where
        T: Fetch<Self> + Send + Sync,
    {
        query.fetch(&self).await
    }
}

/// Generic Error
#[derive(Error, Debug)]
#[error("An unexpected error occurred: {0}")]
pub struct UnexpectedError(String);

impl UnexpectedError {
    pub fn from<T: std::fmt::Display>(err: T) -> Self {
        UnexpectedError(format!("{}", err))
    }
}
