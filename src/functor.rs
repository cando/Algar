/// The `Functor` trait represents the mathematical functor: a mapping between categories in the context of category theory.
/// In practice a functor represents a type that can be mapped over.
///
/// A type f is a Functor if it provides a function fmap which,
/// given any types a and b lets you apply any function from (a -> b) to turn an f a into an f b, preserving the structure of f.
pub trait Functor<'a> {
    /// The inner type which will be mapped over
    type Unwrapped;

    /// Target of the fmap operation. Like `Self`, but has a different wrapped up value underneath.
    type Wrapped<B: 'a>: Functor<'a>;

    /// fmap is used to apply a function of type (a -> b) to a value of type f a, where f is a functor, to produce a value of type f b.
    fn fmap<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> B + 'a;
}

impl<'a, A> Functor<'a> for Option<A> {
    type Unwrapped = A;
    type Wrapped<B: 'a> = Option<B>;

    fn fmap<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> B,
    {
        #[allow(clippy::manual_map)]
        match self {
            Some(a) => Some(f(a)),
            None => None,
        }
    }
}

impl<'a, A, E> Functor<'a> for Result<A, E> {
    type Unwrapped = A;
    type Wrapped<B: 'a> = Result<B, E>;

    fn fmap<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> B,
    {
        match self {
            Result::Ok(a) => Result::Ok(f(a)),
            Result::Err(e) => Result::Err(e),
        }
    }
}

impl<'a, A> Functor<'a> for Vec<A> {
    type Unwrapped = A;
    type Wrapped<B: 'a> = Vec<B>;

    fn fmap<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
    where
        F: Fn(Self::Unwrapped) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::Functor;

    #[test]
    fn option_functor() {
        let a = Option::Some(31337);
        let b = a.fmap(|x| format!("{}", x));
        assert_eq!(b, Option::Some("31337".to_string()));
    }

    #[test]
    fn result_functor() {
        let a: Result<i32, ()> = Result::Ok(31337);
        let b = a.fmap(|x| format!("{}", x));
        assert_eq!(b, Result::Ok("31337".to_string()));
    }

    #[test]
    fn vec_functor() {
        let a = vec![1, 2, 3];
        let b = a.fmap(|x| format!("{}", x));
        assert_eq!(vec!["1", "2", "3"], b);
    }
}
