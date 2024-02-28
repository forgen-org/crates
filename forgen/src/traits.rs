use async_trait::async_trait;

/// Implement on Events
pub trait Dispatch: Default {
    type Message;
    type Event;
    type Error: std::error::Error;

    fn dispatch(&self, message: &Self::Message) -> Result<Vec<Self::Event>, Self::Error>;
}

/// Implement on Projections
pub trait Project: Default {
    type Event;

    fn push(&mut self, event: &Self::Event) -> &mut Self;

    fn extend(&mut self, events: &[Self::Event]) -> &mut Self {
        for event in events {
            self.push(event);
        }
        self
    }

    fn project(events: &[Self::Event]) -> Self {
        let mut value = Self::default();
        value.extend(events);
        value
    }
}

/// Implement on Snapshots
pub trait Rewind: Project {
    type Error: std::error::Error;

    fn rewind(&self) -> Result<Vec<Self::Event>, Self::Error>;
}

/// Implement on Commands
#[async_trait]
pub trait Execute<R> {
    type Error: std::error::Error;

    async fn execute(&self, runtime: &R) -> Result<(), Self::Error>;
}

/// Implement on Queries
#[async_trait]
pub trait Fetch<R> {
    type Output;
    type Error: std::error::Error;

    async fn fetch(&self, runtime: &R) -> Result<Self::Output, Self::Error>;
}

/// Implement on Presenters
#[async_trait]
pub trait Reduce<R> {
    type Action;

    async fn reduce(&self, runtime: &R, action: Self::Action) -> Self;
}
