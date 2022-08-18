use crate::vec3::Vec3;

pub trait Positional {
    fn position(&self) -> Vec3;
    fn set_position(&mut self, value: Vec3);
}
