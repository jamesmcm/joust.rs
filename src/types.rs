use std::ops::AddAssign;
// TODO: May need to switch from i32 to f64s

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Position {
    fn from(tuple: (i32, i32)) -> Position {
        Position {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl AddAssign<Velocity> for Position {
    fn add_assign(&mut self, other: Velocity) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

impl Velocity {
    pub fn zero() -> Velocity {
        Velocity { x: 0, y: 0 }
    }
}

impl From<(i32, i32)> for Velocity {
    fn from(tuple: (i32, i32)) -> Velocity {
        Velocity {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Acceleration {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Acceleration {
    fn from(tuple: (i32, i32)) -> Acceleration {
        Acceleration {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl Acceleration {
    pub fn zero() -> Acceleration {
        Acceleration { x: 0, y: 0 }
    }
}

impl AddAssign<Acceleration> for Velocity {
    fn add_assign(&mut self, other: Acceleration) {
        self.x += other.x;
        self.y += other.y;
    }
}
