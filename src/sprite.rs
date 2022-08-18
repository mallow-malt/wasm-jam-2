use crate::{wasm4::blit, color_state::set_draw_colors};

pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub flags: u32,
    pub data: &'static [u8],
    pub colors: u16,
}

impl Sprite {
    pub fn draw(&self, x: i32, y: i32) {
        set_draw_colors(self.colors);
        blit(
            self.data,
            x,
            y,
            self.width,
            self.height,
            self.flags,
        );
    }

    pub fn draw_with_colors(&self, x: i32, y: i32, colors: u16) {
        set_draw_colors(colors);
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
