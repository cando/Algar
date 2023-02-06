use crate::{Applicative, Apply, Functor, Monad, Monoid};

/// `Writer` helps capture the pattern of writing to a pure log or accumulated
/// value, handling the book-keeping for you.
/// This is often used for loggers, but could be anything as long as the hidden value
/// is a `Monoid`.
///
/// There are many applications of `Writer`s, but as an illustrative point,
/// one could use it for logging across processes and time, since the log
/// is carried around with the result in a pure fashion. The monadic DSL
/// helps make using these feel more natural.
pub struct Writer<A, W: Monoid> {
    /// The enclosed value and log of the `Writer`.
    runner: (A, W),
}

impl<A, W: Monoid> Writer<A, W> {
    /// Construct a `Writer` struct from a starting value and log.
    pub fn new(value: A, log: W) -> Self {
        Self {
            runner: (value, log),
        }
    }

    /// Construct a `Writer` struct from a log.
    pub fn tell(log: W) -> Writer<(), W> {
        Writer { runner: ((), log) }
    }

    /// Extract the enclosed value and log from an `Writer`.
    pub fn execute(self) -> (A, W) {
        self.runner
    }
}

impl<'a, A, W: Monoid> Functor<'a> for Writer<A, W> {
    type Unwrapped = A;

    type Wrapped<B: 'a> = Writer<B, W>;

    fn fmap<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> B,
    {
        let (a, l) = self.runner;
        Writer::new(f(a), l)
    }
}

impl<'a, A, W: Monoid> Apply<'a> for Writer<A, W> {
    fn ap<F, B: 'a>(self, f: Self::Wrapped<F>) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> B + 'a,
    {
        let (a, l1) = self.runner;
        let (f, l2) = f.runner;
        Writer::new(f(a), l1.mappend(l2))
    }

    fn lift_a2<F, B: 'a, C: 'a>(self, b: Self::Wrapped<B>, f: F) -> Self::Wrapped<C>
    where
        F: FnOnce(Self::Unwrapped, B) -> C,
    {
        let (a1, l1) = self.runner;
        let (a2, l2) = b.runner;
        Writer::new(f(a1, a2), l1.mappend(l2))
    }
}

impl<'a, A: 'a, W: Monoid> Applicative<'a> for Writer<A, W> {
    fn of(value: Self::Unwrapped) -> Self::Wrapped<Self::Unwrapped> {
        Writer::new(value, W::mempty())
    }
}

impl<'a, A: 'a, W: Monoid> Monad<'a> for Writer<A, W> {
    type Unwrapped = A;

    type Wrapped<B: 'a> = Writer<B, W>;

    fn bind<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> Self::Wrapped<B> + 'a,
    {
        let (a1, w1) = self.runner;
        let (a2, w2) = f(a1).runner;

        Writer::new(a2, w1.mappend(w2))
    }

    fn of<T: 'a>(value: T) -> Self::Wrapped<T> {
        Writer::new(value, W::mempty())
    }
}

#[cfg(test)]
mod test {
    use crate::functor::Functor;
    use crate::Apply;
    use crate::Monad;
    use crate::Writer;

    #[test]
    fn option_functor() {
        let a = Writer::new(1, String::from("FOO"));
        let b = a.fmap(|x| format!("{}", x));
        assert_eq!(b.runner, ("1".to_string(), "FOO".to_string()));
    }

    #[test]
    fn writer_ap() {
        let a = Writer::new(1, String::from("FOO"));
        let b = a.ap(Writer::new(|x| format!("{}", x), String::from("BAR")));
        assert_eq!(b.runner, ("1".to_string(), "FOOBAR".to_string()));
    }

    #[test]
    fn writer_lifta2() {
        let a = Writer::new(1, String::from("FOO"));
        let b = Writer::new(1, String::from("BAR"));
        let res = a.lift_a2(b, |u1, u2| u1 + u2);
        assert_eq!(res.runner, (2, "FOOBAR".to_string()));
    }

    #[test]
    fn writer_bind() {
        let w = Writer::new(1, "FOO".to_string());
        let w2 = w.bind(|a| Writer::new(a + 41, "BAR".to_string()));

        assert_eq!(w2.runner, (42, "FOOBAR".into()));
    }
}
