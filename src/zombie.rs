use crate::{coord::Coord, vec3::Vec3, positional::Positional, path_finding::Pathing};

pub const ZOMBIE_SPEED: f32 = 0.005;

pub struct Zombie {
    pub position: Vec3,
    pub last: Coord,
    pub target: Option<Vec3>,
}

impl Positional for Zombie {
    fn position(&self) -> Vec3 {
        self.position
    }

    fn set_position(&mut self, value: Vec3) {
        self.position = value
    }
}

impl Pathing for Zombie {
    fn curr_coord(&self) -> Coord {
        Coord(self.position.x.floor() as i32, self.position.y.floor() as i32)
    }

    fn from_coord(&self) -> Coord {
        self.last
    }

    fn set_from_coord(&mut self, value: Coord) {
        self.last = value
    }

    fn set_target(&mut self, value: Option<Vec3>) {
        self.target = value
    }

    fn target(&self) -> Option<Vec3> {
        self.target
    }
}
