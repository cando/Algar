use crate::Bind;

pub trait Monad<'a>: Bind<'a> {}

impl<'a, M> Monad<'a> for M where M: Bind<'a> {}

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

    use crate::Bind;

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
