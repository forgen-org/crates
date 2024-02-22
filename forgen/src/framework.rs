// use super::traits::*;

// // pub struct Framework<R>(R);

// pub trait Framework
// where
//     Self: Sized,
// {
//     fn execute<C>(&self, command: &C) -> Result<(), C::Error>
//     where
//         C: Command<Self>,
//     {
//         command.execute(self)
//     }

//     fn fetch<Q>(&self, query: &Q) -> Result<Q::Output, Q::Error>
//     where
//         Q: Query<Self>,
//     {
//         query.fetch(self)
//     }
// }

// impl<R> Framework for R {}

// // impl<R> Framework<R> {
// //     pub fn execute<C>(&self, command: &C) -> Result<(), C::Error>
// //     where
// //         C: Command<R>,
// //     {
// //         command.execute(&self.0)
// //     }

// //     pub fn fetch<Q>(&self, query: &Q) -> Result<Q::Output, Q::Error>
// //     where
// //         Q: Query<R>,
// //     {
// //         query.fetch(&self.0)
// //     }
// // }
