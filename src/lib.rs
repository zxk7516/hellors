use futures::Async;
use futures::Future;

pub struct Doubler<T> {
    inner: T,
}

pub fn double<T>(inner: T) -> Doubler<T> {
    Doubler { inner }
}
impl<T> Future for Doubler<T>
where
    T: Future<Item = usize>,
{
    type Item = usize;
    type Error = T::Error;

    fn poll(&mut self) -> Result<Async<usize>, T::Error> {
        match self.inner.poll()? {
            Async::Ready(v) => Ok(Async::Ready(v * 2)),
            Async::NotReady => Ok(Async::NotReady),
        }
    }
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
