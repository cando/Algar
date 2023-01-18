use crate::Functor;

pub trait Apply<'a>: Functor<'a> {
    fn ap<F, B: 'a>(self, f: Self::Wrapped<F>) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> B + 'a;

    fn lift_a2<F, B: 'a, C: 'a>(self, b: Self::Wrapped<B>, f: F) -> Self::Wrapped<C>
    where
        F: FnOnce(Self::Unwrapped, B) -> C + 'a;

    // Since Rust doesnt'have (auto)currying, we are forced to manually implement
    // lift_a3, lift_a4, etc.

    // But in Rust we don't neet it, since lift is baked into the language via '?'
    // let a = self?;
    // let b = b?;
    // Some(f(a, b))
}

impl<'a, A> Apply<'a> for Option<A> {
    fn ap<F, B: 'a>(self, f: Self::Wrapped<F>) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> B + 'a,
    {
        self.and_then(|x| f.fmap(|z| z(x)))
    }

    fn lift_a2<F, B: 'a, C: 'a>(self, b: Self::Wrapped<B>, f: F) -> Self::Wrapped<C>
    where
        F: FnOnce(Self::Unwrapped, B) -> C,
    {
        self.and_then(|a_u| b.and_then(|b_u| Some(f(a_u, b_u))))
    }
}

impl<'a, A, E> Apply<'a> for Result<A, E> {
    fn ap<F, B: 'a>(self, f: Self::Wrapped<F>) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> B + 'a,
    {
        self.and_then(|x| f.fmap(|z| z(x)))
    }

    fn lift_a2<F, B: 'a, C: 'a>(self, b: Self::Wrapped<B>, f: F) -> Self::Wrapped<C>
    where
        F: FnOnce(Self::Unwrapped, B) -> C,
    {
        self.and_then(|a_u| b.and_then(|b_u| Result::Ok(f(a_u, b_u))))
    }
}

#[cfg(test)]
mod test {
    use crate::Apply;

    #[test]
    fn option_ap() {
        let a = Option::Some(31337);
        let b = a.ap(Some(|x| format!("{}", x)));
        assert_eq!(b, Option::Some("31337".to_string()));
    }

    #[test]
    fn option_lifta2() {
        let a = Option::Some(42);
        let b = Option::Some(2);
        let res = a.lift_a2(b, |u1, u2| u1 + u2);
        assert_eq!(res, Option::Some(44));
    }

    #[test]
    fn result_ap() {
        let a: Result<i32, ()> = Result::Ok(31337);
        let b = a.ap(Result::Ok(|x| format!("{}", x)));
        assert_eq!(b, Result::Ok("31337".to_string()));
    }
}
