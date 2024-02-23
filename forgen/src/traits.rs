use async_trait::async_trait;

pub trait Messenger: Default {
    type Message;
    type Event;
    type Error: std::error::Error;

    fn send(&self, message: &Self::Message) -> Result<Vec<Self::Event>, Self::Error>;
}

pub trait Projector: Default {
    type Event;

    fn push(&mut self, event: &Self::Event) -> &mut Self;

    fn extend(&mut self, events: &[Self::Event]) -> &mut Self {
        for event in events {
            self.push(event);
        }
        self
    }

    fn new(events: &[Self::Event]) -> Self {
        let mut value = Self::default();
        value.extend(events);
        value
    }
}

pub trait Snapshoter: Projector {
    type Error: std::error::Error;

    fn rewind(&self) -> Result<Vec<Self::Event>, Self::Error>;
}

#[async_trait]
pub trait Commander<R> {
    type Error: std::error::Error;

    async fn execute(&self, runtime: &R) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait Querier<R> {
    type Output;
    type Error: std::error::Error;

    async fn fetch(&self, runtime: &R) -> Result<Self::Output, Self::Error>;
}
