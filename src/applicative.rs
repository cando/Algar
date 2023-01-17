use crate::Apply;

pub trait Applicative: Apply {
    fn of(value: Self::Unwrapped) -> Self::Wrapped<Self::Unwrapped>;
}

impl<A> Applicative for Option<A> {
    fn of(value: Self::Unwrapped) -> Self::Wrapped<Self::Unwrapped> {
        Some(value)
    }
}

impl<A, E> Applicative for Result<A, E> {
    fn of(value: Self::Unwrapped) -> Self::Wrapped<Self::Unwrapped> {
        Result::Ok(value)
    }
}

#[cfg(test)]
mod test {
    use crate::Applicative;

    #[test]
    fn option_of() {
        let a: Option<i32> = Option::of(31337);
        assert_eq!(a, Option::Some(31337));
    }

    #[test]
    fn result_of() {
        let a: Result<i32, ()> = Result::of(31337);
        assert_eq!(a, Result::Ok(31337));
    }
}
