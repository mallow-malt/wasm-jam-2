use crate::{vec3::Vec3, assets};

pub fn world_to_screen(vec: Vec3) -> (i32, i32) {
    let x1 = ((vec.x - 1f32) * 0.5 * ((assets::TILE_WIDTH) as f32)).round() as i32;
    let x2 = (vec.y * (-0.5 * ((assets::TILE_WIDTH) as f32))).round() as i32;
    let y1 = ((vec.x - 1f32) * 0.25 * ((assets::TILE_HEIGHT * 2) as f32)).round() as i32;
    let y2 = (vec.y * 0.25 * ((assets::TILE_HEIGHT * 2) as f32)).round() as i32;
    let z = (vec.z * ((assets::TILE_HEIGHT) as f32)).round() as i32;
    (x1 + x2, y1 + y2 - z)
}
