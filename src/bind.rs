use crate::Applicative;

pub trait Bind<'a>: Applicative<'a> {
    fn bind<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> Self::Wrapped<B> + 'a;
}

impl<'a, A: 'a> Bind<'a> for Option<A> {
    fn bind<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> Self::Wrapped<B> + 'a,
    {
        self.and_then(f)
    }
}

impl<'a, A: 'a, E> Bind<'a> for Result<A, E> {
    fn bind<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> Self::Wrapped<B> + 'a,
    {
        self.and_then(f)
    }
}

#[cfg(test)]
mod test {
    use crate::Bind;

    #[test]
    fn option_bind() {
        let a = Option::Some(31337);
        let b = a.bind(|x| Some(format!("{}", x)));
        assert_eq!(b, Option::Some("31337".to_string()));
    }

    #[test]
    fn option_chain() {
        let a = Option::Some(31337);
        let b: Option<i32> = a.bind(|x| Some(format!("{}", x))).bind(|_| Option::None);
        assert_eq!(b, Option::None);

        let c = a.bind(|x| Some(format!("{}", x))).bind(|mut y| {
            y.push_str("foo");
            Some(y)
        });
        assert_eq!(c, Option::Some("31337foo".to_string()));
    }

    #[test]
    fn result_bind() {
        let a: Result<i32, ()> = Result::Ok(31337);
        let b = a.bind(|x| Result::Ok(format!("{}", x)));
        assert_eq!(b, Result::Ok("31337".to_string()));
    }
}
