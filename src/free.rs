use crate::Monad;

// data Free f a
//   = Pure a
//   | Free (f (Free f a))

pub trait Functor0<'a> {
    type Unwrapped;
    type Wrapped<B: 'a>: Functor0<'a, Unwrapped = B>;

    fn fmap<F, B>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> B + 'a;
}

#[derive(Debug, PartialEq)]
pub enum Free<'a, F, A: 'a>
where
    F: Functor0<'a> + 'a,
{
    Pure(A),
    Free(Box<F::Wrapped<Free<'a, F, A>>>),
}

impl<'a, F, A> Functor0<'a> for Free<'a, F, A>
where
    F: Functor0<'a> + 'a,
{
    type Unwrapped = A;
    type Wrapped<B: 'a> = Free<'a, F::Wrapped<Free<'a, F, A>>, B>;

    fn fmap<G, B: 'a>(self, f: G) -> Self::Wrapped<B>
    where
        G: FnOnce(Self::Unwrapped) -> B + 'a,
    {
        match self {
            Free::Pure(a) => Free::Pure(f(a)),
            Free::Free(b) => {
                // Free (fmap g <$> fx)
                Free::Free(Box::new((*b).fmap(move |a| a.fmap(f))))
            }
        }
    }
}

impl<'a, F, A> Monad<'a> for Free<'a, F, A>
where
    F: Functor0<'a> + 'a,
{
    type Unwrapped = A;
    type Wrapped<T: 'a> = Free<'a, F::Wrapped<Free<'a, F, A>>, T>;

    fn bind<E, B: 'a>(self, f: E) -> Self::Wrapped<B>
    where
        E: FnOnce(Self::Unwrapped) -> Self::Wrapped<B> + 'a,
    {
        // Pure a >>= f = f a
        // Free m >>= f = Free ((>>= f) <$> m)
        match self {
            Free::Pure(a) => f(a),
            Free::Free(m) => Free::Free(Box::new((*m).fmap(|a| a.bind(f)))),
        }
    }

    fn of<T: 'a>(value: T) -> Self::Wrapped<T> {
        Free::Pure(value)
    }
}

pub fn lift_f<'a, F, A>(command: F) -> Free<'a, F, A>
where
    F: Functor0<'a, Unwrapped = A>,
{
    // Free (fmap Pure command)
    Free::Free(Box::new(command.fmap(|a| Free::Pure(a))))
}

#[cfg(test)]
mod test {

    use std::fmt::Display;

    use crate::{m, Free, Monad};

    use super::{lift_f, Functor0};

    pub enum KeyVal<'a, A> {
        Get(String, Box<dyn 'a + FnOnce(String) -> A>),
        Put(String, String, Box<dyn 'a + FnOnce() -> A>),
    }

    impl<'a, A: 'a> Functor0<'a> for KeyVal<'a, A> {
        type Unwrapped = A;

        type Wrapped<B: 'a> = KeyVal<'a, B>;

        fn fmap<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
        where
            F: FnOnce(Self::Unwrapped) -> B + 'a,
        {
            match self {
                KeyVal::Get(k, cont) => KeyVal::Get(k, Box::new(move |s| f(cont(s)))),
                KeyVal::Put(k, v, cont) => KeyVal::Put(k, v, Box::new(|| f(cont()))),
            }
        }
    }

    #[test]
    fn key_val_fmap() {
        let get_key_f = |s| lift_f(KeyVal::Get(s, Box::new(|a| a)));

        let mut get_key_1 = get_key_f("1".to_owned());

        match get_key_1 {
            Free::Pure(_) => panic!(),
            Free::Free(f) => match *f {
                KeyVal::Get(k, next) => {
                    assert_eq!("1", k);
                    match next(k) {
                        Free::Pure(v) => assert_eq!("1", v),
                        Free::Free(_) => panic!(),
                    }
                }

                KeyVal::Put(_, _, _) => panic!(),
            },
        }

        get_key_1 = get_key_f("1".to_owned());
        let get_key_mapped = get_key_1.fmap(|a| a.parse::<i32>().unwrap());

        match get_key_mapped {
            Free::Pure(p) => assert_eq!(p, 1),
            Free::Free(f) => match *f {
                KeyVal::Get(k, next) => {
                    assert_eq!("1", k);
                    match next(k.clone()) {
                        Free::Pure(v) => assert_eq!(1, v), // <------ String has been mapped to Int
                        Free::Free(_) => panic!(),
                    }
                }
                KeyVal::Put(_, _, _) => panic!(),
            },
        }
    }

    #[test]
    fn key_val_bind_and_eval() {
        let get_key_f = |s: &str| lift_f(KeyVal::Get(s.into(), Box::new(|a| a)));
        let put_key_f =
            |s: &str, v: &str| lift_f(KeyVal::Put(s.into(), v.into(), Box::new(|| "".into())));

        let comp = m! {
            put_key_f("1", "ue");
            put_key_f("2", "my love");
            a <- get_key_f("1");
            Free::Pure(a)
        };

        // We have a computation that still has to execute! We can interpret as we want!

        assert_eq!(
            "Put 1,ue\nPut 2,my love\nGet 1\nreturn 1",
            eval_to_string(comp)
        );
    }

    fn eval_to_string<'a, A, R>(prog: Free<'a, KeyVal<'a, A>, R>) -> String
    where
        R: Display,
    {
        match prog {
            Free::Pure(a) => format!("return {}", a),
            Free::Free(m) => match *m {
                KeyVal::Get(k, cont) => format!("Get {}\n{}", k.clone(), eval_to_string(cont(k))),
                KeyVal::Put(k, v, cont) => format!("Put {},{}\n{}", k, v, eval_to_string(cont())),
            },
        }
    }
}
