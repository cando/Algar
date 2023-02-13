/// `Monad` provides a way to link actions, and a way
/// to bring plain values into the correct context (`Applicative`).

/// `Monad` should "derive" from `Applicative` and add only the `bind` operation, but due to the limits of the Rust type system
/// (Higher-kinded Types are missing), we should define a dedicated trait to be able to (partially) support Monad Transformers and avoid
/// some type madness
pub trait Monad<'a> {
    type Unwrapped;
    type Wrapped<T: 'a>;

    /// Sequentially compose actions, piping values through successive function chains.
    ///
    /// The applied linking function must be unary and return data in the same
    /// type of container as the input. The chain function essentially "unwraps"
    /// a contained value, applies a linking function that returns
    /// the initial (wrapped) type, and collects them into a flat(ter) structure.
    fn bind<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> Self::Wrapped<B> + 'a;

    /// Lift a value into a context
    fn of<T: 'a>(value: T) -> Self::Wrapped<T>;
}

impl<'a, A> Monad<'a> for Option<A> {
    type Unwrapped = A;
    type Wrapped<B: 'a> = Option<B>;

    fn bind<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> Self::Wrapped<B> + 'a,
    {
        match self {
            Some(x) => f(x),
            None => None,
        }

        // self.and_then(f)
    }

    fn of<T: 'a>(value: T) -> Self::Wrapped<T> {
        Some(value)
    }
}

impl<'a, A, E> Monad<'a> for Result<A, E> {
    type Unwrapped = A;
    type Wrapped<B: 'a> = Result<B, E>;

    fn bind<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> Self::Wrapped<B> + 'a,
    {
        match self {
            Result::Ok(x) => f(x),
            Result::Err(e) => Result::Err(e),
        }

        // self.and_then(f)
    }

    fn of<T: 'a>(value: T) -> Self::Wrapped<T> {
        Result::Ok(value)
    }
}

// Took from: https://docs.rs/do-notation/latest/do_notation/

/// Provides the Haskell monadic syntactic sugar `do`.
#[macro_export]
macro_rules! m {

// let-binding
(let $p:pat = $e:expr ; $($r:tt)*) => {{
  let $p = $e;
  m!($($r)*)
}};

// const-bind
(_ <- $x:expr ; $($r:tt)*) => {
  $x.bind(move |_| { m!($($r)*) })
};

// bind
($binding:ident <- $x:expr ; $($r:tt)*) => {
  $x.bind(move |$binding| { m!($($r)*) })
};

// const-bind
($e:expr ; $($a:tt)*) => {
    $e.bind(move |_| m!($($a)*))
};

// pure
($a:expr) => {
  $a
}
}

#[cfg(test)]
mod test {
    use crate::Monad;

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

    #[test]
    fn result_macro() {
        let r: Result<i32, &str> = m! {
          v <- Ok(3);
          Ok(v)
        };

        assert_eq!(r, Ok(3));

        let r: Result<i32, &str> = m! {
          v <- r;
          x <- Ok(10);
          Ok(v * x)
        };

        assert_eq!(r, Ok(30));

        let n: Result<i32, &str> = Err("error");
        let r: Result<i32, &str> = m! {
          v <- Ok(314);
          x <- n;
          Ok(v * x)
        };

        assert_eq!(r, Err("error"));

        let r = m! {
          Result::<&str, &str>::Ok("a");
          b <- Ok("b");
          _ <- Result::<&str, &str>::Err("nope");
          Ok(b)
        };

        assert_eq!(r, Err("nope"));
    }
}
