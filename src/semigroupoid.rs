use crate::{Apply, Semigroup};

/// A semigroupoid describes some way of composing morphisms on between some
/// collection of objects.
pub trait Semigroupoid {
    /// Take two morphisms and return their composition "the math way".
    /// That is, `(b -> c) -> (a -> b) -> (a -> c)`.
    fn compose(self, other: Self) -> Self;
}

/// Morphisms in Rust type system are normal functions: `Fn/FnMut/FnOnce`,
/// but due to limitations of type system there is no way to implement the `Semigroupoid` trait.
/// Let's stick with this generic `compose` function
#[allow(dead_code)]
pub fn compose<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

impl<A> Semigroupoid for Option<A>
where
    A: Semigroup,
{
    fn compose(self, other: Self) -> Self {
        self.lift_a2(other, |a, b| a.mappend(b))
    }
}

#[cfg(test)]
mod tests {

    use crate::semigroupoid;
    use crate::Semigroup;
    use crate::Semigroupoid;

    #[test]
    fn fun_compose() {
        let f = |a: i32| a.to_string();
        let g = |a: String| a.mappend("BAR".to_string());
        assert_eq!("12BAR".to_string(), semigroupoid::compose(f, g)(12));
    }

    #[test]
    fn option_compose() {
        let a = Option::Some(String::from("FOO"));
        let b = Option::Some(String::from("BAR"));

        assert_eq!("FOOBAR".to_string(), a.compose(b).unwrap());
    }
}
