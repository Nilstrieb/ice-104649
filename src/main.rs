use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub struct JoinHandle<T>(T);

impl<T> Future for JoinHandle<T> {
    type Output = Result<T, ()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}

pub fn spawn<T: Future>(future: T) -> JoinHandle<T::Output> {
    loop {}
}

use fut::StreamExt;
mod fut {
    use std::{
        future::Future,
        pin::Pin,
        task::{Context, Poll},
    };

    pub trait Stream {
        type Item;
        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
    }

    #[derive(Debug, Clone)]
    pub struct Iter<I> {
        iter: I,
    }

    pub fn iter<I>(i: I) -> Iter<I::IntoIter>
    where
        I: IntoIterator,
    {
        Iter {
            iter: i.into_iter(),
        }
    }

    impl<I> Stream for Iter<I>
    where
        I: Iterator,
    {
        type Item = I::Item;

        fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<I::Item>> {
            Poll::Ready(self.iter.next())
        }
    }

    impl<T: ?Sized> StreamExt for T where T: Stream {}

    fn assert_future<F, T>(t: T) -> T {
        t
    }

    pub trait StreamExt: Stream {
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

        fn buffer_unordered(self, n: impl Into<Option<usize>>) -> BufferUnordered<Self>
        where
            Self::Item: Future,
            Self: Sized,
        {
            loop {}
        }
    }

    #[must_use = "streams do nothing unless polled"]
    pub struct Map<St, F> {
        stream: St,
        f: F,
    }

    impl<St, F> Stream for Map<St, F>
    where
        St: Stream,
        F: FnMut(St::Item),
    {
        type Item = F::Output;

        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            loop {}
        }
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
            self(arg)
        }
    }

    pub trait FnMut1<A>: FnOnce1<A> {
        fn call_mut(&mut self, arg: A) -> Self::Output;
    }

    impl<T, A, R> FnMut1<A> for T
    where
        T: FnMut(A) -> R,
    {
        fn call_mut(&mut self, arg: A) -> R {
            self(arg)
        }
    }

    impl<St, F> Stream for &Map<St, F>
    where
        St: Stream,
        F: FnMut1<St::Item>,
    {
        type Item = F::Output;

        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            loop {}
        }
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

        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            loop {}
        }
    }
}

fn main() {
    let bodies = fut::iter([])
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
