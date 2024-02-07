use std::{future::Future, pin::Pin};

pub struct Effect<R, T, E> {
    f: Box<dyn Fn(R) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send + Sync>>>,
}

impl<R, T, E> Effect<R, T, E> {
    pub fn new<F, Fut>(f: F) -> Effect<R, T, E>
    where
        F: Fn(R) -> Fut + 'static,
        Fut: Future<Output = Result<T, E>> + Send + Sync + 'static,
    {
        Effect {
            f: Box::new(move |runtime: R| Box::pin(f(runtime))),
        }
    }

    pub async fn run(&self, runtime: R) -> Result<T, E> {
        (self.f)(runtime).await
    }
}

pub trait ToEffect<R, T, E> {
    fn to_effect(self) -> Effect<R, T, E>;
}
