use crate::{vec3::Vec3, positional::Positional};

pub struct Portal {
    pub position: Vec3,
}

impl Positional for Portal {
    fn get_position(&self) -> Vec3 {
        self.position
    }
}
