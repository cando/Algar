use std::f32::consts::PI;

#[derive(PartialEq, Debug)]
pub enum PenState {
    Up,
    Down,
}

#[derive(PartialEq, Debug)]
pub enum PenColor {
    Black,
    Red,
}

#[derive(PartialEq, Debug)]
pub struct Position(f32, f32);

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position(x, y)
    }
}

#[derive(PartialEq, Debug)]
pub struct Angle(f32);

impl Angle {
    pub fn new(x: f32) -> Self {
        Angle(x)
    }
}

#[derive(PartialEq, Debug)]
pub struct Distance(f32);

impl Distance {
    pub fn new(x: f32) -> Self {
        Distance(x)
    }
}

#[derive(PartialEq, Debug)]
pub struct Turtle {
    pub position: Position,
    pub angle: Angle,
    pub state: PenState,
    pub color: PenColor,
}

impl Default for Turtle {
    fn default() -> Self {
        Self {
            position: Position(0.0, 0.0),
            angle: Angle(0.0),
            state: PenState::Up,
            color: PenColor::Black,
        }
    }
}

impl Turtle {
    pub fn r#move(self, d: Distance) -> Self {
        Self {
            position: Self::calc_new_position(&d, &self.angle, &self.position),
            ..self
        }
    }

    pub fn r#move2(self, d: Distance) -> ((), Self) {
        (
            (),
            Self {
                position: Self::calc_new_position(&d, &self.angle, &self.position),
                ..self
            },
        )
    }

    pub fn turn(self, angle: Angle) -> Self {
        Self {
            angle: Angle(angle.0),
            ..self
        }
    }

    pub fn turn2(self, angle: Angle) -> ((), Self) {
        (
            (),
            Self {
                angle: Angle(angle.0),
                ..self
            },
        )
    }

    pub fn set_pen_state(self, pen_state: PenState) -> Self {
        Self {
            state: pen_state,
            ..self
        }
    }

    pub fn set_pen_color(self, pen_color: PenColor) -> Self {
        Self {
            color: pen_color,
            ..self
        }
    }

    fn calc_new_position(distance: &Distance, angle: &Angle, cur_position: &Position) -> Position {
        let angle_in_rads = angle.0 * PI / 180.0;
        let new_x = cur_position.0 + distance.0 * angle_in_rads.cos();
        let new_y = cur_position.1 + distance.0 * angle_in_rads.sin();

        Position(new_x.ceil(), new_y.ceil())
    }
}

#[cfg(test)]
mod test {

    use super::{Angle, Distance, PenColor, PenState, Position, Turtle};

    #[test]
    fn test_move() {
        let t = Turtle::default();

        assert_eq!(t.r#move(Distance(1.0)).position, Position(1.0, 0.0));
    }

    #[test]
    fn test_turn() {
        let t = Turtle::default();

        assert_eq!(t.turn(Angle(90.0)).angle, Angle(90.0));
    }

    #[test]
    fn test_state() {
        let t = Turtle::default();

        assert_eq!(t.set_pen_state(PenState::Down).state, PenState::Down);
    }

    #[test]
    fn test_color() {
        let t = Turtle::default();

        assert_eq!(t.set_pen_color(PenColor::Red).color, PenColor::Red);
    }
}
