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

#[cfg(test)]
mod test {
    // use crate::{Free, Functor};

    // pub enum KeyVal<'a, A> {
    //     Get(String, &'a dyn Fn(String) -> A),
    //     Put(String, String, A),
    // }

    // impl<'a, A: 'a> Functor<'a> for KeyVal<'a, A> {
    //     type Unwrapped = A;

    //     type Wrapped<B: 'a> = KeyVal<'a, B>;

    //     fn fmap<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
    //     where
    //         F: Fn(Self::Unwrapped) -> B + 'a + Copy,
    //     {
    //         match self {
    //             KeyVal::Get(k, cont) => {
    //                 let c = move |s| f(cont(s));

    //                 KeyVal::Get(k, &c)
    //             }
    //             KeyVal::Put(k, v, cont) => todo!(),
    //         }
    //     }
    // }

    #[test]
    fn free_option() {
        // let a = Free::<Toy<String, ()>, Toy<String, ()>>::Pure(Toy::Done);

        // let b: Free<Toy<_, _>, String> = a.fmap(|x| "done".to_string());
        // dbg!(b);
    }
}
