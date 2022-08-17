use crate::vec3::Vec3;

pub const TILE_WIDTH: f32 = 16.;
pub const TILE_HEIGHT: f32 = 8.; 

pub fn world_to_screen(vec: Vec3) -> (i32, i32) {
    let x1 = ((vec.x - 1f32) * 0.5 * TILE_WIDTH).round() as i32;
    let x2 = (vec.y * (-0.5 * TILE_WIDTH)).round() as i32;
    let y1 = ((vec.x - 1f32) * 0.25 * (TILE_HEIGHT * 2.)).round() as i32;
    let y2 = (vec.y * 0.25 * (TILE_HEIGHT * 2.)).round() as i32;
    let z = (vec.z * TILE_HEIGHT).round() as i32;
    (x1 + x2, y1 + y2 - z)
}
