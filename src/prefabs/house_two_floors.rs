use crate::{constants::{HOUSE_INTERIOR_COLUMNS, HOUSE_INTERIOR_ROWS}, entities::{building::{Building, BuildingType}, household_objects::HouseholdObject, teleporter::Teleporter}, game_engine::{entity::Entity, entity_body::EmbodiedEntity, world::World}, maps::{biome_tiles::Biome, constructions_tiles::Construction}, utils::ids::get_next_id};

pub fn new_house_two_floors(variant: i32, source_world_id: u32, x: i32, y: i32) -> Vec<Box<dyn Entity>> {
    let mut building = Building::new(BuildingType::HouseTwoFloors(variant));
    building.body_mut().frame.x = x;
    building.body_mut().frame.y = y;

    let first_floor_id = get_next_id();
    let second_floor_id = get_next_id();

    let mut door = Teleporter::new(first_floor_id);
    door.body_mut().frame.x = x + (building.body().frame.w as f32 / 2.0).ceil() as i32;
    door.body_mut().frame.y = y + 4;

    let mut door_back1 = Teleporter::new(source_world_id);
    door_back1.body_mut().frame.x = (HOUSE_INTERIOR_COLUMNS as f32 / 2.0).ceil() as i32;
    door_back1.body_mut().frame.y = (HOUSE_INTERIOR_ROWS + 2) as i32;

    let mut door_back2 = Teleporter::new(source_world_id);
    door_back2.body_mut().frame = door_back1.body().frame.offset_x(1);

    let mut stairs_up = HouseholdObject::StairsUp.make_entity();
    stairs_up.body_mut().frame.x = HOUSE_INTERIOR_COLUMNS as i32 - 2;
    stairs_up.body_mut().frame.y = 0;

    let mut stairs_up_door = Teleporter::new(second_floor_id);
    stairs_up_door.body_mut().frame.x = stairs_up.body().frame.x;
    stairs_up_door.body_mut().frame.y = stairs_up.body().frame.y + 1;

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
    first_floor.add_entity(Box::new(stairs_up));
    first_floor.add_entity(Box::new(stairs_up_door));
    first_floor.save();    

    let mut stairs_down = HouseholdObject::StairsDown.make_entity();
    stairs_down.body_mut().frame.x = HOUSE_INTERIOR_COLUMNS as i32 - 2;
    stairs_down.body_mut().frame.y = 1;

    let mut stairs_down_door = Teleporter::new(first_floor_id);
    stairs_down_door.body_mut().frame.x = stairs_down.body().frame.x;
    stairs_down_door.body_mut().frame.y = stairs_down.body().frame.y + 1;

    let mut second_floor = World::load_or_create(second_floor_id);

    for row in 0..HOUSE_INTERIOR_ROWS {
        for col in 0..HOUSE_INTERIOR_COLUMNS {
            second_floor.biome_tiles.update_tile(row + 2, col + 1, Biome::DarkWood);
        }
    }
    for row in [0, 1, HOUSE_INTERIOR_ROWS + 2] {
        for col in 0..(HOUSE_INTERIOR_COLUMNS + 1) {
            second_floor.constructions_tiles.update_tile(row, col, Construction::LightWall);
        }
    }
    for row in 0..(HOUSE_INTERIOR_ROWS + 3) {
        second_floor.constructions_tiles.update_tile(row, 0, Construction::LightWall);
    }

    second_floor.add_entity(Box::new(stairs_down));
    second_floor.add_entity(Box::new(stairs_down_door));
    second_floor.save();    

    vec![
        Box::new(building),
        Box::new(door),
    ]   
}