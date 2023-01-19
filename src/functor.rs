pub trait Functor<'a> {
    type Unwrapped;
    type Wrapped<B: 'a>;

    fn fmap<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> B + 'a;
}

impl<'a, A> Functor<'a> for Option<A> {
    type Unwrapped = A;
    type Wrapped<B: 'a> = Option<B>;

    fn fmap<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> B,
    {
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
        F: FnOnce(Self::Unwrapped) -> B,
    {
        match self {
            Result::Ok(a) => Result::Ok(f(a)),
            Result::Err(e) => Result::Err(e),
        }
    }
}

// https://varkor.github.io/blog/2019/03/28/idiomatic-monads-in-rust.html
// we need generic associated trait to have a unique abstraction which covers Vecs too.

// impl<A> Functor for Vec<A> {
//     type Unwrapped = A;

//     type Wrapped<B> = Vec<B>;

//     fn fmap<F, B>(self, f: F) -> Self::Wrapped<B>
//     where
//         F: Fn(Self::Unwrapped) -> B,
//     {
//         self.into_iter().map(f).collect()
//     }
// }

//
// EVIL FUNCTOR IMPLEMENTATION
//
// impl<A> Functor for Option<A> {
//     type Unwrapped = A;
//     type Wrapped<B> = Result<B, ()>;

//     fn fmap<F, B>(self, f: F) -> Self::Wrapped<B>
//     where
//         F: FnOnce(Self::Unwrapped) -> B,
//     {
//         match self {
//             Some(a) => Result::Ok(f(a)),
//             None => Result::Err(()),
//         }
//     }
// }

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
}
