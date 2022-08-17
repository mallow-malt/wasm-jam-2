use crate::{coord::Coord, vec3::Vec3};

pub const ZOMBIE_SPEED: f32 = 0.005;

pub struct Zombie {
    pub position: Vec3,
    pub last: Coord,
    pub target: Option<Vec3>,
}

impl Zombie {
    pub fn curr_coord(&self) -> Coord {
        Coord(self.position.x.floor() as i32, self.position.y.floor() as i32)
    }
}

