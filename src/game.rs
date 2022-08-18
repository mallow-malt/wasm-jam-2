use crate::{
    assets,
    color_state::set_pallet,
    coord::Coord,
    isometric::world_to_screen,
    maps::{is_hill, MAP_03},
    path_finding::move_along_path,
    portal::Portal,
    positional::Positional,
    vec3::Vec3,
    wasm4::SCREEN_SIZE,
    zombie::{Zombie, ZOMBIE_SPEED},
    ORIGIN_X, ORIGIN_Y, PLAYABLE_TILES_X, PLAYABLE_TILES_Y, arror_tower::ArrowTower,
};

fn draw_hill(x: f32, y: f32) {
    let pos = world_to_screen(Vec3 { x, y, z: 0. });

    assets::HILL.draw(pos.0 + ORIGIN_X as i32, pos.1 + ORIGIN_Y);
}

pub struct Game {
    entities: Vec<Entity>,
}

enum Entity {
    Zombie(Zombie),
    Portal(Portal),
    ArrowTower(ArrowTower),
}

fn entity_depth(entity: &Entity) -> f32 {
    match entity {
        Entity::Zombie(zombie) => zombie.position().closeness(),
        Entity::Portal(portal) => portal.position().closeness(),
        Entity::ArrowTower(arror_tower) => arror_tower.position().closeness(),
    }
}

fn draw_entity(entity: &Entity) {
    match entity {
        Entity::Zombie(zombie) => {
            let pos = world_to_screen(zombie.position);

            assets::ZOMBIE.draw(pos.0 + ORIGIN_X as i32 + 5, pos.1 + ORIGIN_Y - 2);
        }
        Entity::Portal(portal) => {
            let draw_at = world_to_screen(portal.position);
            assets::PORTAL.draw(draw_at.0 + ORIGIN_X as i32 + 2, draw_at.1 + ORIGIN_Y - 10);
        }
        Entity::ArrowTower(arrow_tower) => {
            let pos = world_to_screen(arrow_tower.position);

            assets::ARROW_TOWER.draw(pos.0 + ORIGIN_X as i32 + 4, pos.1 + ORIGIN_Y - 1);
        }
    }
}

impl Game {
    pub fn new() -> Self {
        let mut entities = Vec::new();
        entities.push(Entity::Portal(Portal {
            position: Vec3 {
                x: 10.5,
                y: 10.5,
                z: 0.,
            },
        }));
        Self { entities }
    }

    pub fn start(&mut self) {
        self.entities.push(Entity::Zombie(Zombie {
            position: Vec3 {
                x: 9.5,
                y: 10.5,
                z: 0.,
            },
            last: Coord(10, 10),
            target: None,
        }));
        self.entities.push(Entity::Zombie(Zombie {
            position: Vec3 {
                x: 11.5,
                y: 10.5,
                z: 0.,
            },
            last: Coord(10, 10),
            target: None,
        }));
        self.entities.push(Entity::Zombie(Zombie {
            position: Vec3 {
                x: 10.5,
                y: 9.5,
                z: 0.,
            },
            last: Coord(10, 10),
            target: None,
        }));
        self.entities.push(Entity::Zombie(Zombie {
            position: Vec3 {
                x: 10.5,
                y: 11.5,
                z: 0.,
            },
            last: Coord(10, 10),
            target: None,
        }));

        self.entities.push(Entity::ArrowTower(ArrowTower {
            position: Vec3 {
                x: 5.5,
                y: 4.5,
                z: 0.5,
            }
        }));

        set_pallet([0x5a3921, 0x6b8c42, 0x7bc67b, 0xffffb5]);
    }

    fn render(&self) {
        let padding_tiles_x = 6;
        let padding_tiles_y = 6;

        let cull_min_x = 0;
        let cull_max_x = SCREEN_SIZE;
        let cull_min_y = 0;
        let cull_max_y = SCREEN_SIZE;

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
                let draw_colors = if playable_tile { 0x320 } else { 0x210 };

                let blit_pos_x = pos.0 + ORIGIN_X as i32;
                let blit_pos_y = pos.1 + ORIGIN_Y + 5;

                if blit_pos_x > cull_max_x as i32
                    || blit_pos_x < cull_min_x - (assets::TILE.width as i32)
                    || blit_pos_y > cull_max_y as i32
                    || blit_pos_y < cull_min_y - (assets::TILE.height as i32)
                {
                    continue;
                }

                assets::TILE.draw_with_colors(blit_pos_x, blit_pos_y, draw_colors);
            }
        }

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

        let mut enitity_i: usize = 0;

        for depth_level in 0..(PLAYABLE_TILES_X + PLAYABLE_TILES_Y - 1) {
            while enitity_i < self.entities.len()
                && entity_depth(&self.entities[enitity_i]) < (depth_level as f32 + 0.5)
            {
                draw_entity(&self.entities[enitity_i]);
                enitity_i += 1;
                // WARN: Won't draw entities closer then nearest playable tile
            }
            for i in 0..(depth_level + 1) {
                let coord = Coord(i, depth_level - i);
                if is_hill(coord.0, coord.1) {
                    draw_hill(coord.0 as f32, coord.1 as f32);
                }
            }
        }

        // Front towers
        {
            assets::TOWER.draw(4, SCREEN_SIZE as i32 - 3 - assets::TOWER.height as i32);

            assets::TOWER.draw(
                SCREEN_SIZE as i32 - 4 - assets::TOWER.width as i32,
                SCREEN_SIZE as i32 - 3 - assets::TOWER.height as i32,
            );
        }

        // for entity in self.entities.iter() {
        //     match entity {
        //         Entity::Zombie(zombie) => {
        //             set_draw_colors(0x40);
        //             let pos = world_to_screen(zombie.position);

        //             assets::ZOMBIE.draw(pos.0 + ORIGIN_X as i32 + 5, pos.1 + ORIGIN_Y - 2);
        //         }
        //         Entity::Portal(portal) => {
        //             set_draw_colors(0x4310);
        //             let draw_at = world_to_screen(portal.position);
        //             assets::PORTAL.draw(draw_at.0 + ORIGIN_X as i32 + 2, draw_at.1 + ORIGIN_Y - 10);
        //         }
        //     }
        // }
    }

    pub fn update(&mut self) {
        self.entities
            .sort_by(|a, b| entity_depth(a).partial_cmp(&entity_depth(b)).unwrap());

        for entity in self.entities.iter_mut() {
            match entity {
                Entity::Zombie(zombie) => {
                    move_along_path(zombie, ZOMBIE_SPEED);
                }
                Entity::Portal(_) => {
                    // Do nothing
                }
                Entity::ArrowTower(_) => {
                    // Do nothing
                }
            };
        }

        self.render();
    }
}
