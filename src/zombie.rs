use crate::coord::Coord;

pub const ZOMBIE_SPEED: f32 = 0.005;

pub struct Zombie {
    pub curr_x: f32,
    pub curr_y: f32,
    pub last: Coord,
}

impl Zombie {
    pub fn curr_coord(&self) -> Coord {
        Coord(self.curr_x.floor() as i32, self.curr_y.floor() as i32)
    }
}

