use crate::wasm4::blit;

pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub flags: u32,
    pub data: &'static [u8],
}

impl Sprite {
    pub fn draw(&self, x: i32, y: i32) {
        blit(
            self.data,
            x,
            y,
            self.width,
            self.height,
            self.flags,
        );
    }
}
