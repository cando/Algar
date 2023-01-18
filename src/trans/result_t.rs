use crate::Monad;

pub struct ResultT<M> {
    pub runner: M,
}

impl<'a, M> ResultT<M> {
    pub fn new(runner: M) -> Self
    where
        M: Monad<'a>,
    {
        Self { runner }
    }

    pub fn execute(self) -> M
    where
        M: Monad<'a>,
    {
        self.runner
    }

    pub fn lift<A: 'a, E: 'a>(base: M) -> ResultT<M::Wrapped<Result<A, E>>>
    where
        M: Monad<'a, Unwrapped = A>,
    {
        ResultT {
            runner: base.bind(|a| M::of(Result::Ok(a))),
        }
    }

    pub fn bind<F, A: 'a, B: 'a, E: 'a>(self, f: F) -> ResultT<M::Wrapped<Result<B, E>>>
    where
        M: Monad<'a, Unwrapped = Result<A, E>>,
        F: FnOnce(A) -> ResultT<M::Wrapped<Result<B, E>>> + 'a,
    {
        ResultT {
            runner: (self.runner.bind(|r| match r {
                Ok(ok) => f(ok).runner,
                Err(err) => M::of(Result::Err(err)),
            })),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::trans::ResultT;

    #[test]
    fn result_t_bind() {
        let r_t = ResultT::new(Option::Some(Result::<i32, String>::Ok(1)));

        let r_t2 = r_t.bind(move |a| ResultT::new(Option::Some(Result::Ok(a + 41))));

        assert_eq!(r_t2.runner, Option::Some(Result::Ok(42)));
    }

    #[test]
    fn result_t_fail() {
        let r_t = ResultT::new(Option::Some(Result::<i32, String>::Ok(1)));
        let r_t2 = r_t.bind(move |_a| ResultT::new(Option::<Result<i32, String>>::None));

        assert_eq!(r_t2.runner, Option::None);
    }

    #[test]
    fn result_t_lift() {
        let a = Option::Some(42);
        let lifted = ResultT::lift::<i32, String>(a);
        // lifted type is ResultT<Option<Result<i32, String>>>!

        assert_eq!(
            Result::Ok(43),
            lifted
                .bind(|a| ResultT::new(Option::Some(Ok(a + 1))))
                .execute()
                .unwrap()
        );

        // But if the value we lift is None, we get...
        let b: Option<i32> = None;
        let lifted2 = ResultT::lift::<i32, String>(b);

        assert!(lifted2
            .bind(|a| ResultT::new(Option::Some(Ok(a + 1))))
            .execute()
            .is_none());
    }
}
