/// A `Semigroup` is a type with an associative operation. In plain terms, this
/// means you can take two values of this type and add them together into a
/// different value of the same type. The most obvious example of this is
/// addition of numbers: `2 + 2 = 4`, another is string concatenation: `"Hello "
/// + "Joe" = "Hello Joe"`.
///
/// Semigroups must follow the law of associativity:
/// * `(x + y) + z = x + (y + z)`
///
/// A `Semigroup` differs from `std::ops::Add` in that `Add` can be defined
/// for any collection of types, eg. you could define `Add` for a type `A` which
/// takes a second argument of type `B` and returns a third type `C`, whereas a
/// `Semigroup` only deals with a single type `A`.
pub trait Semigroup {
    fn mappend(self, other: Self) -> Self;
}

impl Semigroup for String {
    fn mappend(self, other: Self) -> Self {
        self + &other
    }
}

impl<A> Semigroup for Vec<A> {
    fn mappend(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}
