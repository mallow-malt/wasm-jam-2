#[cfg(feature = "buddy-alloc")]
mod alloc;
mod assets;
mod wasm4;
mod game;
mod zombie;
mod coord;
mod color_state;

use std::ops::{Add, Sub};
use wasm4::*;
use game::Game;
use lazy_static::lazy_static;
use std::sync::Mutex;
use zombie::*;
use coord::*;
use color_state::*;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

#[derive(Debug, Copy, Clone)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn mag(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

const DIRECTIONS: [Coord; 4] = [Coord(-1, 0), Coord(1, 0), Coord(0, -1), Coord(0, 1)];

fn find_target_direction(curr_coord: Coord, last_coord: Coord) -> Option<Coord> {
    // trace(format!("At tile {:?}", curr_coord));
    for offset in DIRECTIONS {
        let targ = curr_coord + offset;

        // trace(format!(
        //     "Looking at tile {:?}, is_hill {}, is_last_coord {}, offset {:?}, is_hill(0, 0) {}",
        //     targ,
        //     is_hill(targ.0, targ.1),
        //     targ == last_coord,
        //     offset,
        //     is_hill(0, 0)
        // ));

        if targ.0 >= 0
            && targ.1 >= 0
            && targ.0 < PLAYABLE_TILES_X
            && targ.1 < PLAYABLE_TILES_Y
            && !is_hill(targ.0, targ.1)
            && targ != last_coord
        {
            // trace("Returned offset");
            return Some(offset);
        }
    }

    None
}

fn world_to_screen(vec: Vec3) -> (i32, i32) {
    let x1 = ((vec.x - 1f32) * 0.5 * ((assets::TILE_WIDTH) as f32)).round() as i32;
    let x2 = (vec.y * (-0.5 * ((assets::TILE_WIDTH) as f32))).round() as i32;
    let y1 = ((vec.x - 1f32) * 0.25 * ((assets::TILE_HEIGHT * 2) as f32)).round() as i32;
    let y2 = (vec.y * 0.25 * ((assets::TILE_HEIGHT * 2) as f32)).round() as i32;
    let z = (vec.z * ((assets::TILE_HEIGHT) as f32)).round() as i32;
    (x1 + x2, y1 + y2 - z)
}

static mut ZOMBIES: Vec<Zombie> = Vec::new();

// Top right is 0,0
#[rustfmt::skip]
const MAP_03: [u32; 21] = [
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

#[no_mangle]
fn start() {
    GAME.lock().expect("game_state").start();
    
    unsafe {
        ZOMBIES.push(Zombie {
            curr_x: 9.5,
            curr_y: 10.5,
            last: Coord(10, 10),
        });
    }

    set_draw_colors(0x4321);

    set_pallet([0x5a3921, 0x6b8c42, 0x7bc67b, 0xffffb5]);
}

fn draw_hill(x: f32, y: f32) {
    let pos = world_to_screen(Vec3 { x, y, z: 0. });

    blit(
        &assets::HILL,
        pos.0 + ORIGIN_X,
        pos.1 + ORIGIN_Y,
        assets::HILL_WIDTH,
        assets::HILL_HEIGHT,
        assets::HILL_FLAGS,
    );
}

const BOARD_WIDTH: i32 = 160;
const ORIGIN_X: i32 = BOARD_WIDTH / 2;
const ORIGIN_Y: i32 = assets::TILE_HEIGHT as i32;
const PLAYABLE_TILES_X: i32 = 21;
const PLAYABLE_TILES_Y: i32 = 21;

fn is_hill(x: i32, y: i32) -> bool {
    (MAP_03[x as usize] >> y) & 1 == 1
}

#[no_mangle]
fn update() {
    GAME.lock().expect("game_state").update();

    set_draw_colors(0x30);

    let padding_tiles_x = 6;
    let padding_tiles_y = 6;

    let cull_min_x = 0;
    let cull_max_x = 160;
    let cull_min_y = 0;
    let cull_max_y = 160;

    set_draw_colors(0x31);
    
    for x in -padding_tiles_x..PLAYABLE_TILES_X + padding_tiles_x {
        for y in -padding_tiles_y..PLAYABLE_TILES_Y + padding_tiles_y {
            let vec = Vec3 {
                x: x as f32,
                y: y as f32,
                z: 0.0,
            };
            let pos = world_to_screen(vec);
            let playable_tile = x >= 0 && x < PLAYABLE_TILES_X && y >= 0 && y < PLAYABLE_TILES_Y;
            let draw_colors = if playable_tile { 0x310 } else { 0x210 };
            set_draw_colors(draw_colors);

            let blit_pos_x = pos.0 + ORIGIN_X;
            let blit_pos_y = pos.1 + ORIGIN_Y + 5;

            if blit_pos_x > cull_max_x
                || blit_pos_x < cull_min_x - (assets::TILE_WIDTH as i32)
                || blit_pos_y > cull_max_y
                || blit_pos_y < cull_min_y - (assets::TILE_HEIGHT as i32)
            {
                continue;
            }

            blit(
                &assets::TILE,
                blit_pos_x,
                blit_pos_y,
                assets::TILE_WIDTH,
                assets::TILE_HEIGHT,
                assets::TILE_FLAGS,
            );
        }
    }

    set_draw_colors(0x3210);

    // blit(
    //     &assets::HILL,
    //     pos.0 + ORIGIN_X,
    //     pos.1 + ORIGIN_Y,
    //     assets::HILL_WIDTH,
    //     assets::HILL_HEIGHT,
    //     assets::HILL_FLAGS,
    // );

    // Back towers
    {
        blit(
            &assets::TOWER,
            4,
            5,
            assets::TOWER_WIDTH,
            assets::TOWER_HEIGHT,
            assets::TOWER_FLAGS,
        );

        blit(
            &assets::TOWER,
            160 - 4 - assets::TOWER_WIDTH as i32,
            5,
            assets::TOWER_WIDTH,
            assets::TOWER_HEIGHT,
            assets::TOWER_FLAGS,
        );
    }

    assert_eq!(MAP_03.len() as i32, PLAYABLE_TILES_Y);

    for x in 0..PLAYABLE_TILES_X {
        for y in 0..PLAYABLE_TILES_Y {
            if is_hill(x, y) {
                draw_hill(x as f32, y as f32);
            }
        }
    }

    // Front towers
    {
        blit(
            &assets::TOWER,
            4,
            160 - 3 - assets::TOWER_HEIGHT as i32,
            assets::TOWER_WIDTH,
            assets::TOWER_HEIGHT,
            assets::TOWER_FLAGS,
        );

        blit(
            &assets::TOWER,
            160 - 4 - assets::TOWER_WIDTH as i32,
            160 - 3 - assets::TOWER_HEIGHT as i32,
            assets::TOWER_WIDTH,
            assets::TOWER_HEIGHT,
            assets::TOWER_FLAGS,
        );
    }

    set_draw_colors(0x4310);

    blit(
        &assets::PORTAL,
        (80 - assets::PORTAL_WIDTH / 2) as i32,
        80,
        assets::PORTAL_WIDTH,
        assets::PORTAL_HEIGHT,
        assets::PORTAL_FLAGS,
    );

    set_draw_colors(0x40);

    unsafe {
        for zombie in ZOMBIES.iter_mut() {
            // trace(format!("zombie {},{}", zombie.curr_x, zombie.curr_y));

            trace(format!("zombie coord {:?}", zombie.curr_coord()));
            let start_coord = zombie.curr_coord();

            match find_target_direction(start_coord, zombie.last) {
                Some(targ) => {
                    // trace(format!("Found tile to move to at {:?}", targ));

                    let motion_x = targ.0 as f32 * ZOMBIE_SPEED;
                    let motion_y = targ.1 as f32 * ZOMBIE_SPEED;
                    zombie.curr_x += motion_x;
                    zombie.curr_y += motion_y;

                    if zombie.curr_coord() != start_coord {
                        zombie.last = start_coord
                    }
                }
                None => {
                    // Do nothing
                    //  Can't fild tile to move to
                    // trace("Can't find tile to move to");
                }
            }

            let pos = world_to_screen(Vec3 {
                x: zombie.curr_x,
                y: zombie.curr_y,
                z: 0.,
            });

            blit(
                &assets::ZOMBIE,
                pos.0 + ORIGIN_X + 5,
                pos.1 + ORIGIN_Y - 2,
                assets::ZOMBIE_WIDTH,
                assets::ZOMBIE_HEIGHT,
                assets::ZOMBIE_FLAGS,
            );
        }
    }

    // unsafe { *_COLORS = 0x4320 }
    // text("Hello from Rust!", 10, 10);

    // let gamepad = unsafe { *GAMEPAD1 };
    // if gamepad & BUTTON_1 != 0 {
    //     unsafe { *DRAW_COLORS = 4 }
    // }
}
