use crate::Apply;

/// `Applicative` extends `Apply` with the ability to lift value into a
/// particular data type or "context".
pub trait Applicative<'a>: Apply<'a> {
    /// Lift a value into a context
    fn of<T: 'a>(value: T) -> Self::Wrapped<T>;
}

impl<'a, A: 'a + Clone> Applicative<'a> for Option<A> {
    fn of<T: 'a>(value: T) -> Self::Wrapped<T> {
        Some(value)
    }
}

impl<'a, A: 'a + Clone, E> Applicative<'a> for Result<A, E> {
    fn of<T: 'a>(value: T) -> Self::Wrapped<T> {
        Result::Ok(value)
    }
}

#[cfg(test)]
mod test {
    use crate::Applicative;

    #[test]
    fn option_of() {
        let a = Option::<i32>::of(31337);
        assert_eq!(a, Option::Some(31337));
    }

    #[test]
    fn result_of() {
        let a = Result::<i32, ()>::of(31337);
        assert_eq!(a, Result::Ok(31337));
    }
}
