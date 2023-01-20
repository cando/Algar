//! Algebraic structures, higher-kinded types and other category theory bad ideas.
//!
//! I wrote this for two reasons: first, mainly as a playground for learning Category Theory and Rust, second to see if it was even possible to
//! implement such general abstract nonsense in Rust.

mod semigroupoid;
pub use crate::semigroupoid::Semigroupoid;

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

mod state;
pub use crate::state::State;

mod writer;
pub use crate::writer::Writer;

mod trans;
pub use crate::trans::ResultT;
pub use crate::trans::StateT;
