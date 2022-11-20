use std::marker::PhantomData;

use futures::{stream, Future, StreamExt};

pub struct JoinHandle<T> {
    _p: PhantomData<T>,
}

impl<T> Future for JoinHandle<T> {
    type Output = Result<T, ()>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        loop {}
    }
}

pub fn spawn<T>(future: T) -> JoinHandle<T::Output>
where
    T: Future,
{
    loop {}
}

fn main() {
    let bodies = stream::iter([])
        .map(|url: String| spawn(async { Result::Ok(url) }))
        .buffer_unordered(0);

    bodies.for_each(|b| async {
        match b {
            Ok(Ok(url)) => {}
            Err(e) => {}
            Ok(Err(e)) => {}
        }
    });
}
