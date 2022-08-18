use crate::{coord::Coord, maps::is_hill, vec3::Vec3, PLAYABLE_TILES_X, PLAYABLE_TILES_Y, positional::Positional};

pub trait Pathing : Positional {
    fn curr_coord(&self) -> Coord;
    fn from_coord(&self) -> Coord;
    fn target(&self) -> Option<Vec3>;
    fn set_from_coord(&mut self, value: Coord);
    fn set_target(&mut self, value: Option<Vec3>);
}

const DIRECTIONS: [Coord; 4] = [Coord(-1, 0), Coord(1, 0), Coord(0, -1), Coord(0, 1)];

fn update_target(entity: &mut dyn Pathing) {
    let start_coord = entity.curr_coord();
    match find_target_direction(start_coord, entity.from_coord()) {
        Some(dir) => {
            entity.set_target(Some(Vec3::from_coord(dir + start_coord)));
            entity.set_from_coord(start_coord);
        }
        None => {
            entity.set_target(None);
        }
    };
}

/// Return a cardinal unit coordinate which represents the first
/// nearby tile which is not a hill and not last_coord
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

pub fn move_along_path(entity: &mut dyn Pathing, distance: f32) {
    let mut distance_to_consume = distance;
    loop {
        match entity.target() {
            Some(targ) => {
                let to_target = targ - entity.position();
                let distance_to_target = to_target.mag();
                if distance_to_target <= distance_to_consume {
                    distance_to_consume -= distance_to_target;
                    entity.set_position(targ);
                    update_target(entity);
                    if entity.target().is_none() {
                        break;
                    }
                } else {
                    entity.set_position(entity.position() + to_target.normalize() * distance_to_consume);
                    break;
                }
            }
            None => {
                update_target(entity);
                if entity.target().is_none() {
                    break;
                }
            }
        }
    }
}
