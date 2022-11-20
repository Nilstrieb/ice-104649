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
        num::NonZeroUsize,
        ops::DerefMut,
        pin::Pin,
        task::{Context, Poll},
    };

    #[must_use = "streams do nothing unless polled"]
    pub trait Stream {
        type Item;
        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, None)
        }
    }

    impl<S: ?Sized + Stream + Unpin> Stream for &mut S {
        type Item = S::Item;

        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            S::poll_next(Pin::new(&mut **self), cx)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (**self).size_hint()
        }
    }

    impl<P> Stream for Pin<P>
    where
        P: DerefMut + Unpin,
        P::Target: Stream,
    {
        type Item = <P::Target as Stream>::Item;

        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            self.get_mut().as_mut().poll_next(cx)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (**self).size_hint()
        }
    }

    #[derive(Debug, Clone)]
    #[must_use = "streams do nothing unless polled"]
    pub struct Iter<I> {
        iter: I,
    }

    impl<I> Unpin for Iter<I> {}

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

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.iter.size_hint()
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
            Map::new(self, f)
        }

        fn for_each<Fut, F>(self, f: F) -> ForEach<Self, Fut, F>
        where
            F: FnMut(Self::Item) -> Fut,
            Fut: Future<Output = ()>,
            Self: Sized,
        {
            assert_future::<(), _>(ForEach::new(self, f))
        }

        fn buffer_unordered(self, n: impl Into<Option<usize>>) -> BufferUnordered<Self>
        where
            Self::Item: Future,
            Self: Sized,
        {
            BufferUnordered::new(self, n.into())
        }
    }

    #[must_use = "streams do nothing unless polled"]
    pub struct Map<St, F> {
        stream: St,
        f: F,
    }

    impl<St, F> Map<St, F> {
        pub(crate) fn new(stream: St, f: F) -> Self {
            Self { stream, f }
        }
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

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.stream.size_hint()
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

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.stream.size_hint()
        }
    }

    pub struct ForEach<St, Fut, F> {
        stream: St,
        f: F,
        future: Option<Fut>,
    }

    impl<St, Fut, F> ForEach<St, Fut, F>
    where
        St: Stream,
        F: FnMut(St::Item) -> Fut,
        Fut: Future<Output = ()>,
    {
        pub(super) fn new(stream: St, f: F) -> Self {
            Self {
                stream,
                f,
                future: None,
            }
        }
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
        max: Option<NonZeroUsize>,
    }

    impl<St> BufferUnordered<St>
    where
        St: Stream,
        St::Item: Future,
    {
        pub(super) fn new(stream: St, n: Option<usize>) -> Self {
            Self {
                stream: stream,
                max: n.and_then(NonZeroUsize::new),
            }
        }
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

        fn size_hint(&self) -> (usize, Option<usize>) {
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
