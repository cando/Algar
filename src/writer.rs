use crate::{Monad, Monoid};

pub struct Writer<A, W: Monoid> {
    runner: (A, W),
}

impl<A, W: Monoid> Writer<A, W> {
    pub fn new(value: A, log: W) -> Self {
        Self {
            runner: (value, log),
        }
    }

    pub fn tell(log: W) -> Writer<(), W> {
        Writer { runner: ((), log) }
    }

    pub fn execute(self) -> (A, W) {
        self.runner
    }
}

impl<'a, A, W: Monoid> Monad<'a> for Writer<A, W> {
    type Unwrapped = A;

    type Wrapped<T: 'a> = Writer<T, W>;

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
    use crate::Monad;
    use crate::Writer;

    #[test]
    fn writer_bind() {
        let w = Writer::new(1, "FOO".to_string());
        let w2 = w.bind(|a| Writer::new(a + 41, "BAR".to_string()));

        assert_eq!(w2.runner, (42, "FOOBAR".into()));
    }
}
