use fut::Stream;

trait Project {
    type Assoc;
}

pub struct FutResult<T>(T);

impl<T> Project for FutResult<T> {
    type Assoc = Result<T, ()>;
}

mod fut {
    use std::future::Future;

    use crate::Project;

    pub trait Stream {
        type Item;

        fn for_each<Fut, F>(self, f: F) -> ForEach<Self, Fut, F>
        where
            F: FnMut(Self::Item) -> Fut,
            Fut: Future<Output = ()>,
            Self: Sized,
        {
            loop {}
        }
    }

    pub fn map<T, F>(f: F) -> Map<F>
    where
        F: FnMut(String) -> T,
    {
        loop {}
    }

    pub struct Map<F>(F);

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

    impl<F> Stream for Map<F>
    where
        F: FnOnce1<String>,
        F::Output: Project,
    {
        type Item = <F::Output as Project>::Assoc;
    }

    pub struct ForEach<St, Fut, F> {
        stream: St,
        f: F,
        future: Fut,
    }
}

fn main() {
    let bodies = fut::map(|url| FutResult(Result::Ok(url)));

    bodies.for_each(|b| async {
        match b {
            Ok(Ok(url)) => {}
            Err(e) => {}
            Ok(Err(e)) => {}
        }
    });
}
