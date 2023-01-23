#![doc = include_str!("../README.md")]

mod semigroupoid;
pub use crate::semigroupoid::Semigroupoid;

mod category;
pub use crate::category::Category;

mod semigroup;
pub use crate::semigroup::Semigroup;

mod monoid;
pub use crate::monoid::Monoid;

mod functor;
pub use crate::functor::Functor;

mod apply;
pub use crate::apply::Apply;

mod applicative;
pub use crate::applicative::Applicative;

mod monad;
pub use crate::monad::Monad;

mod foldable;
pub use crate::foldable::Foldable;

mod traversable;
pub use crate::traversable::Traversable;

mod state;
pub use crate::state::State;

mod writer;
pub use crate::writer::Writer;

mod trans;
pub use crate::trans::ResultT;
pub use crate::trans::StateT;
