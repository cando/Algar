use crate::Monad;

pub struct ResultT<M> {
    pub runner: M,
}

impl<'a, M> ResultT<M> {
    pub fn new(runner: M) -> Self {
        Self { runner }
    }

    pub fn execute(self) -> M {
        self.runner
    }

    pub fn lift<E>(
        base: M,
    ) -> ResultT<<M as Monad<'a>>::Wrapped<Result<<M as Monad<'a>>::Unwrapped, E>>>
    where
        M: Monad<'a>,
    {
        ResultT {
            runner: base.bind(|a| M::of(Result::Ok(a))),
        }
    }
}

impl<'a, M: 'a + Monad<'a, Unwrapped = Result<A, E>>, A: 'a, E: 'a> Monad<'a> for ResultT<M> {
    type Unwrapped = A;

    type Wrapped<C: 'a> = ResultT<M::Wrapped<Result<C, E>>>;

    fn bind<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> Self::Wrapped<B> + 'a,
    {
        ResultT::new(self.runner.bind(|r| match r {
            Ok(ok) => f(ok).runner,
            Err(err) => M::of(Result::Err(err)),
        }))
    }

    fn of<T: 'a>(value: T) -> Self::Wrapped<T> {
        ResultT::new(M::of(Result::Ok(value)))
    }
}

#[cfg(test)]
mod test {
    use crate::Monad;
    use crate::ResultT;

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
        let lifted = ResultT::lift::<String>(a);
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
        let lifted2 = ResultT::lift::<String>(b);

        assert!(lifted2
            .bind(|a| ResultT::new(Option::Some(Ok(a + 1))))
            .execute()
            .is_none());
    }
}
