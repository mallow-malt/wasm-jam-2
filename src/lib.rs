#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
use wasm4::*;

// hill
const HILL_WIDTH: u32 = 16;
const HILL_HEIGHT: u32 = 13;
const HILL_FLAGS: u32 = 1; // BLIT_2BPP
const HILL: [u8; 52] = [
    0x00, 0x01, 0x40, 0x00, 0x00, 0x17, 0xd4, 0x00, 0x01, 0x7f, 0xfd, 0x40, 0x17, 0xff, 0xff, 0xd4,
    0x7f, 0xff, 0xff, 0xfd, 0x57, 0xff, 0xff, 0xd5, 0x69, 0x7f, 0xfd, 0x69, 0x6a, 0x97, 0xd6, 0xa9,
    0x6a, 0xa9, 0x6a, 0xa9, 0x16, 0xaa, 0x6a, 0x94, 0x01, 0x6a, 0x69, 0x40, 0x00, 0x16, 0x54, 0x00,
    0x00, 0x01, 0x40, 0x00,
];


// tile
const TILE_WIDTH: u32 = 16;
const TILE_HEIGHT: u32 = 8;
const TILE_FLAGS: u32 = 0; // BLIT_1BPP
const TILE: [u8; 16] = [ 0xf9,0x9f,0xe7,0xe7,0x9f,0xf9,0x7f,0xfe,0x9f,0xf9,0xe7,0xe7,0xf9,0x9f,0xfe,0x7f ];

#[derive(Debug, Copy, Clone)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

fn world_to_screen(vec: Vec3) -> (i32, i32) {
    let x1 = (vec.x*0.5*((TILE_WIDTH) as f32)).round() as i32;
    let x2 = (vec.y*(-0.5*((TILE_WIDTH) as f32))).round() as i32;
    let y1 = (vec.x*0.25*((TILE_HEIGHT*2) as f32)).round() as i32;
    let y2 = (vec.y*0.25*((TILE_HEIGHT*2) as f32)).round() as i32;
    let z = (vec.z * ((TILE_HEIGHT) as f32)).round() as i32;
    (x1+ x2, y1 + y2 - z)
}


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
            0x5a3921,
            0x6b8c42,
            0x7bc67b,
            0xffffb5,
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
    unsafe { *DRAW_COLORS = 0x4321 }
    
    // unsafe { *DRAW_COLORS = 0x42 }
    for x in 0..10 {
        for y in 0..10 {
            let vec = Vec3 {
                x: x as f32,
                y: y as f32,
                z: 0.0
            };
            let pos = world_to_screen(vec);
            blit(
                &TILE,
                pos.0 + 64,
                pos.1 + 5,
                TILE_WIDTH,
                TILE_HEIGHT,
                TILE_FLAGS,
            );
        }
    }

    let pos = world_to_screen(Vec3 { x: 0., y: 0., z: 0. });

    // oval(pos.0 + 64 + ((TILE_WIDTH/2) as i32), pos.1 + 5, 4, 4);

    unsafe { *DRAW_COLORS = 0x3210 }
    
    blit(&HILL, pos.0 + 64, pos.1, HILL_WIDTH, HILL_HEIGHT, HILL_FLAGS);

    // unsafe { *DRAW_COLORS = 0x4320 }
    // text("Hello from Rust!", 10, 10);

    // let gamepad = unsafe { *GAMEPAD1 };
    // if gamepad & BUTTON_1 != 0 {
    //     unsafe { *DRAW_COLORS = 4 }
    // }

    // blit(&SMILEY, 76, 76, 8, 8, BLIT_1BPP);
    // text("Press X to blink", 16, 90);

}
