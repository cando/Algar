extern crate algar;

pub mod model;

/// Examples inspired by [Thirteen ways of looking at a turtle](https://fsharpforfunandprofit.com/posts/13-ways-of-looking-at-a-turtle/)
fn main() {}

#[cfg(test)]
mod test {
    use algar::{m, Monad, ResultT, State, StateT, Writer};

    use crate::model::{
        helpers::{lift_validation, move_turtle, run, set_turtle_color, turn_turtle},
        turtle::{Angle, Distance, PenColor, PenState, Position, Turtle},
        turtleM::{r#move, set_pen_color, turn},
        turtle_api::{self, TurtleError},
    };

    use crate::model::helpers::lift_op;

    #[test]
    fn test_plain_turtle() {
        let t = Turtle::default();

        let new_t = t
            .turn(Angle::new(90.0))
            .r#move(Distance::new(12.0))
            .set_pen_color(PenColor::Red)
            .set_pen_state(PenState::Down);

        assert_eq!(
            new_t,
            Turtle {
                angle: Angle::new(90.0),
                color: PenColor::Red,
                state: PenState::Down,
                position: Position::new(0.0, 12.0)
            }
        )
    }

    #[test]
    fn test_chaining_api_turtle_success() {
        let t = Turtle::default();

        let new_t = turtle_api::turn(t, "90.0")
            .bind(|t| turtle_api::r#move(t, "12.0"))
            .bind(|t| turtle_api::set_pen_color(t, "RED"))
            .bind(|t| turtle_api::set_pen_state(t, "DOWN"));

        assert_eq!(
            new_t.unwrap(),
            Turtle {
                angle: Angle::new(90.0),
                color: PenColor::Red,
                state: PenState::Down,
                position: Position::new(0.0, 12.0)
            }
        )
    }

    #[test]
    fn test_chaining_api_turtle_fail() {
        let t = Turtle::default();

        let new_t = turtle_api::turn(t, "90.0")
            .bind(|t| turtle_api::r#move(t, "NOT_VALID"))
            .bind(|t| turtle_api::set_pen_color(t, "RED"));

        assert!(new_t.is_err())
    }

    #[test]
    fn test_do_notation_api_turtle_fail() {
        let t = Turtle::default();

        let new_t = m! {
            t <- turtle_api::turn(t, "90.0");
            t <- turtle_api::r#move(t, "NOT_VALID");
            turtle_api::set_pen_color(t, "RED")
        };

        assert!(new_t.is_err())
    }

    #[test]
    fn test_state_monad_simple_turtle() {
        let do_move = |p| State::new(move |s_t: Turtle| ((), s_t.r#move(Distance::new(p))));
        let do_turn = |p| State::new(move |s_t: Turtle| ((), s_t.turn(Angle::new(p))));
        let do_set_pen_color = |p| State::new(move |s_t: Turtle| ((), s_t.set_pen_color(p)));
        let do_set_pen_state = |p| State::new(move |s_t: Turtle| ((), s_t.set_pen_state(p)));

        let new_t = State::execute(
            m! {
                do_turn(90.0);
                do_move(12.0);
                do_set_pen_color(PenColor::Red);
                do_set_pen_state(PenState::Down)
            },
            Turtle::default(),
        );

        assert_eq!(
            new_t.1,
            Turtle {
                angle: Angle::new(90.0),
                color: PenColor::Red,
                state: PenState::Down,
                position: Position::new(0.0, 12.0)
            }
        )
    }

    // #[test]
    // fn test_state_monad_api_turtle() {
    //     let do_move = |p| State::new(move |s_t: Turtle| ((), turtle_api::r#move(s_t, p)));

    //     // It's not going to work since we need to unwrap automagically the Result<_,_> which is returned by turtle_api calls
    //     // Monad transformers FTW!
    //     let new_t = State::execute(
    //         m! {
    //             do_move("12.0");
    //         },
    //         Turtle::default(),
    //     );
    // }

    #[test]
    fn test_state_monad_trans_api_turtle() {
        let do_move = |p| StateT::new(move |s_t: Turtle| turtle_api::r#move2(s_t, p));
        let do_turn = |p| StateT::new(move |s_t: Turtle| turtle_api::turn2(s_t, p));

        let new_t = StateT::execute(
            m! {
                do_turn("90.0");
                do_move("12.0")
            },
            Turtle::default(),
        );

        assert!(new_t.is_ok());

        assert_eq!(
            new_t.unwrap().1,
            Turtle {
                angle: Angle::new(90.0),
                color: PenColor::Black,
                state: PenState::Up,
                position: Position::new(0.0, 12.0)
            }
        );

        let fail_t = StateT::execute(
            m! {
                do_move("12.0");
                do_turn("90.0");
                do_move("NAAAAAAAAAAAAA");
                do_move("12.0")
            },
            Turtle::default(),
        );

        assert!(fail_t.is_err());
    }

    #[test]
    fn test_state_either_and_writer_trans_api_turtle() {
        let do_move = |p| {
            StateT::new(move |s_t: Turtle| {
                ResultT::new(Writer::new(
                    turtle_api::r#move2(s_t, p),
                    format!("moving {}\n", p),
                ))
            })
        };

        let do_turn = |p| {
            StateT::new(move |s_t: Turtle| {
                ResultT::new(Writer::new(
                    turtle_api::turn2(s_t, p),
                    format!("turning {}\n", p),
                ))
            })
        };

        let new_t = Writer::execute(ResultT::execute(StateT::execute(
            m! {
                do_turn("90.0");
                do_move("12.0")
            },
            Turtle::default(),
        )));

        assert!(new_t.0.is_ok());

        assert_eq!(
            new_t.0.unwrap().1,
            Turtle {
                angle: Angle::new(90.0),
                color: PenColor::Black,
                state: PenState::Up,
                position: Position::new(0.0, 12.0)
            }
        );

        assert_eq!(new_t.1, "turning 90.0\nmoving 12.0\n");
    }

    #[test]
    fn test_monad_trans_lift() {
        let log = |log: &str| {
            StateT::<Turtle, ResultT<Writer<Result<(), TurtleError>, String>>>::lift(ResultT::lift(
                Writer::<Turtle, _>::tell(String::from(log)),
            ))
        };

        let validate_distance = lift_validation(turtle_api::validate_distance);
        let _validate_pen_color = lift_validation(turtle_api::validate_pen_color);
        let validate_angle = lift_validation(turtle_api::validate_angle);

        let do_move = lift_op(Turtle::r#move);
        let do_turn = lift_op(Turtle::turn);
        let _do_set_pen_color = lift_op(Turtle::set_pen_color);
        let _do_set_pen_state = lift_op(Turtle::set_pen_state);

        let comp_t = m! {
            log("turning 90.0\n");
            angle <- validate_angle("90.0");
            do_turn(angle);

            log("moving 12.0\n");
            distance <- validate_distance("12.0");
            do_move(distance)
        };

        let new_t = Writer::execute(ResultT::execute(StateT::execute(comp_t, Turtle::default())));

        assert_eq!(
            new_t.0.unwrap().1,
            Turtle {
                angle: Angle::new(90.0),
                color: PenColor::Black,
                state: PenState::Up,
                position: Position::new(0.0, 12.0)
            }
        );
        assert_eq!(new_t.1, "turning 90.0\nmoving 12.0\n");
    }

    #[test]
    fn test_moar_simple_final_solution() {
        //  And here we are finally. Ultra-simply fully pure functional (but looks like imperative) code
        //  handling state + either + writer monad! <3

        let comp_t = m! {
            turn_turtle("90.0");
            move_turtle("12.0");
            set_turtle_color("RED")
        };

        let new_t = run(comp_t, Turtle::default());

        assert_eq!(
            new_t.0.unwrap().1,
            Turtle {
                angle: Angle::new(90.0),
                color: PenColor::Red,
                state: PenState::Up,
                position: Position::new(0.0, 12.0)
            }
        );
        assert_eq!(new_t.1, "turning 90.0\nmoving 12.0\ncoloring pen to  RED\n");
    }

    #[test]
    fn test_final_solution_yes_we_handle_failure() {
        let comp_t = m! {
            turn_turtle("90.0");
            move_turtle("NOT_VALID!!!!!!!!!!!!!");
            set_turtle_color("RED")
        };

        let new_t = run(comp_t, Turtle::default());

        assert_eq!(new_t.0.err().unwrap(), TurtleError::InvalidDistance);
    }

    #[test]
    fn test_new_type_turtle_m() {
        // Hide transformer stack complexity behind a new type: TurtleM
        // And now we are back to example `fn test_plain_turtle()` with a similar syntax, but much more power underneath!
        let turtle_moves = m! {
            turn("90.0");
            r#move("12.0");
            set_pen_color("RED")
        };

        let turtle_run = run(turtle_moves, Turtle::default());

        assert_eq!(
            turtle_run.0.unwrap().1,
            Turtle {
                angle: Angle::new(90.0),
                color: PenColor::Red,
                state: PenState::Up,
                position: Position::new(0.0, 12.0)
            }
        );
        assert_eq!(
            turtle_run.1,
            "turning 90.0\nmoving 12.0\ncoloring pen to  RED\n"
        );
    }
}
