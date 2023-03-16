use crate::Functor;

// data Free f a
//   = Pure a
//   | Free (f (Free f a))

#[derive(Debug, PartialEq)]
pub enum Free<'a, F, A: 'a>
where
    F: Functor<'a> + 'a,
{
    Pure(A),
    Free(Box<F::Wrapped<Free<'a, F, A>>>),
}

impl<'a, F, A> Functor<'a> for Free<'a, F, A>
where
    F: Functor<'a> + 'a,
{
    type Unwrapped = A;
    type Wrapped<B: 'a> = Free<'a, F::Wrapped<Free<'a, F, A>>, B>;

    fn fmap<G, B: 'a>(self, f: G) -> Self::Wrapped<B>
    where
        G: Fn(Self::Unwrapped) -> B + 'a + Copy,
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

fn lift_f<'a, F, A>(command: F) -> Free<'a, F, A>
where
    F: Functor<'a, Unwrapped = A>,
{
    // Free (fmap Pure command)
    Free::Free(Box::new(command.fmap(Free::Pure)))
}

#[cfg(test)]
mod test {

    use crate::{Free, Functor};

    use super::lift_f;
    pub enum KeyVal<'a, A> {
        Get(String, Box<dyn 'a + Fn(String) -> A>),
        Put(String, String, A),
    }

    impl<'a, A: 'a> Functor<'a> for KeyVal<'a, A> {
        type Unwrapped = A;

        type Wrapped<B: 'a> = KeyVal<'a, B>;

        fn fmap<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
        where
            F: Fn(Self::Unwrapped) -> B + 'a,
        {
            match self {
                KeyVal::Get(k, cont) => KeyVal::Get(k, Box::new(move |s| f(cont(s)))),
                KeyVal::Put(k, v, cont) => KeyVal::Put(k, v, f(cont)),
            }
        }
    }

    #[test]
    fn key_val_fmap() {
        let get_key_f = |s| lift_f(KeyVal::Get(s, Box::new(|a| a)));

        let get_key_1 = get_key_f("1".to_owned());

        match &get_key_1 {
            Free::Pure(_) => panic!(),
            Free::Free(f) => match &**f {
                KeyVal::Get(k, next) => {
                    assert_eq!("1", k);
                    match next(k.clone()) {
                        Free::Pure(v) => assert_eq!("1", v),
                        Free::Free(_) => panic!(),
                    }
                }

                KeyVal::Put(_, _, _) => panic!(),
            },
        }

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
}
