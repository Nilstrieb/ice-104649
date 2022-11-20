use fut::Stream;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub struct FutResult<T>(T);

impl<T> Future for FutResult<T> {
    type Output = Result<T, ()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}

mod fut {
    use std::{
        future::Future,
        pin::Pin,
        task::{Context, Poll},
    };

    pub trait Stream {
        type Item;

        fn map<T, F>(self, f: F) -> Map<Self, F>
        where
            F: FnMut(Self::Item) -> T,
            Self: Sized,
        {
            loop {}
        }

        fn for_each<Fut, F>(self, f: F) -> ForEach<Self, Fut, F>
        where
            F: FnMut(Self::Item) -> Fut,
            Fut: Future<Output = ()>,
            Self: Sized,
        {
            loop {}
        }

        fn buffer_unordered(self) -> BufferUnordered<Self>
        where
            Self::Item: Future,
            Self: Sized,
        {
            loop {}
        }
    }

    pub struct Empty;

    impl Stream for Empty {
        type Item = String;
    }

    pub struct Map<St, F> {
        stream: St,
        f: F,
    }

    pub trait FnOnce1<A> {
        type Output;
        fn call_once(self, arg: A) -> Self::Output;
    }

    impl<T, A, R> FnOnce1<A> for T
    where
        T: FnOnce(A) -> R,
    {
        type Output = R;
        fn call_once(self, arg: A) -> R {
            loop {}
        }
    }

    impl<St, F> Stream for Map<St, F>
    where
        St: Stream,
        F: FnOnce1<St::Item>,
    {
        type Item = F::Output;
    }

    pub struct ForEach<St, Fut, F> {
        stream: St,
        f: F,
        future: Option<Fut>,
    }

    impl<St, Fut, F> Future for ForEach<St, Fut, F>
    where
        St: Stream,
        F: FnMut(St::Item) -> Fut,
        Fut: Future<Output = ()>,
    {
        type Output = ();

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
            loop {}
        }
    }

    pub struct BufferUnordered<St>
    where
        St: Stream,
    {
        stream: St,
    }

    impl<St> Stream for BufferUnordered<St>
    where
        St: Stream,
        St::Item: Future,
    {
        type Item = <St::Item as Future>::Output;
    }
}

fn main() {
    let bodies = fut::Empty
        .map(|url| FutResult(Result::Ok(url)))
        .buffer_unordered();

    bodies.for_each(|b| async {
        match b {
            Ok(Ok(url)) => {}
            Err(e) => {}
            Ok(Err(e)) => {}
        }
    });
}
