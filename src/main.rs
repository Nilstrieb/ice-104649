use std::marker::PhantomData;

use futures::{stream, StreamExt, Future};

pub struct JoinHandle<T> {
    raw: Option<()>,
    id: u64,
    _p: PhantomData<T>,
}

impl<T> Future for JoinHandle<T> {
    type Output = Result<T, ()>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        loop {}
    }
}

pub fn spawn<T>(future: T) -> JoinHandle<T::Output>
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
   loop {}
}

fn main() {
    let bodies = stream::iter([])
        .map(|url: String| spawn(async move { Result::Ok((url, "A".to_owned())) }))
        .buffer_unordered(0);

    bodies.for_each(|b| async {
        match b {
            Ok(Ok((url, b))) => {}
            Err(e) => {}
            Ok(Err(e)) => {}
        }
    });
}
