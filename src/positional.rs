use crate::vec3::Vec3;

pub trait Positional {
    fn get_position(&self) -> Vec3;
}
