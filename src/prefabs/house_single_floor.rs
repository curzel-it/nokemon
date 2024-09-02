use crate::{constants::{HOUSE_INTERIOR_COLUMNS, HOUSE_INTERIOR_ROWS}, entities::{building::{Building, BuildingType}, teleporter::Teleporter}, game_engine::{entity::Entity, entity_body::EmbodiedEntity, world::World}, maps::{biome_tiles::Biome, constructions_tiles::Construction}, utils::ids::get_next_id};

pub fn new_house_single_floor(variant: i32, source_world_id: u32, x: i32, y: i32) -> Vec<Box<dyn Entity>> {
    let mut building = Building::new(BuildingType::House(variant));
    building.body_mut().frame.x = x;
    building.body_mut().frame.y = y;

    let first_floor_id = get_next_id();
    let mut door = Teleporter::new(first_floor_id);
    door.body_mut().frame.x = x + (building.body().frame.w as f32 / 2.0).ceil() as i32;
    door.body_mut().frame.y = y + 3;

    let mut door_back1 = Teleporter::new(source_world_id);
    door_back1.body_mut().frame.x = (HOUSE_INTERIOR_COLUMNS as f32 / 2.0).ceil() as i32;
    door_back1.body_mut().frame.y = (HOUSE_INTERIOR_ROWS + 2) as i32;

    let mut door_back2 = Teleporter::new(source_world_id);
    door_back2.body_mut().frame.x = door_back1.body().frame.x + 1;
    door_back2.body_mut().frame.y = door_back1.body().frame.y;

    let mut first_floor = World::load_or_create(first_floor_id);

    for row in 0..HOUSE_INTERIOR_ROWS {
        for col in 0..HOUSE_INTERIOR_COLUMNS {
            first_floor.biome_tiles.update_tile(row + 2, col + 1, Biome::DarkWood);
        }
    }
    for row in [0, 1, HOUSE_INTERIOR_ROWS + 2] {
        for col in 0..(HOUSE_INTERIOR_COLUMNS + 1) {
            if row != HOUSE_INTERIOR_ROWS + 2 || (col != door_back1.body().frame.x as usize && col != door_back2.body().frame.x as usize) {
                first_floor.constructions_tiles.update_tile(row, col, Construction::LightWall);
            }
        }
    }
    for row in 0..(HOUSE_INTERIOR_ROWS + 3) {
        first_floor.constructions_tiles.update_tile(row, 0, Construction::LightWall);
    }

    first_floor.add_entity(Box::new(door_back1));
    first_floor.add_entity(Box::new(door_back2));
    first_floor.save();

    vec![
        Box::new(building),
        Box::new(door),
    ]   
}