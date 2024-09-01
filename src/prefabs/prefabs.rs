use crate::{entities::building::BuildingType, game_engine::entity::Entity};

use super::{house_single_floor::new_house_single_floor, house_two_floors::new_house_two_floors};

pub fn new_building(source_world_id: u32, x: i32, y: i32, building_type: BuildingType) -> Vec<Box<dyn Entity>> {
    match building_type {
        BuildingType::House => new_house_single_floor(source_world_id, x, y),
        BuildingType::HouseTwoFloors => new_house_two_floors(source_world_id, x, y),
    }
}