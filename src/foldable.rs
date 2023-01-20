use crate::Monoid;

/// A `Foldable` is something that can be folded over to change its structure by alter and/or combining elements to a summary value.
/// In other words, it is a type which supports "foldr".
pub trait Foldable {
    // The internal type of the `Foldable` which will be wrapped over
    type Unwrapped: Monoid;

    /// Right-associative fold over a structure to alter and/or reduce
    /// it to a single summary value. The right-association makes it possible to
    /// cease computation on infinite streams of data.
    ///
    /// The folder must be a binary function, with the second argument being the
    /// accumulated value thus far.
    fn foldr<B: Monoid, F>(self, init: B, folder: F) -> B
    where
        F: Fn(B, &Self::Unwrapped) -> B;
}

impl<A: Monoid> Foldable for Vec<A> {
    type Unwrapped = A;

    fn foldr<B, F>(self, init: B, folder: F) -> B
    where
        F: Fn(B, &Self::Unwrapped) -> B,
    {
        self.iter().rfold(init, |acc, a| folder(acc, &a))
    }
}

#[cfg(test)]
mod tests {
    use crate::Foldable;

    #[test]
    fn vec_foldr() {
        let a = vec![1, 2, 3];

        assert_eq!(18, a.foldr(12, |acc, v| acc + v));
    }
}
