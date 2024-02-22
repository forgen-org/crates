use super::traits::*;

pub struct Framework<R>(R);

impl<R> Framework<R> {
    pub fn execute<C>(&self, command: &C) -> Result<(), C::Error>
    where
        C: Command<R>,
    {
        command.execute(&self.0)
    }

    pub fn fetch<Q>(&self, query: &Q) -> Result<Q::Output, Q::Error>
    where
        Q: Query<R>,
    {
        query.fetch(&self.0)
    }
}
