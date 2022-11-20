use fut::Stream;
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

pub fn spawn<T>(future: T) -> JoinHandle<T> {
    loop {}
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

        fn buffer_unordered(self, n: impl Into<Option<usize>>) -> BufferUnordered<Self>
        where
            Self::Item: Future,
            Self: Sized,
        {
            loop {}
        }
    }

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

    impl<St, F> Stream for &Map<St, F>
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
    let bodies = fut::iter([])
        .map(|url: String| spawn(Result::Ok(url)))
        .buffer_unordered(0);

    bodies.for_each(|b| async {
        match b {
            Ok(Ok(url)) => {}
            Err(e) => {}
            Ok(Err(e)) => {}
        }
    });
}
/*
capture1 = CapturedPlace {
    place: Place {
        base_ty: std::result::Result<std::result::Result<std::string::String, _>, ()>,
        base: Upvar(UpvarId(HirId { owner: OwnerId { def_id: DefId(0:100 ~ ice_104649[7838]::main) }, local_id: 40 };`b`;DefId(0:104 ~ ice_104649[7838]::main::{closure#1}::{closure#0}))),
        projections: [Projection { ty: (), kind: Field(0, 1) }]
    },
    info: CaptureInfo { capture_kind_expr_id: Some(HirId { owner: OwnerId { def_id: DefId(0:100 ~ ice_104649[7838]::main) }, local_id: 45 }), path_expr_id: Some(HirId { owner: OwnerId { def_id: DefId(0:100 ~ ice_104649[7838]::main) }, local_id: 45 }), capture_kind: ByRef(ImmBorrow) },
    mutability: Not, region: Some('_#8r)
}
capture2 = CapturedPlace {
    place: Place {
        base_ty: std::result::Result<std::result::Result<std::string::String, _>, ()>,
        base: Upvar(UpvarId(HirId { owner: OwnerId { def_id: DefId(0:100 ~ ice_104649[7838]::main) },local_id: 40 };`b`;DefId(0:104 ~ ice_104649[7838]::main::{closure#1}::{closure#0}))),
        projections: [Projection { ty: std::result::Result<std::string::String, _>, kind: Field(0, 0) }, Projection { ty: std::string::String, kind: Field(0, 0) }]
    },
    info: CaptureInfo { capture_kind_expr_id: Some(HirId { owner: OwnerId { def_id: DefId(0:100 ~ ice_104649[7838]::main) }, local_id: 45 }), path_expr_id: Some(HirId { owner: OwnerId { def_id: DefId(0:100 ~ ice_104649[7838]::main) }, local_id: 45 }), capture_kind: ByValue },
    mutability: Not, region: None
}
*/
