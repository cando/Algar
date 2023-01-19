use crate::Monad;

/// A `State` transformer monad parameterized by the state type (S) and the inner monad (M)
pub struct StateT<'a, S, M> {
    /// The "stateful" function which return a Monad whose Unwrapped value is the tuple (_, S)
    pub runner: Box<dyn 'a + FnOnce(S) -> M>,
}

impl<'a, A: 'a, S: 'a, M: 'a + Monad<'a, Unwrapped = (A, S)>> Monad<'a> for StateT<'a, S, M> {
    type Unwrapped = A;
    type Wrapped<B: 'a> = StateT<'a, S, M::Wrapped<(B, S)>>;

    fn bind<F, B>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> Self::Wrapped<B> + 'a,
    {
        StateT {
            runner: Box::new(move |s| {
                let m = (self.runner)(s);

                let chain_fun = |v| {
                    let (a1, s1) = v;
                    (f(a1).runner)(s1)
                };
                m.bind(chain_fun)
            }),
        }
    }

    fn of<T: 'a>(value: T) -> Self::Wrapped<T> {
        StateT {
            runner: Box::new(move |s| M::of((value, s))),
        }
    }
}

impl<'a, S: 'a, M: 'a + Monad<'a>> StateT<'a, S, M> {
    pub fn new<F>(runner: F) -> Self
    where
        F: FnOnce(S) -> M + 'a,
    {
        Self {
            runner: Box::new(runner),
        }
    }

    pub fn execute(self, state: S) -> M {
        (self.runner)(state)
    }

    pub fn lift<N, B>(base: N) -> StateT<'a, S, <N as Monad<'a>>::Wrapped<(B, S)>>
    where
        N: Monad<'a, Unwrapped = B> + 'a,
    {
        StateT {
            runner: Box::new(|s| base.bind(|a| N::of((a, s)))),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Monad;
    use crate::StateT;

    #[test]
    fn state_t_new_option() {
        let s_t = StateT::new(|s| Option::Some((1, s)));

        assert_eq!(StateT::execute(s_t, "TEST"), Option::Some((1, "TEST")));
    }

    // #[test]
    // fn state_t_new_option_no_compile() {
    //     let s_t = StateT::new(|s| Option::Some((1, s, 2)));

    //     assert_eq!(StateT::execute(s_t, "TEST"), Option::Some((1, "TEST", 2)));
    // }

    #[test]
    fn state_t_bind() {
        let s_t = StateT::new(|s| Option::Some((1, s)));

        let s_t2 = s_t.bind(|a| StateT::new(move |s| Option::Some((a + 41, format!("{}_BAR", s)))));

        assert_eq!(
            (s_t2.runner)("FOO".into()),
            Option::Some((42, "FOO_BAR".into()))
        );
    }

    #[test]
    fn state_t_bind_fail() {
        let s_t = StateT::new(|s: String| Option::Some((1, s)));

        let s_t2 = s_t.bind(|_a| StateT::new(|_s| Option::<(i32, String)>::None));

        assert_eq!((s_t2.runner)("FOO".into()), Option::None);
    }

    #[test]
    fn state_t_lift() {
        let a = Option::Some(42);
        let lifted = StateT::<&str, Option<i32>>::lift(a);
        //lifted type is StateT<&str, Option<(i32, &str)>>>!

        assert_eq!(
            Some((43, "STATE")),
            lifted
                .bind(|a| StateT::new(move |s| Option::Some((a + 1, s))))
                .execute("STATE")
        );

        // But if the value we lift is None, we get...
        let b: Option<i32> = None;

        let lifted2 = StateT::<&str, Option<i32>>::lift(b);

        assert!(lifted2
            .bind(|a| StateT::new(move |s| Option::Some((a + 1, s))))
            .execute("STATE")
            .is_none());
    }
}
