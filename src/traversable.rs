use crate::{Foldable, Functor};

/// `Traversable` represents data structures which can be traversed while perserving the shape.
/// Helpful to walk through a data structure from left to right,
/// running some action on each element in turn.
/// Similar to applicatives, it can be used to do things like collecting some effects
pub trait Traversable<'a>: Foldable + Functor<'a> {}
