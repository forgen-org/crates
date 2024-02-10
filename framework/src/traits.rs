use async_trait::async_trait;

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

/// Should be implemented on UseCases
#[async_trait]
pub trait Listen<R>
where
    R: Send + Sync,
{
    async fn listen(runtime: &R);
}

/// Should be implemented on Listeners
// #[async_trait]
// pub trait Listen<R>
// where
//     R: Send + Sync,
// {
//     async fn listen(&self, runtime: &R, ) -> Result<Self::Output, Self::Error>;
// }

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
