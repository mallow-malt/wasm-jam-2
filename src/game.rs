use crate::{zombie::Zombie, coord::Coord};

pub struct Game {
    zombies: Vec<Zombie>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            zombies: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.zombies.push(Zombie {
            curr_x: 9.5,
            curr_y: 10.5,
            last: Coord(10, 10),
        })
    }

    pub fn update(&mut self) {}
}
