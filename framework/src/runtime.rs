// use async_trait::async_trait;

// struct Runtime<R> {
//     runtime: R,
//     listeners: Vec<Box<dyn Listener<R>>>,
// }

// impl<R> Runtime<R>
// where
//     R: Send + Sync,
// {
//     pub fn new(runtime: R) -> Self {
//         Self {
//             runtime,
//             listeners: vec![],
//         }
//     }

//     pub fn add_listener<T>(&mut self, listener: T)
//     where
//         T: Listener<R> + 'static,
//     {
//         self.listeners.push(Box::new(listener))
//     }
// }

// /// Should be implemented on Listeners
// #[async_trait]
// pub trait Listener<R>
// where
//     R: Send + Sync,
// {
//     async fn listen(&self, runtime: &R) -> ();
// }
