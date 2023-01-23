use crate::{Applicative, Foldable, Functor, Monoid};

/// `Traversable` represents data structures which can be traversed while perserving the shape.
/// Helpful to walk through a data structure from left to right,
/// running some action on each element in turn.
/// Similar to applicatives, it can be used to do things like collecting some effects
pub trait Traversable<'a>: Functor<'a> {
    /// Convert elements to actions, then evaluate the actions from left-to-right
    /// and collect the results.
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

    /// Evaluate each action in the structure from left to right, and collect the results
    ///
    /// Haskell signature
    /// sequenceA :: Applicative f => t (f a) -> f (t a)
    fn sequence_a<B: 'a, W>(self) -> W::Wrapped<Self::Wrapped<B>>
    where
        W: Applicative<'a, Unwrapped = Self::Wrapped<B>, Wrapped<Self::Wrapped<B>> = W>
            + 'a
            + Monoid,
        W: Applicative<'a, Wrapped<B> = Self::Unwrapped>,
        <Self as Functor<'a>>::Wrapped<B>: 'a,
        Self::Unwrapped: Applicative<'a> + 'a + Copy;
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

    fn sequence_a<B: 'a, W>(self) -> W::Wrapped<Self::Wrapped<B>>
    where
        W: Applicative<'a, Unwrapped = Self::Wrapped<B>, Wrapped<Self::Wrapped<B>> = W>
            + 'a
            + Monoid,
        <Self as Functor<'a>>::Wrapped<B>: 'a,
        W: Applicative<'a, Wrapped<B> = Self::Unwrapped>,
        Self::Unwrapped: Applicative<'a> + 'a + Copy,
    {
        self.traverse::<_, B, W>(|a| *a)
    }
}

#[cfg(test)]
mod test {
    use crate::Traversable;

    #[test]
    fn test_vec_option_traverse() {
        let a = vec![1, 2, 3];

        let result = a.traverse::<_, _, Option<_>>(|v| Option::Some((*v).to_string()));
        assert_eq!(
            Option::Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
            result
        );
    }

    #[test]
    fn test_vec_option_traverse_fail() {
        let a = vec![1, 2, 3];

        let result =
            a.traverse::<_, _, Option<_>>(|v| if *v > 2 { None } else { Option::Some(*v) });
        assert_eq!(None, result);
    }

    #[test]
    fn test_vec_option_sequence() {
        let a = vec![Option::Some(1), Option::Some(2), Option::Some(3)];

        let result = a.sequence_a::<_, Option<_>>();
        assert_eq!(Option::Some(vec![1, 2, 3]), result);
    }

    #[test]
    fn test_vec_option_sequence_fail() {
        let a = vec![Option::Some(1), None, Option::Some(3)];

        let result = a.sequence_a::<_, Option<_>>();
        assert_eq!(None, result);
    }
}
