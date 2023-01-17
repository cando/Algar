pub trait Monad<'a> {
    type Unwrapped;
    type Wrapped<T: 'a>;

    fn bind<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> Self::Wrapped<B> + 'a;

    fn of<T: 'a>(value: T) -> Self::Wrapped<T>;
}

impl<'a, A> Monad<'a> for Option<A> {
    type Unwrapped = A;
    type Wrapped<B: 'a> = Option<B>;

    fn bind<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> Self::Wrapped<B> + 'a,
    {
        self.and_then(f)
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
        self.and_then(f)
    }

    fn of<T: 'a>(value: T) -> Self::Wrapped<T> {
        Result::Ok(value)
    }
}

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
