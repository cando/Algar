use algar::{Applicative, Apply};

use crate::model::turtle::{Angle, Distance, PenColor, PenState, Turtle};

#[derive(PartialEq, Debug)]
pub enum TurtleError {
    InvalidDistance,
    InvalidAngle,
    InvalidPenColor(String),
    InvalidPenState(String),
}

pub fn r#move(turtle: Turtle, distance: &str) -> Result<Turtle, TurtleError> {
    Apply::lift_a2(
        Result::of(turtle),
        validate_distance(distance),
        Turtle::r#move,
    )
}

pub fn r#move2(turtle: Turtle, distance: &str) -> Result<((), Turtle), TurtleError> {
    Apply::lift_a2(
        Result::of(turtle),
        validate_distance(distance),
        Turtle::r#move2,
    )
}

pub fn turn(turtle: Turtle, angle: &str) -> Result<Turtle, TurtleError> {
    Apply::lift_a2(Result::of(turtle), validate_angle(angle), Turtle::turn)
}

pub fn turn2(turtle: Turtle, angle: &str) -> Result<((), Turtle), TurtleError> {
    Apply::lift_a2(Result::of(turtle), validate_angle(angle), Turtle::turn2)
}

pub fn set_pen_color(turtle: Turtle, color: &str) -> Result<Turtle, TurtleError> {
    Apply::lift_a2(
        Result::of(turtle),
        validate_pen_color(color),
        Turtle::set_pen_color,
    )
}

pub fn set_pen_state(turtle: Turtle, state: &str) -> Result<Turtle, TurtleError> {
    Apply::lift_a2(
        Result::of(turtle),
        validate_pen_state(state),
        Turtle::set_pen_state,
    )
}

pub fn validate_distance(d: &str) -> Result<Distance, TurtleError> {
    match d.parse::<f32>() {
        Ok(v) => Result::Ok(Distance::new(v)),
        Err(_) => Result::Err(TurtleError::InvalidDistance),
    }
}

pub fn validate_angle(a: &str) -> Result<Angle, TurtleError> {
    match a.parse::<f32>() {
        Ok(v) => Result::Ok(Angle::new(v)),
        Err(_) => Result::Err(TurtleError::InvalidAngle),
    }
}

pub fn validate_pen_color(c: &str) -> Result<PenColor, TurtleError> {
    match c.to_uppercase().as_str() {
        "RED" => Result::Ok(PenColor::Red),
        "BLACK" => Result::Ok(PenColor::Black),
        c => Result::Err(TurtleError::InvalidPenColor(c.to_string())),
    }
}

pub fn validate_pen_state(s: &str) -> Result<PenState, TurtleError> {
    match s.to_uppercase().as_str() {
        "UP" => Result::Ok(PenState::Up),
        "DOWN" => Result::Ok(PenState::Down),
        s => Result::Err(TurtleError::InvalidPenState(s.to_string())),
    }
}

#[cfg(test)]
mod test {
    use crate::model::{turtle::Turtle, turtle_api::TurtleError};

    #[test]
    fn test_api_move_ok() {
        let t = Turtle::default();

        assert!(super::r#move(t, "42").is_ok());
    }

    #[test]
    fn test_api_move_err() {
        let t = Turtle::default();

        assert_eq!(
            super::r#move(t, "NOT A DISTANCE"),
            Result::Err(TurtleError::InvalidDistance)
        );
    }

    #[test]
    fn test_api_turn_ok() {
        let t = Turtle::default();

        assert!(super::turn(t, "42").is_ok());
    }

    #[test]
    fn test_api_turn_err() {
        let t = Turtle::default();

        assert_eq!(
            super::turn(t, "NOT A ANGLE"),
            Result::Err(TurtleError::InvalidAngle)
        );
    }
}
