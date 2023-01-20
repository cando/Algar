use crate::Semigroup;

/// In abstract algebra, a `Monoid` is a set equipped with an associative binary operation and an identity element.
/// In category theory, a `Monoid` is a "single object category" equipped with two morphisms:
/// - μ: M ⊗ M → M called multiplication (a.k.a the associative operation of the `Semigroup`)
/// - η: I → M called unit (a.k.a the `mempty` defined in this trait)
pub trait Monoid: Semigroup {
    /// The identity element/morphism of the monoid
    fn mempty() -> Self;
}

impl Monoid for String {
    fn mempty() -> Self {
        String::from("")
    }
}

impl Monoid for i32 {
    fn mempty() -> Self {
        0
    }
}

impl<A> Monoid for Vec<A> {
    fn mempty() -> Self {
        vec![]
    }
}
