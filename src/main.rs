use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use futures::{stream, StreamExt};

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

pub trait Stream {
    /// Values yielded by the stream.
    type Item;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

pub struct Iter<I> {
    iter: I,
}

impl<I> Unpin for Iter<I> {}

pub(crate) fn assert_stream<T, S>(stream: S) -> S
where
    S: Stream<Item = T>,
{
    stream
}

pub fn iter<I>(i: I) -> Iter<I::IntoIter>
where
    I: IntoIterator,
{
    assert_stream::<I::Item, _>(Iter {
        iter: i.into_iter(),
    })
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

impl<S: Stream> Stream for &S {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        todo!()
    }
}
impl<S: Stream> Stream for &mut S {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        todo!()
    }
}
trait UwuStream: Stream {
    fn map<T, F>(self, f: F) -> Map<Self, F>
    where
        F: FnMut(Self::Item) -> T,
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

impl<S: Stream> UwuStream for S {}
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.stream.size_hint()
    }
}
pub struct BufferUnordered<St>
where
    St: Stream,
{
    uwu: St,
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
fn main() {
    let bodies = iter([])
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
