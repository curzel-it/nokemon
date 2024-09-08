use serde::{Deserialize, Serialize};

use crate::{constants::{SPRITE_SHEET_ANIMATED_OBJECTS, SPRITE_SHEET_BUILDINGS, SPRITE_SHEET_HOUSEHOLD_OBJECTS, SPRITE_SHEET_HUMANOIDS}, dialogues::models::EntityDialogues, features::animated_sprite::AnimatedSprite, game_engine::{concrete_entity::ConcreteEntity, entity::Entity, state_updates::WorldStateUpdate, world::World}, utils::{directions::Direction, ids::get_next_id, rect::Rect, vector::Vector2d}};

use super::{building::BuildingType, household_objects::HouseholdObject, npc::NpcType, pickable_objects::PickableObject};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    Hero,
    Building(BuildingType),   
    Npc(NpcType), 
    HouseholdObject(HouseholdObject),
    PickableObject(PickableObject),
}

impl EntityType {
    pub fn make_entity(&self) -> ConcreteEntity {
        let frame = self.texture_source_rect(Direction::Unknown, 0.0);

        ConcreteEntity {
            id: get_next_id(),
            frame,  
            species: self.clone(),  
            offset: Vector2d::zero(),
            direction: Direction::Unknown,
            current_speed: 0.0,
            is_rigid: self.is_rigid(),
            z_index: self.z_index(),
            sprite: self.make_sprite(),
            dialogues: EntityDialogues::empty(),
            time_immobilized: 0.0,
            name: "".to_string(),
        }
    }
}

impl EntityType {
    fn make_sprite(&self) -> AnimatedSprite {
        let frame = self.texture_source_rect(Direction::Unknown, 0.0);
        AnimatedSprite::new(
            self.sprite_sheet(), 
            self.number_of_frames(), 
            frame.w, 
            frame.h
        )
    }
}

impl EntityType {
    fn z_index(&self) -> i32 {
        match self {
            EntityType::Hero => 150,
            EntityType::Npc(_) => 0,
            EntityType::Building(_) => 0,
            EntityType::HouseholdObject(item_type) => match item_type {
                HouseholdObject::StairsUp => 1000,
                HouseholdObject::StairsDown => 1000,
                HouseholdObject::SeatBrown => 100,
                HouseholdObject::SeatGreen => 100,
                HouseholdObject::SeatOrange => 100,
                HouseholdObject::SeatPink => 100,
                HouseholdObject::Table => 200,
                HouseholdObject::Bed => 200,
            }
            EntityType::PickableObject(_) => 0,
        }
        
    }
    
    pub fn base_speed(&self) -> f32 {
        match self {
            EntityType::Hero => 3.0,
            EntityType::Npc(_) => 2.0,
            EntityType::Building(_) => 2.0,
            EntityType::HouseholdObject(_) => 0.0,
            EntityType::PickableObject(_) => 0.0,
        }
    }

    fn is_rigid(&self) -> bool {
        match self {
            EntityType::Hero => true,
            EntityType::Building(_) => true,
            EntityType::Npc(_) => true,
            EntityType::HouseholdObject(item) => match item {
                HouseholdObject::StairsUp => true,
                HouseholdObject::StairsDown => true,
                HouseholdObject::SeatBrown => false,
                HouseholdObject::SeatGreen => false,
                HouseholdObject::SeatOrange => false,
                HouseholdObject::SeatPink => false,
                HouseholdObject::Table => true,
                HouseholdObject::Bed => true,
            },
            EntityType::PickableObject(pickable_object) => match pickable_object {
                PickableObject::Key => false,
            },
        }
    }

    fn sprite_sheet(&self) -> u32 {
        match self {
            EntityType::Hero => SPRITE_SHEET_HUMANOIDS,
            EntityType::Building(_) => SPRITE_SHEET_BUILDINGS,
            EntityType::Npc(_) => SPRITE_SHEET_HUMANOIDS,
            EntityType::HouseholdObject(_) => SPRITE_SHEET_HOUSEHOLD_OBJECTS,
            EntityType::PickableObject(_) => SPRITE_SHEET_ANIMATED_OBJECTS
        }
    }

    fn texture_source_rect(&self, direction: Direction, speed: f32) -> Rect {
        let (x, y, w, h) = match self {
            EntityType::Hero => humanoid_texture_source_rect(12, direction, speed),
            EntityType::Building(building_type) => match building_type {
                BuildingType::House(variant) => (0, 5 * variant + 1, 5, 4),
                BuildingType::HouseTwoFloors(variant) => (5, 5 * variant, 5, 5),
            },
            EntityType::Npc(npc_type) => match npc_type {
                NpcType::OldMan => humanoid_texture_source_rect(4, direction, speed),
                NpcType::OldWoman => humanoid_texture_source_rect(8, direction, speed),
            },
            EntityType::HouseholdObject(item) => match item {
                HouseholdObject::StairsUp => (1, 0, 1, 2),
                HouseholdObject::StairsDown => (2, 0, 1, 2),
                HouseholdObject::SeatBrown => (3, 0, 1, 1),
                HouseholdObject::SeatGreen => (3, 1, 1, 1),
                HouseholdObject::SeatOrange => (3, 2, 1, 1),
                HouseholdObject::SeatPink => (3, 3, 1, 1),
                HouseholdObject::Table => (4, 0, 2, 2),
                HouseholdObject::Bed => (0, 2, 1, 2),
            },
            EntityType::PickableObject(pickable_object) => match pickable_object {
                PickableObject::Key => (0, 0, 1, 1),
            },
        };
        Rect::new(x, y, w, h)
    }

    fn number_of_frames(&self) -> i32 {
        match self {
            EntityType::Hero => 4,
            EntityType::Building(_) => 1,
            EntityType::Npc(_) => 4,
            EntityType::HouseholdObject(_) => 1,
            EntityType::PickableObject(pickable_object) => match pickable_object {
                PickableObject::Key => 8,
            },
        }
    }
}

fn humanoid_texture_source_rect(column: i32, direction: Direction, speed: f32) -> (i32, i32, i32, i32) {
    let row = match (direction, speed != 0.0) {
        (Direction::Up, true) => 0,
        (Direction::Up, false) => 1,
        (Direction::Right, true) => 2,
        (Direction::Right, false) => 3,
        (Direction::Down, true) => 4,
        (Direction::Down, false) => 5,
        (Direction::Left, true) => 6,
        (Direction::Left, false) => 7,
        (Direction::Unknown, true) => 5,
        (Direction::Unknown, false) => 5
    };
    (column, row, 1, 2)
}

impl EntityType {
    pub fn inventory_texture_source_rect(&self) -> Rect {
        let (row, col) = self.inventory_texture_offsets();
        Rect::new(col, row, 1, 1)
    }

    fn inventory_texture_offsets(&self) -> (i32, i32) {
        match self {
            EntityType::Hero => (0, 0),
            EntityType::Building(building_type) => match building_type {
                BuildingType::House(variant) => (4, variant * 2 + 1),
                BuildingType::HouseTwoFloors(variant) => (4, variant * 2 + 2),
            },
            EntityType::Npc(npc_type) => match npc_type {
                NpcType::OldMan => (2, 2),
                NpcType::OldWoman => (2, 3),
            },
            EntityType::HouseholdObject(item) => match item {
                HouseholdObject::StairsUp => (3, 2),
                HouseholdObject::StairsDown => (3, 3),
                HouseholdObject::SeatBrown => (3, 4),
                HouseholdObject::SeatGreen => (3, 5),
                HouseholdObject::SeatOrange => (3, 6),
                HouseholdObject::SeatPink => (3, 7),
                HouseholdObject::Table => (3, 8),
                HouseholdObject::Bed => (3, 9),
            },
            EntityType::PickableObject(pickable_object) => match pickable_object {
                PickableObject::Key => (5, 1),
            },
        }
    }
}