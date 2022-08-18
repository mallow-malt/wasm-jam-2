use crate::wasm4::{DRAW_COLORS, PALETTE};

pub fn set_draw_colors(new_state: u16) {
    unsafe {
        DRAW_COLORS.write_volatile(new_state);
    }
}

pub fn set_pallet(new_state: [u32; 4]) {
    unsafe {
        PALETTE.write_volatile(new_state);
    }
}
