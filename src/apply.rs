use crate::Functor;

/// An extension of `Functor`, `Apply` provides a way to _apply_ arguments
/// to functions when both are wrapped in the same kind of container. This can be
/// seen as running function application "in a context".
///
/// For a nice, illustrated introduction,
/// see [Functors, Applicatives, And Monads In Pictures](http://adit.io/posts/2013-04-17-functors,_applicatives,_and_monads_in_pictures.html).
///
pub trait Apply<'a>: Functor<'a> {
    /// Apply a function wrapped in a context to to a value wrapped in the same type of context
    fn ap<F, B: 'a>(self, f: Self::Wrapped<F>) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> B + 'a;

    /// Lift an (unwrapped) binary function and apply to two wrapped values
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

impl<'a, A: Clone> Apply<'a> for Option<A> {
    fn ap<F, B: 'a>(self, f: Self::Wrapped<F>) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> B + 'a,
    {
        self.and_then(|x| f.fmap(|z| z(x.clone())))
    }

    fn lift_a2<F, B: 'a, C: 'a>(self, b: Self::Wrapped<B>, f: F) -> Self::Wrapped<C>
    where
        F: FnOnce(Self::Unwrapped, B) -> C,
    {
        self.and_then(|a_u| b.map(|b_u| f(a_u, b_u)))
    }
}

impl<'a, A: Clone, E> Apply<'a> for Result<A, E> {
    fn ap<F, B: 'a>(self, f: Self::Wrapped<F>) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> B + 'a,
    {
        self.and_then(|x| f.fmap(|z| z(x.clone())))
    }

    fn lift_a2<F, B: 'a, C: 'a>(self, b: Self::Wrapped<B>, f: F) -> Self::Wrapped<C>
    where
        F: FnOnce(Self::Unwrapped, B) -> C,
    {
        self.and_then(|a_u| b.map(|b_u| f(a_u, b_u)))
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
