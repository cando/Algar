/// A free monad is a construction which allows you to build a monad from any Functor.
/// Like other monads, it is a pure way to represent and manipulate computations.
/// In particular, free monads provide a practical way to:
///
/// * represent stateful computations as data, and run them
/// * build an embedded DSL (domain-specific language)
/// * run a computation using multiple different interpreters
///
use crate::Monad;

/// From https://hackage.haskell.org/package/free-5.2/docs/Control-Monad-Free.html
///
/// data Free f a
///   = Pure a
///   | Free (f (Free f a))
pub enum Free<'a, F, A: 'a>
where
    F: FunctorOnce<'a> + 'a,
{
    Pure(A),
    Free(Box<F::Wrapped<Free<'a, F, A>>>),
}

// Here we do not reuse the generic `Functor` trait since we need a FnOnce fmap operation to avoid lifetime-hell in recursive Free structure
pub trait FunctorOnce<'a> {
    type Unwrapped;
    type Wrapped<B: 'a>: FunctorOnce<'a, Unwrapped = B>;

    fn fmap<F, B>(self, f: F) -> Self::Wrapped<B>
    where
        F: FnOnce(Self::Unwrapped) -> B + 'a;
}

impl<'a, F, A> FunctorOnce<'a> for Free<'a, F, A>
where
    F: FunctorOnce<'a> + 'a,
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
    F: FunctorOnce<'a> + 'a,
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

#[allow(unused)]
pub fn lift_f<'a, F, A>(command: F) -> Free<'a, F, A>
where
    F: FunctorOnce<'a, Unwrapped = A>,
{
    // Free (fmap Pure command)
    Free::Free(Box::new(command.fmap(|a| Free::Pure(a))))
}

#[cfg(test)]
mod test {

    use std::{collections::HashMap, fmt::Display};

    use crate::{m, Free, Free::Pure, Monad};

    use super::{lift_f, FunctorOnce};

    pub enum KeyValF<'a, A> {
        Get(String, Box<dyn 'a + FnOnce(String) -> A>),
        Put(String, String, A),
    }

    type KeyVal<'a, A> = Free<'a, KeyValF<'a, A>, A>;
    type KeyValProg<'a, A> = Free<'a, KeyValF<'a, Free<'a, KeyValF<'a, A>, A>>, A>;

    impl<'a, A: 'a> FunctorOnce<'a> for KeyValF<'a, A> {
        type Unwrapped = A;

        type Wrapped<B: 'a> = KeyValF<'a, B>;

        fn fmap<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
        where
            F: FnOnce(Self::Unwrapped) -> B + 'a,
        {
            match self {
                KeyValF::Get(k, cont) => KeyValF::Get(k, Box::new(move |s| f(cont(s)))),
                KeyValF::Put(k, v, cont) => KeyValF::Put(k, v, f(cont)),
            }
        }
    }

    #[test]
    fn key_val_fmap() {
        let get_key_f = |s| lift_f(KeyValF::Get(s, Box::new(|a| a)));

        let mut get_key_1 = get_key_f("1".to_owned());

        match get_key_1 {
            Free::Pure(_) => panic!(),
            Free::Free(f) => match *f {
                KeyValF::Get(k, next) => {
                    assert_eq!("1", k);
                    match next(k) {
                        Free::Pure(v) => assert_eq!("1", v),
                        Free::Free(_) => panic!(),
                    }
                }

                KeyValF::Put(_, _, _) => panic!(),
            },
        }

        get_key_1 = get_key_f("1".to_owned());
        let get_key_mapped = get_key_1.fmap(|a| a.parse::<i32>().unwrap());

        match get_key_mapped {
            Free::Pure(p) => assert_eq!(p, 1),
            Free::Free(f) => match *f {
                KeyValF::Get(k, next) => {
                    assert_eq!("1", k);
                    match next(k.clone()) {
                        Free::Pure(v) => assert_eq!(1, v), // <------ String has been mapped to Int
                        Free::Free(_) => panic!(),
                    }
                }
                KeyValF::Put(_, _, _) => panic!(),
            },
        }
    }

    fn prog<'a>() -> KeyValProg<'a, String> {
        m! {
            put_key("1", "ue");
            put_key("2", "my love");
            a <- get_key("2");
            Pure(a)
        }
    }

    #[test]
    fn key_val_bind_and_eval() {
        // We have a computation that still has to execute! We can interpret as we want!
        let _p = prog();

        assert_eq!(
            "Put 1,ue\nPut 2,my love\nGet 2\nreturn 2",
            eval_to_string(prog())
        );

        assert_eq!(
            "my love",
            eval_real(prog(), &mut HashMap::<String, _>::new())
        );
    }

    fn get_key<'a>(key: &str) -> KeyVal<'a, String> {
        lift_f(KeyValF::Get(key.into(), Box::new(|a| a)))
    }

    fn put_key<'a>(key: &str, val: &str) -> KeyVal<'a, String> {
        lift_f(KeyValF::Put(key.into(), val.into(), String::new()))
    }

    fn eval_to_string<'a, A, R>(prog: Free<'a, KeyValF<'a, A>, R>) -> String
    where
        R: Display,
    {
        match prog {
            Free::Pure(a) => format!("return {}", a),
            Free::Free(m) => match *m {
                KeyValF::Get(k, cont) => format!("Get {}\n{}", k.clone(), eval_to_string(cont(k))),
                KeyValF::Put(k, v, cont) => format!("Put {},{}\n{}", k, v, eval_to_string(cont)),
            },
        }
    }

    fn eval_real<'a, A, R>(
        prog: Free<'a, KeyValF<'a, A>, R>,
        cache: &mut HashMap<String, String>,
    ) -> R
    where
        R: Display,
    {
        match prog {
            Free::Pure(a) => a,
            Free::Free(m) => match *m {
                KeyValF::Get(k, cont) => {
                    let v = cache.get(&k).unwrap();
                    eval_real(cont(v.clone()), cache)
                }
                KeyValF::Put(k, v, cont) => {
                    let _v = cache.insert(k, v);
                    eval_real(cont, cache)
                }
            },
        }
    }
}
