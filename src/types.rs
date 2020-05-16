use std::ops::Add;
use std::ops::AddAssign;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for Position {
    fn from(tuple: (f64, f64)) -> Position {
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

impl Add<Velocity> for Position {
    type Output = Self;

    fn add(mut self, other: Velocity) -> Self {
        self.x += other.x;
        self.y += other.y;
        self
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

impl Velocity {
    pub fn zero() -> Velocity {
        Velocity { x: 0.0, y: 0.0 }
    }
}

impl From<(f64, f64)> for Velocity {
    fn from(tuple: (f64, f64)) -> Velocity {
        Velocity {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Acceleration {
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for Acceleration {
    fn from(tuple: (f64, f64)) -> Acceleration {
        Acceleration {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl Acceleration {
    pub fn zero() -> Acceleration {
        Acceleration { x: 0.0, y: 0.0 }
    }
}

impl AddAssign<Acceleration> for Velocity {
    fn add_assign(&mut self, other: Acceleration) {
        self.x += other.x;
        self.y += other.y;
    }
}
