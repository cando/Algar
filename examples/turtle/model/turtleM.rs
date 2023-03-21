use algar::{ResultT, StateT, Writer};

use super::{turtle::Turtle, turtle_api::TurtleError};

pub type TurtleM<'a, A> =
    StateT<'a, Turtle, ResultT<Writer<Result<(A, Turtle), TurtleError>, String>>>;

pub fn r#move(d: &str) -> TurtleM<()> {
    super::helpers::move_turtle(d)
}

pub fn turn(a: &str) -> TurtleM<()> {
    super::helpers::turn_turtle(a)
}

pub fn set_pen_color(c: &str) -> TurtleM<()> {
    super::helpers::set_turtle_color(c)
}

pub fn run(computation: TurtleM<()>, t: Turtle) -> (Result<((), Turtle), TurtleError>, String) {
    super::helpers::run(computation, t)
}
