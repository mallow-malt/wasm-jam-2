#[cfg(feature = "buddy-alloc")]
mod alloc;
mod assets;
mod wasm4;
mod game;
mod zombie;
mod coord;
mod color_state;
mod maps;
mod isometric;
mod vec3;

use game::Game;
use lazy_static::lazy_static;
use wasm4::SCREEN_SIZE;
use std::sync::Mutex;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

#[no_mangle]
fn start() {
    GAME.lock().expect("game_state").start();
}

const ORIGIN_X: u32 = SCREEN_SIZE / 2;
const ORIGIN_Y: i32 = assets::TILE_HEIGHT as i32;
const PLAYABLE_TILES_X: i32 = 21;
const PLAYABLE_TILES_Y: i32 = 21;

#[no_mangle]
fn update() {
    GAME.lock().expect("game_state").update();
}
