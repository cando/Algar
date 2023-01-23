use crate::{Applicative, Foldable, Functor, Monoid};

/// `Traversable` represents data structures which can be traversed while perserving the shape.
/// Helpful to walk through a data structure from left to right,
/// running some action on each element in turn.
/// Similar to applicatives, it can be used to do things like collecting some effects
pub trait Traversable<'a> {
    type Inner;
    /// Convert elements to actions, and then evaluate the actions from left-to-right,
    /// and accumulate the results.
    ///
    /// Haskell signature
    /// traverse  :: Applicative f => (a -> f b) -> t a -> f (t b)
    fn traverse<F, B: 'a, U, W>(self, f: F) -> W::Wrapped<Vec<B>>
    where
        U: Applicative<'a, Unwrapped = B> + IntoIterator<Item = B>,
        U::Wrapped<B>: IntoIterator<Item = U::Wrapped<B>>,
        F: Fn(&Self::Inner) -> U::Wrapped<B>,
        W: Applicative<'a, Unwrapped = Vec<B>, Wrapped<Vec<B>> = W>
            + 'a
            + IntoIterator<Item = Vec<Self::Inner>>,
        <W as Functor<'a>>::Wrapped<Vec<B>>: Applicative<'a> + Monoid,
        <W as Functor<'a>>::Wrapped<B>: FromIterator<U::Wrapped<B>>;
}

impl<'a, A: Monoid> Traversable<'a> for Vec<A> {
    type Inner = A;
    fn traverse<F, B: 'a, U, W>(self, f: F) -> W::Wrapped<Vec<B>>
    where
        U: Applicative<'a, Unwrapped = B> + IntoIterator<Item = B>,
        U::Wrapped<B>: IntoIterator<Item = U::Wrapped<B>>,
        F: Fn(&Self::Inner) -> U::Wrapped<B>,
        W: Applicative<'a, Unwrapped = Vec<B>, Wrapped<Vec<B>> = W>
            + 'a
            + IntoIterator<Item = Vec<Self::Inner>>,
        <W as Functor<'a>>::Wrapped<Vec<B>>: Applicative<'a> + Monoid,
        <W as Functor<'a>>::Wrapped<B>: FromIterator<U::Wrapped<B>>,
    {
        self.foldr(W::of(vec![]), |k, v| {
            let c = f(v).into_iter().collect();

            k.lift_a2(c, |mut acc, v: B| {
                acc.push(v);
                acc
            })
        })
    }
}
#[cfg(test)]
mod test {
    use crate::Traversable;

    #[test]
    fn test_vec_option_traverse() {
        let a = vec![1, 2, 3];

        let result = a.traverse::<_, _, Option<i32>, Option<Vec<i32>>>(|v| Option::Some(*v));
    }
}
