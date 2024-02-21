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

/// Should be implemented on States and Projections
pub trait Project<E>: Default {
    fn apply(&mut self, event: &E) -> &mut Self;

    /// Default methods
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
pub trait Execute<R>
where
    R: Send + Sync,
{
    type Error: std::error::Error;
    async fn execute(&self, runtime: &R) -> Result<(), Self::Error>;
}

/// Should be implemented on Queries
pub trait Fetch<R>
where
    R: Send + Sync,
{
    type Output;
    type Error: std::error::Error;
    async fn fetch(&self, runtime: &R) -> Result<Self::Output, Self::Error>;
}

/// Should be implemented on UseCases
pub trait Listen<R>
where
    R: Send + Sync,
{
    async fn listen(runtime: &R);
}
