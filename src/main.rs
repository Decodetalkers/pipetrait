use std::sync::{Arc, Mutex};

trait PipeTrait {
    fn pipe<T>(self, f: fn(Self) -> T) -> T
    where
        Self: Sized,
        T: PipeTrait,
    {
        f(self)
    }

    fn pipe_ref<T>(&self, f: fn(&Self) -> T) -> T
    where
        T: PipeTrait,
    {
        f(self)
    }

    fn pipe_refmut<T>(&mut self, f: fn(&mut Self) -> T) -> T
    where
        T: PipeTrait,
    {
        f(self)
    }
}

impl<T> PipeTrait for Mutex<T> {}

impl<T> PipeTrait for Arc<T> {}

impl<T> PipeTrait for Vec<T> {}

impl PipeTrait for i32 {}

impl PipeTrait for usize {}

fn main() {
    let _ = 1.pipe(Mutex::new).pipe(Arc::new);
    println!("Hello, world!");
}
