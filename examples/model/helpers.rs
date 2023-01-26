use algar::{m, Monad, Monoid, ResultT, StateT, Writer};

use super::{
    turtle::Turtle,
    turtle_api::{self, TurtleError},
};

pub fn lift_op<'a, A: 'a>(
    op_fun: impl Fn(Turtle, A) -> Turtle + 'static,
) -> impl FnOnce(A) -> StateT<'a, Turtle, ResultT<Writer<Result<((), Turtle), TurtleError>, String>>>
{
    move |p: A| {
        StateT::new(move |s| {
            ResultT::lift(Writer::new(
                ((), op_fun(s, p)),
                <String as Monoid>::mempty(),
            ))
        })
    }
}

pub fn lift_validation<'a, A: 'a>(
    validate_fun: impl Fn(String) -> Result<A, TurtleError>,
) -> impl Fn(&str) -> StateT<'a, Turtle, ResultT<Writer<Result<(A, Turtle), TurtleError>, String>>>
{
    move |p: &str| {
        StateT::<'a, Turtle, ResultT<Writer<Result<(A, Turtle), TurtleError>, String>>>::lift(
            ResultT::new(Writer::new(
                validate_fun(p.to_string()),
                <String as Monoid>::mempty(),
            )),
        )
    }
}

pub fn log_info(
    l: String,
) -> StateT<'static, Turtle, ResultT<Writer<Result<((), Turtle), TurtleError>, String>>> {
    StateT::<Turtle, ResultT<Writer<Result<(), TurtleError>, String>>>::lift(ResultT::lift(
        Writer::<Turtle, _>::tell(String::from(l)),
    ))
}

pub fn move_turtle(
    d: &str,
) -> StateT<Turtle, ResultT<Writer<Result<((), Turtle), TurtleError>, String>>> {
    let validate_distance = lift_validation(turtle_api::validate_distance);
    let do_move = lift_op(Turtle::r#move);

    m! {
        log_info(format!("moving {}\n", d));
        distance <- validate_distance(d);
        do_move(distance)
    }
}

pub fn turn_turtle(
    a: &str,
) -> StateT<Turtle, ResultT<Writer<Result<((), Turtle), TurtleError>, String>>> {
    let validate_angle = lift_validation(turtle_api::validate_angle);
    let do_turn = lift_op(Turtle::turn);

    m! {
        log_info(format!("turning {}\n", a));
        angle <- validate_angle(a);
        do_turn(angle)
    }
}

pub fn set_turtle_color(
    c: &str,
) -> StateT<Turtle, ResultT<Writer<Result<((), Turtle), TurtleError>, String>>> {
    let validate_pen_color = lift_validation(turtle_api::validate_pen_color);
    let do_set_pen_color = lift_op(Turtle::set_pen_color);

    m! {
        log_info(format!("coloring pen to  {}\n", c));
        color <- validate_pen_color(c);
        do_set_pen_color(color)
    }
}

pub fn run(
    computation: StateT<Turtle, ResultT<Writer<Result<((), Turtle), TurtleError>, String>>>,
    t: Turtle,
) -> (Result<((), Turtle), TurtleError>, String) {
    Writer::execute(ResultT::execute(StateT::execute(computation, t)))
}
