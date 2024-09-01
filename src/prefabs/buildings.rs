use crate::{entities::building::{Building, BuildingType}, game_engine::{entity::Entity, entity_body::EmbodiedEntity}};

pub fn new_building(x: u32, y: u32, building_type: BuildingType) -> Vec<Box<dyn Entity>> {
    let mut building = Building::new(building_type);
    building.body_mut().frame.x = x;
    building.body_mut().frame.y = y;

    vec![
        Box::new(building)
    ]
    
}