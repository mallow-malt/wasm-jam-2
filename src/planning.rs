use crate::{game::Game, wasm4::text, sprite::Sprite, assets::HILL};

const PLANNING_MENU: [Option<MenuItem>; 5] = [
    Some(MenuItem {
        sprite: HILL,
        action: None,
        submenu: Some(&BUYING_MENU),
    }),
    Some(MenuItem {
        sprite: HILL,
        action: None,
        submenu: None,
    }),
    Some(MenuItem {
        sprite: HILL,
        action: None,
        submenu: None,
    }),
    Some(MenuItem {
        sprite: HILL,
        action: None,
        submenu: Some(&TROOP_MENU),
    }),
    Some(MenuItem {
        sprite: HILL,
        action: None,
        submenu: None,
    }),
];

const BUYING_MENU: [Option<MenuItem>; 2] = [
    Some(MenuItem {
        sprite: HILL,
        action: None,
        submenu: None,
    }),
    Some(MenuItem {
        sprite: HILL,
        action: None,
        submenu: None,
    }),
];

const TROOP_MENU: [Option<MenuItem>; 2] = [
    Some(MenuItem {
        sprite: HILL,
        action: None,
        submenu: None,
    }),
    Some(MenuItem {
        sprite: HILL,
        action: None,
        submenu: None,
    }),
];

// Buy, Sell, Upgrade, Troops, End
// Buy: Arrow, Tar
// Troops: Squire, Cavalry

enum PlanningMode {
    Menu,
    Buy,
    Sell,
    Upgrade,
}

struct MenuItem {
    sprite: Sprite,
    action: Option<&'static dyn FnMut(&mut Game)>,
    submenu: Option<&'static [Option<MenuItem>]>,
}

pub struct PlanningState {
    mode: PlanningMode,
} 

impl PlanningState {
    pub fn new() -> Self {
        Self { 
            mode: PlanningMode::Menu
        }
    }
}
pub fn planning (game: &mut Game) {
    text("debug", 0, 0);
}