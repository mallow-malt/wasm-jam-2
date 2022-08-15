#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
use wasm4::*;

#[rustfmt::skip]
const SMILEY: [u8; 8] = [
    0b11000011,
    0b10000001,
    0b00100100,
    0b00100100,
    0b00000000,
    0b00100100,
    0b10011001,
    0b11000011,
];

const MAP_01: [[u8; 4]; 7] = [
    [1,1,1,1],
    [1,1,0,1],
    [1,1,0,1],
    [1,1,0,1],
    [1,0,0,1],
    [1,0,1,1],
    [1,1,1,1]
];

// const MAP_01_START: Pos = {
//     x: 2,
//     y: 5,
// };

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

struct Pos {
    x: i32,
    y: i32,
}

fn render_2d(player_pos: Pos) {

}

#[no_mangle]
fn update() {
    unsafe { *DRAW_COLORS = 0x42 }
    rect(10, 10, 32, 32);
    // unsafe { *DRAW_COLORS = 0x4320 }
    // text("Hello from Rust!", 10, 10);

    // let gamepad = unsafe { *GAMEPAD1 };
    // if gamepad & BUTTON_1 != 0 {
    //     unsafe { *DRAW_COLORS = 4 }
    // }

    // blit(&SMILEY, 76, 76, 8, 8, BLIT_1BPP);
    // text("Press X to blink", 16, 90);

}
