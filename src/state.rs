use crate::Monad;

pub struct State<'a, S, A> {
    pub runner: Box<dyn 'a + FnOnce(S) -> (A, S)>,
}

impl<'a, S: 'a, A: 'a> Monad<'a> for State<'a, S, A> {
    type Unwrapped = A;

    type Wrapped<T: 'a> = State<'a, S, T>;

    fn bind<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> Self::Wrapped<B> + 'a,
    {
        State {
            runner: Box::new(move |s| {
                let (a1, s1) = (self.runner)(s);
                let g = f(a1).runner;
                (g)(s1)
            }),
        }
    }

    fn of<T: 'a>(value: T) -> Self::Wrapped<T> {
        State {
            runner: Box::new(|s| (value, s)),
        }
    }
}

impl<'a, S: 'a, A: 'a> State<'a, S, A> {
    pub fn new<F>(runner: F) -> Self
    where
        F: FnOnce(S) -> (A, S) + 'a,
    {
        Self {
            runner: Box::new(runner),
        }
    }

    // Here we don't implement Functor trait not to get dirty with lifetimes the trait
    pub fn fmap<F: 'a, B>(self, f: F) -> State<'a, S, B>
    where
        F: FnOnce(A) -> B,
    {
        State {
            runner: Box::new(move |s| {
                let (a1, s1) = (self.runner)(s);
                (f(a1), s1)
            }),
        }
    }

    pub fn execute(self, state: S) -> (A, S) {
        (self.runner)(state)
    }
}

#[cfg(test)]
mod test {
    use crate::Monad;
    use crate::State;

    #[test]
    fn state_fmap() {
        let s = State {
            runner: Box::new(|s| (12, s)),
        };

        let s2 = s.fmap(|a| a + 2);
        assert_eq!((s2.runner)("DOES_NOT_MATTER"), (14, "DOES_NOT_MATTER"));
    }

    #[test]
    fn state_bind() {
        let s: State<String, i32> = State {
            runner: Box::new(|s| (12, s)),
        };

        let s2 = s.bind(|a| State {
            runner: Box::new(move |s| (a + 30, format!("{}_BAR", s))),
        });

        assert_eq!((s2.runner)("FOO".into()), (42, "FOO_BAR".into()));
    }
}
