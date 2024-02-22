pub trait State: Default {
    type Message;
    type Event;
    type Error: std::error::Error;

    fn apply(&mut self, event: &Self::Event) -> &mut Self;

    fn send(&self, message: &Self::Message) -> Result<Vec<Self::Event>, Self::Error>;

    fn new(events: &[Self::Event]) -> Self {
        let mut value = Self::default();
        for event in events {
            value.apply(event);
        }
        value
    }
}

pub trait Projection: Default {
    type Event;

    fn apply(&mut self, event: &Self::Event) -> &mut Self;

    fn apply_all(&mut self, events: &[Self::Event]) -> &mut Self {
        for event in events {
            self.apply(event);
        }
        self
    }
}

pub trait Snapshot: Projection {
    type Error: std::error::Error;

    fn rewind(&self) -> Result<Vec<Self::Event>, Self::Error>;
}

pub trait Command<R> {
    type Error: std::error::Error;

    fn execute(&self, runtime: &R) -> Result<(), Self::Error>;
}

pub trait Query<R> {
    type Output;
    type Error: std::error::Error;

    fn fetch(&self, runtime: &R) -> Result<Self::Output, Self::Error>;
}

pub trait Listener<R> {
    type Signal;

    fn listen(&self, runtime: &R, signal: &Self::Signal);
}
