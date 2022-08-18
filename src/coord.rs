use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Coord(pub i32, pub i32);

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

