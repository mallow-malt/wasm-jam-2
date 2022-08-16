// Top right is 0,0
#[rustfmt::skip]
pub const MAP_03: [u32; 21] = [
    0b000000000000010100000,
    0b000000000000010100000,
    0b000000000111110100000,
    0b000000000100000100000,
    0b000000000101111100000,
    0b000000000101000011111,
    0b000000000101000010000,
    0b000000000101000010111,
    0b000000000101000010100,
    0b001111111101111110100,
    0b001000000000000000100,
    0b001011111101111111100,
    0b001010000101000000000,
    0b111010000101000000000,
    0b000010000101000000000,
    0b111110000101000000000,
    0b000001111101000000000,
    0b000001000001000000000,
    0b000001011111000000000,
    0b000001010000000000000,
    0b000001010000000000000,
];

pub fn is_hill(x: i32, y: i32) -> bool {
    (MAP_03[x as usize] >> y) & 1 == 1
}

