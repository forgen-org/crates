#[allow(dead_code)]
pub struct Function<R, T, E> {
    f: Box<dyn Fn(R) -> Result<T, E>>,
}

impl<R, T, E> Function<R, T, E> {
    #[allow(dead_code)]
    pub fn new<F>(f: F) -> Function<R, T, E>
    where
        F: Fn(R) -> Result<T, E> + 'static,
    {
        Function {
            f: Box::new(move |runtime: R| f(runtime)),
        }
    }

    #[allow(dead_code)]
    pub fn run(&self, runtime: R) -> Result<T, E> {
        (self.f)(runtime)
    }
}
