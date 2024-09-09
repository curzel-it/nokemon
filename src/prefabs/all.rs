use crate::{entities::{known_species::{SPECIES_HOUSE_1, SPECIES_HOUSE_2, SPECIES_HOUSE_3, SPECIES_HOUSE_TWO_FLOORS_1, SPECIES_HOUSE_TWO_FLOORS_2, SPECIES_HOUSE_TWO_FLOORS_3}, species::Species}, game_engine::entity::Entity};

use super::{house_single_floor::new_house_single_floor, house_two_floors::new_house_two_floors};

pub fn new_building(source_world_id: u32, x: i32, y: i32, species: &Species) -> Vec<Entity> {
    match species.id {
        SPECIES_HOUSE_1 => new_house_single_floor(species, source_world_id, x, y),
        SPECIES_HOUSE_2 => new_house_single_floor(species, source_world_id, x, y),
        SPECIES_HOUSE_3 => new_house_single_floor(species, source_world_id, x, y),
        SPECIES_HOUSE_TWO_FLOORS_1 => new_house_two_floors(species, source_world_id, x, y),
        SPECIES_HOUSE_TWO_FLOORS_2 => new_house_two_floors(species, source_world_id, x, y),
        SPECIES_HOUSE_TWO_FLOORS_3 => new_house_two_floors(species, source_world_id, x, y),
        _ => new_house_single_floor(species, source_world_id, x, y)
    }
}