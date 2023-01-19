use crate::{Applicative, Apply, Functor, Monad};

/// `State` describes a wrapped function that can be used to pass around some
/// "hidden" pure state.
/// This has numerous applications, but the primary advantage is purity. The state
/// gets passed around with the value, and the monadic DSL helps it feel more
/// natural than passing everything around by hand.
/// Simulates a global mutable state by means of composition of pure functions.

/// This module is inspired by the paper Functional Programming with Overloading and Higher-Order Polymorphism,
/// Mark P Jones <http://web.cecs.pdx.edu/~mpj/> Advanced School of Functional Programming, 1995.
pub struct State<'a, S, A> {
    /// The (apparently) "stateful" function
    pub runner: Box<dyn 'a + FnOnce(S) -> (A, S)>,
}

impl<'a, S: 'a, A: 'a> State<'a, S, A> {
    /// Constructs a new `State` by passing in the function which uses and updates the state
    pub fn new<F>(runner: F) -> Self
    where
        F: FnOnce(S) -> (A, S) + 'a,
    {
        Self {
            runner: Box::new(runner),
        }
    }

    /// Run a `State` by passing in some initial state to actualy run the enclosed
    /// state runner.
    pub fn execute(self, state: S) -> (A, S) {
        (self.runner)(state)
    }
}

impl<'a, S: 'a, A: 'a> Functor<'a> for State<'a, S, A> {
    type Unwrapped = A;

    type Wrapped<B: 'a> = State<'a, S, B>;

    fn fmap<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> B + 'a,
    {
        State {
            runner: Box::new(move |s| {
                let (a1, s1) = (self.runner)(s);
                (f(a1), s1)
            }),
        }
    }
}

impl<'a, S: 'a, A: 'a> Apply<'a> for State<'a, S, A> {
    fn ap<F, B: 'a>(self, f: Self::Wrapped<F>) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> B + 'a,
    {
        State {
            runner: Box::new(move |s| {
                let (a1, s1) = (self.runner)(s);
                let (f, s2) = (f.runner)(s1);
                (f(a1), s2)
            }),
        }
    }

    fn lift_a2<F, B: 'a, C: 'a>(self, b: Self::Wrapped<B>, f: F) -> Self::Wrapped<C>
    where
        F: FnOnce(Self::Unwrapped, B) -> C + 'a,
    {
        State {
            runner: Box::new(move |s| {
                let (a1, s1) = (self.runner)(s);
                let (a2, s2) = (b.runner)(s1);
                (f(a1, a2), s2)
            }),
        }
    }
}

impl<'a, S: 'a, A: 'a> Applicative<'a> for State<'a, S, A> {
    fn of<T: 'a>(value: T) -> Self::Wrapped<T> {
        State::new(|s| (value, s))
    }
}

impl<'a, S: 'a, A: 'a> Monad<'a> for State<'a, S, A> {
    type Unwrapped = A;

    type Wrapped<B: 'a> = State<'a, S, B>;

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
        State::new(|s| (value, s))
    }
}

#[cfg(test)]
mod test {
    use crate::Apply;
    use crate::Functor;
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
    fn state_ap() {
        let s = State {
            runner: Box::new(|s| (12, s)),
        };
        let b = State {
            runner: Box::new(|s| (|x| format!("{}", x), s)),
        };

        let res = s.ap(b);

        assert_eq!(res.execute("FOO"), ("12".to_string(), "FOO"));
    }

    #[test]
    fn state_lifta2() {
        let s = State {
            runner: Box::new(|s| (12, s)),
        };
        let b = State {
            runner: Box::new(|s| (14, format!("{}BAR", s))),
        };

        let res = s.lift_a2(b, |x, y| x + y);

        assert_eq!(res.execute("FOO".to_string()), (26, "FOOBAR".to_string()));
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
