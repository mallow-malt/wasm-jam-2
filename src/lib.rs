#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
mod gamepad;
mod menus;
mod state;
use wasm4::*;
use gamepad::*;
use menus::*;
use state::*;

#[no_mangle]
fn start() {
    unsafe { *DRAW_COLORS = 0x4321 }

    unsafe {
        *PALETTE = [
            0x000000,
            0x32cd32,
            0x8b0000,
            0xadd8e6,
        ];
    }
}

#[no_mangle]
unsafe fn update() {
    gamepad_update();

    match GAME_STATE {
        0 => main_menu(),
        1 => join_menu(),
        2 => text("Game Started", 0, 0),
        _ => panic!()
    }
}
