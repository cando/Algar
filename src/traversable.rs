use crate::{Applicative, Foldable, Functor, Monoid};

/// `Traversable` represents data structures which can be traversed while perserving the shape.
/// Helpful to walk through a data structure from left to right,
/// running some action on each element in turn.
/// Similar to applicatives, it can be used to do things like collecting some effects
pub trait Traversable<'a>: Functor<'a> {
    /// Convert elements to actions, and then evaluate the actions from left-to-right,
    /// and accumulate the results.
    ///
    /// Haskell signature
    /// traverse  :: Applicative f => (a -> f b) -> t a -> f (t b)
    fn traverse<F, B: 'a, W>(self, f: F) -> W::Wrapped<Self::Wrapped<B>>
    where
        F: Fn(&Self::Unwrapped) -> W::Wrapped<B>,
        W: Applicative<'a, Unwrapped = Self::Wrapped<B>, Wrapped<Self::Wrapped<B>> = W>
            + 'a
            + Monoid,
        <W as Functor<'a>>::Wrapped<Self::Wrapped<B>>: Applicative<'a>,
        <Self as Functor<'a>>::Wrapped<B>: 'a;

    // ///
    // /// Haskell signature
    // /// sequenceA :: Applicative f => t (f a) -> f (t a)
    // fn sequenceA<B: 'a, W>(self) -> W::Wrapped<Self::Wrapped<B>>
    // where
    //     W: Applicative<'a, Unwrapped = Vec<B>, Wrapped<Vec<B>> = W> + 'a,
    //     <W as Functor<'a>>::Wrapped<Vec<B>>: Applicative<'a> + Monoid,
    //     <W as Functor<'a>>::Wrapped<B>: FromIterator<<W as Functor<'a>>::Wrapped<B>>,
    //     Self::Unwrapped: IntoIterator<Item = W::Wrapped<B>> + Copy;
}

impl<'a, A: Monoid> Traversable<'a> for Vec<A> {
    fn traverse<F, B: 'a, W>(self, f: F) -> W::Wrapped<Self::Wrapped<B>>
    where
        F: Fn(&Self::Unwrapped) -> W::Wrapped<B>,
        W: Applicative<'a, Unwrapped = Self::Wrapped<B>, Wrapped<Self::Wrapped<B>> = W>
            + 'a
            + Monoid,
        <W as Functor<'a>>::Wrapped<Self::Wrapped<B>>: Applicative<'a> + Monoid,
        <Self as Functor<'a>>::Wrapped<B>: 'a,
    {
        self.foldr(W::of(vec![]), |k, v| {
            let c = f(v);

            k.lift_a2(c, |mut acc, v: B| {
                acc.insert(0, v);
                acc
            })
        })
    }

    // fn sequenceA<B: 'a, W>(self) -> W::Wrapped<Self::Wrapped<B>>
    // where
    //     W: Applicative<'a, Unwrapped = Vec<B>, Wrapped<Vec<B>> = W> + 'a,
    //     <W as Functor<'a>>::Wrapped<Vec<B>>: Applicative<'a> + Monoid,
    //     <W as Functor<'a>>::Wrapped<B>: FromIterator<<W as Functor<'a>>::Wrapped<B>>,
    //     Self::Unwrapped: IntoIterator<Item = W::Wrapped<B>> + Copy,
    // {
    //     self.traverse::<_, B, W>(|a| a.into_iter().collect::<W::Wrapped<B>>())
    // }
}

#[cfg(test)]
mod test {
    use crate::Traversable;

    #[test]
    fn test_vec_option_traverse() {
        let a = vec![1, 2, 3];

        let result = a.traverse::<_, _, Option<Vec<String>>>(|v| Option::Some((*v).to_string()));
        assert_eq!(
            Option::Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
            result
        );
    }

    #[test]
    fn test_vec_option_traverse_fail() {
        let a = vec![1, 2, 3];

        let result =
            a.traverse::<_, _, Option<Vec<i32>>>(|v| if *v > 2 { None } else { Option::Some(*v) });
        assert_eq!(None, result);
    }

    // #[test]
    // fn test_vec_option_sequence() {
    //     let a = vec![Option::Some(1), Option::Some(2), Option::Some(3)];

    //     let result = a.sequenceA::<_, Option<Vec<i32>>>();
    //     assert_eq!(Option::Some(vec![1, 2, 3]), result);
    // }
}
