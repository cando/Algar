use crate::Semigroup;

pub trait Monoid: Semigroup {
    fn mempty() -> Self;
}
impl Monoid for String {
    fn mempty() -> Self {
        String::from("")
    }
}

impl<A> Monoid for Vec<A> {
    fn mempty() -> Self {
        vec![]
    }
}
