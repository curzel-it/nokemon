use crate::{constants::{HOUSE_INTERIOR_COLUMNS, HOUSE_INTERIOR_ROWS}, game_engine::{entity::{BuildingType, Entity, Species, HouseholdObject}, world::World}, maps::{biome_tiles::Biome, constructions_tiles::Construction}, utils::ids::get_next_id};

pub fn new_house_two_floors(variant: i32, source_world_id: u32, x: i32, y: i32) -> Vec<Entity> {
    let mut building = Species::Building(BuildingType::HouseTwoFloors(variant)).make_entity();
    building.frame.x = x;
    building.frame.y = y;

    let first_floor_id = get_next_id();
    let second_floor_id = get_next_id();

    let mut door = Species::Teleporter.make_entity();
    door.destination = first_floor_id;
    door.frame.x = x + (building.frame.w as f32 / 2.0).ceil() as i32;
    door.frame.y = y + 4;

    let mut door_back1 = Species::Teleporter.make_entity();
    door_back1.destination = source_world_id;
    door_back1.frame.x = (HOUSE_INTERIOR_COLUMNS as f32 / 2.0).ceil() as i32;
    door_back1.frame.y = (HOUSE_INTERIOR_ROWS + 2) as i32;

    let mut door_back2 = Species::Teleporter.make_entity();
    door_back2.destination = source_world_id;
    door_back2.frame = door_back1.frame.offset_x(1);

    let mut stairs_up = Species::HouseholdObject(HouseholdObject::StairsUp).make_entity();
    stairs_up.frame.x = HOUSE_INTERIOR_COLUMNS as i32 - 2;
    stairs_up.frame.y = 0;

    let mut stairs_up_door = Species::Teleporter.make_entity();
    stairs_up_door.destination = second_floor_id;
    stairs_up_door.frame.x = stairs_up.frame.x;
    stairs_up_door.frame.y = stairs_up.frame.y + 1;

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

    let mut table = Species::HouseholdObject(HouseholdObject::Table).make_entity();
    table.frame.x = 1;
    table.frame.y = 4;

    let mut seat1 = Species::HouseholdObject(HouseholdObject::SeatGreen).make_entity();
    seat1.frame.x = 1;
    seat1.frame.y = 4;

    let mut seat2 = Species::HouseholdObject(HouseholdObject::SeatGreen).make_entity();
    seat2.frame.x = 2;
    seat2.frame.y = 4;

    let mut seat3 = Species::HouseholdObject(HouseholdObject::SeatGreen).make_entity();
    seat3.frame.x = 1;
    seat3.frame.y = 6;

    let mut seat4 = Species::HouseholdObject(HouseholdObject::SeatGreen).make_entity();
    seat4.frame.x = 2;
    seat4.frame.y = 6;

    first_floor.add_entity(door_back1);
    first_floor.add_entity(door_back2);
    first_floor.add_entity(stairs_up);
    first_floor.add_entity(stairs_up_door);
    first_floor.add_entity(table);
    first_floor.add_entity(seat1);
    first_floor.add_entity(seat2);
    first_floor.add_entity(seat3);
    first_floor.add_entity(seat4);
    first_floor.save();    

    let mut stairs_down = Species::HouseholdObject(HouseholdObject::StairsDown).make_entity();
    stairs_down.frame.x = HOUSE_INTERIOR_COLUMNS as i32 - 2;
    stairs_down.frame.y = 1;

    let mut stairs_down_door = Species::Teleporter.make_entity();
    stairs_down_door.destination = first_floor_id;
    stairs_down_door.frame.x = stairs_down.frame.x;
    stairs_down_door.frame.y = stairs_down.frame.y + 1;

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

    second_floor.add_entity(stairs_down);
    second_floor.add_entity(stairs_down_door);
    second_floor.save();    

    vec![building, door]   
}