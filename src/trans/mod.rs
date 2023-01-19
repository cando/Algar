/// A monad transformer is a type constructor which takes a monad as an argument and returns a monad as a result.
/// Monad transformers can be used to compose and stack monads – such as state and exception handling – in a modular way.
mod state_t;
pub use crate::trans::state_t::StateT;

mod result_t;
pub use crate::trans::result_t::ResultT;
