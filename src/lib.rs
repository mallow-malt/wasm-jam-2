#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
mod gamepad;
use wasm4::*;
use gamepad::*;

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

    if PLAYER01_PREVIOUS_GAMEPAD & BUTTON_1 != 0 {
        text("P01 pressing x", 0, 10)
    } else {
        text("P01 not pressing x", 0, 10)
    }

    if PLAYER02_PREVIOUS_GAMEPAD & BUTTON_1 != 0 {
        text("P02 pressing x", 0, 20)
    } else {
        text("P02 not pressing x", 0, 20)
    }

    if PLAYER03_PREVIOUS_GAMEPAD & BUTTON_1 != 0 {
        text("P03 pressing x", 0, 30)
    } else {
        text("P03 not pressing x", 0, 30)
    }

    if PLAYER04_PREVIOUS_GAMEPAD & BUTTON_1 != 0 {
        text("P04 pressing x", 0, 40)
    } else {
        text("P04 not pressing x", 0, 40)
    }
}
