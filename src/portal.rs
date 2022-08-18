use crate::{vec3::Vec3, positional::Positional};

pub struct Portal {
    pub position: Vec3,
}

impl Positional for Portal {
    fn position(&self) -> Vec3 {
        self.position
    }

    fn set_position(&mut self, value: Vec3) {
        self.position = value
    }
}
