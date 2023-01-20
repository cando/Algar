use crate::{Semigroup, Semigroupoid};

/// A category is some collection of objects and relationships (morphisms) between them.
///
/// This idea is captured by the notion of an identity function for objects,
/// and the ability to compose relationships between objects.
pub trait Category: Semigroupoid {
    // A function which take something and return it again.
    fn identity(self) -> Self;
}

impl<A: Semigroup> Category for Option<A> {
    fn identity(self) -> Self {
        self
    }
}

mod tests {
    #[allow(unused_imports)]
    use crate::Category;

    #[test]
    fn option_compose() {
        let a = Option::Some(String::from("FOO"));

        assert_eq!(Option::Some(String::from("FOO")), a.identity());
    }
}
