use crate::{constants::{HOUSE_INTERIOR_COLUMNS, HOUSE_INTERIOR_ROWS}, game_engine::{concrete_entity::{BuildingType, ConcreteEntity, EntityType, HouseholdObject}, world::World}, maps::{biome_tiles::Biome, constructions_tiles::Construction}, utils::ids::get_next_id};

pub fn new_house_single_floor(variant: i32, source_world_id: u32, x: i32, y: i32) -> Vec<ConcreteEntity> {
    let mut building = EntityType::Building(BuildingType::House(variant)).make_entity();
    building.frame.x = x;
    building.frame.y = y;

    let first_floor_id = get_next_id();
    let mut door = EntityType::Teleporter.make_entity();
    door.destination = first_floor_id;
    door.frame.x = x + (building.frame.w as f32 / 2.0).ceil() as i32;
    door.frame.y = y + 3;

    let mut door_back1 = EntityType::Teleporter.make_entity();
    door_back1.destination = source_world_id;
    door_back1.frame.x = (HOUSE_INTERIOR_COLUMNS as f32 / 2.0).ceil() as i32;
    door_back1.frame.y = (HOUSE_INTERIOR_ROWS + 2) as i32;

    let mut door_back2 = EntityType::Teleporter.make_entity();
    door_back2.destination = source_world_id;
    door_back2.frame.x = door_back1.frame.x + 1;
    door_back2.frame.y = door_back1.frame.y;

    let mut first_floor = World::load_or_create(first_floor_id);

    for row in 0..HOUSE_INTERIOR_ROWS {
        for col in 0..HOUSE_INTERIOR_COLUMNS {
            first_floor.biome_tiles.update_tile(row + 2, col + 1, Biome::DarkWood);
        }
    }
    for row in [0, 1, HOUSE_INTERIOR_ROWS + 2] {
        for col in 0..(HOUSE_INTERIOR_COLUMNS + 1) {
            if row != HOUSE_INTERIOR_ROWS + 2 || (col != door_back1.frame.x as usize && col != door_back2.frame.x as usize) {
                first_floor.constructions_tiles.update_tile(row, col, Construction::LightWall);
            }
        }
    }
    for row in 0..(HOUSE_INTERIOR_ROWS + 3) {
        first_floor.constructions_tiles.update_tile(row, 0, Construction::LightWall);
    }

    let mut table = EntityType::HouseholdObject(HouseholdObject::Table).make_entity();
    table.frame.x = 1;
    table.frame.y = 4;

    let mut seat1 = EntityType::HouseholdObject(HouseholdObject::SeatGreen).make_entity();
    seat1.frame.x = 1;
    seat1.frame.y = 4;

    let mut seat2 = EntityType::HouseholdObject(HouseholdObject::SeatGreen).make_entity();
    seat2.frame.x = 2;
    seat2.frame.y = 4;

    let mut seat3 = EntityType::HouseholdObject(HouseholdObject::SeatGreen).make_entity();
    seat3.frame.x = 1;
    seat3.frame.y = 6;

    let mut seat4 = EntityType::HouseholdObject(HouseholdObject::SeatGreen).make_entity();
    seat4.frame.x = 2;
    seat4.frame.y = 6;

    first_floor.add_entity(door_back1);
    first_floor.add_entity(door_back2);
    first_floor.add_entity(table);
    first_floor.add_entity(seat1);
    first_floor.add_entity(seat2);
    first_floor.add_entity(seat3);
    first_floor.add_entity(seat4);
    first_floor.save();

    vec![building, door]   
}