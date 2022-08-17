use crate::{
    assets,
    color_state::{set_draw_colors, set_pallet},
    coord::Coord,
    isometric::world_to_screen,
    maps::{is_hill, MAP_03},
    path_finding::move_along_path,
    portal::Portal,
    vec3::Vec3,
    wasm4::SCREEN_SIZE,
    zombie::{Zombie, ZOMBIE_SPEED},
    ORIGIN_X, ORIGIN_Y, PLAYABLE_TILES_X, PLAYABLE_TILES_Y,
};

fn draw_hill(x: f32, y: f32) {
    let pos = world_to_screen(Vec3 { x, y, z: 0. });

    assets::HILL.draw(pos.0 + ORIGIN_X as i32, pos.1 + ORIGIN_Y);
}

pub struct Game {
    zombies: Vec<Zombie>,
    portal: Portal,
}

impl Game {
    pub fn new() -> Self {
        Self {
            zombies: Vec::new(),
            portal: Portal {
                position: Vec3 {
                    x: 10.5,
                    y: 10.5,
                    z: 0.,
                },
            },
        }
    }

    pub fn start(&mut self) {
        self.zombies.push(Zombie {
            position: Vec3 {
                x: 9.5,
                y: 10.5,
                z: 0.,
            },
            last: Coord(10, 10),
            target: None,
        });

        set_draw_colors(0x4321);

        set_pallet([0x5a3921, 0x6b8c42, 0x7bc67b, 0xffffb5]);
    }

    fn render(&self) {
        let padding_tiles_x = 6;
        let padding_tiles_y = 6;

        let cull_min_x = 0;
        let cull_max_x = SCREEN_SIZE;
        let cull_min_y = 0;
        let cull_max_y = SCREEN_SIZE;

        set_draw_colors(0x31);

        for x in -padding_tiles_x..PLAYABLE_TILES_X + padding_tiles_x {
            for y in -padding_tiles_y..PLAYABLE_TILES_Y + padding_tiles_y {
                let vec = Vec3 {
                    x: x as f32,
                    y: y as f32,
                    z: 0.0,
                };
                let pos = world_to_screen(vec);
                let playable_tile =
                    x >= 0 && x < PLAYABLE_TILES_X && y >= 0 && y < PLAYABLE_TILES_Y;
                let draw_colors = if playable_tile { 0x310 } else { 0x210 };
                set_draw_colors(draw_colors);

                let blit_pos_x = pos.0 + ORIGIN_X as i32;
                let blit_pos_y = pos.1 + ORIGIN_Y + 5;

                if blit_pos_x > cull_max_x as i32
                    || blit_pos_x < cull_min_x - (assets::TILE.width as i32)
                    || blit_pos_y > cull_max_y as i32
                    || blit_pos_y < cull_min_y - (assets::TILE.height as i32)
                {
                    continue;
                }

                assets::TILE.draw(blit_pos_x, blit_pos_y);
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
            assets::TOWER.draw(4, 5);

            assets::TOWER.draw(SCREEN_SIZE as i32 - 4 - assets::TOWER.width as i32, 5);
        }

        assert_eq!(MAP_03.len() as i32, PLAYABLE_TILES_Y);

        // trace("Start");
        for depth_level in 0..(PLAYABLE_TILES_X + PLAYABLE_TILES_Y - 1) {
            // 0 = 0,0
            // 1 = 1,0 0,1
            // 2 = 2,0 1,1 0,2
            // 3 = 3,0 2,1 1,2 0,3

            // trace(format!("Start level {}", depth_level));
            for i in 0..(depth_level + 1) {
                let coord = Coord(i, depth_level - i);
                if is_hill(coord.0, coord.1) {
                    draw_hill(coord.0 as f32, coord.1 as f32);
                }
            }
            // trace(format!("End level {}", depth_level));
        }
        // trace("End");

        // for x in 0..PLAYABLE_TILES_X {
        //     for y in 0..PLAYABLE_TILES_Y {
        //         if is_hill(x, y) {
        //             draw_hill(x as f32, y as f32);
        //         }
        //     }
        // }

        // Front towers
        {
            assets::TOWER.draw(4, SCREEN_SIZE as i32 - 3 - assets::TOWER.height as i32);

            assets::TOWER.draw(
                SCREEN_SIZE as i32 - 4 - assets::TOWER.width as i32,
                SCREEN_SIZE as i32 - 3 - assets::TOWER.height as i32,
            );
        }

        set_draw_colors(0x4310);

        {
            let draw_at = world_to_screen(self.portal.position);
            assets::PORTAL.draw(draw_at.0 + ORIGIN_X as i32 + 2, draw_at.1 + ORIGIN_Y - 10);
        }

        set_draw_colors(0x40);

        for zombie in self.zombies.iter() {
            let pos = world_to_screen(zombie.position);

            assets::ZOMBIE.draw(pos.0 + ORIGIN_X as i32 + 5, pos.1 + ORIGIN_Y - 2);
        }
    }

    pub fn update(&mut self) {
        for zombie in self.zombies.iter_mut() {
            move_along_path(zombie, ZOMBIE_SPEED);
        }

        self.render();
    }
}
