// Same Idea found.. sad
// so I still think this train is good
pub trait PipeTrait {
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

impl<X> PipeTrait for X {}
