trait Project {
    type Assoc;
}

pub struct Wrap<T>(T);

impl<T> Project for Wrap<T> {
    type Assoc = Result<T, ()>;
}

pub trait Stream {
    type Item;

    fn for_each<Fut, F>(self, f: F)
    where
        F: FnMut(Self::Item) -> Fut,
        Self: Sized,
    {
        loop {}
    }
}

pub fn map<T, F>(f: F) -> Map<F>
where
    F: FnMut(()) -> T,
{
    loop {}
}

pub struct Map<F>(F);

impl<F> Stream for Map<F>
where
    F: FnOnce1,
    F::Output: Project,
{
    type Item = <F::Output as Project>::Assoc;
}

pub trait FnOnce1 {
    type Output;
}

impl<T, R> FnOnce1 for T
where
    T: FnOnce(()) -> R,
{
    type Output = R;
}

fn main() {
    let bodies = map(|url| Wrap(Result::Ok(url)));

    bodies.for_each(|b| async {
        match b {
            Ok(Ok(url)) => {}
            Err(e) => {}
            Ok(Err(e)) => {}
        }
    });
}
