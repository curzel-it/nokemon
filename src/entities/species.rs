use serde::{Deserialize, Serialize};

use crate::{constants::{HERO_ENTITY_ID, WORLD_ID_NONE}, dialogues::models::EntityDialogues, game_engine::entity::Entity, lang::localizable::LocalizableText, utils::{directions::Direction, ids::get_next_id, vector::Vector2d}};

use super::{buildings::BuildingType, household_objects::HouseholdObject, npcs::NpcType, pickable_objects::PickableObject};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Species {
    Hero,
    Building(BuildingType),   
    Npc(NpcType), 
    HouseholdObject(HouseholdObject),
    PickableObject(PickableObject),
    Teleporter,
}

impl Species {
    pub fn make_entity(&self) -> Entity {
        Entity {
            id: self.next_entity_id(),
            frame: self.texture_source_rect(false),  
            species: self.clone(),  
            offset: Vector2d::zero(),
            direction: Direction::Unknown,
            current_speed: 0.0,
            is_rigid: self.is_rigid(),
            z_index: self.z_index(),
            sprite: self.make_sprite(false),
            dialogues: EntityDialogues::empty(),
            time_immobilized: 0.0,
            name: self.default_name(),
            destination: WORLD_ID_NONE,
        }
    }

    fn next_entity_id(&self) -> u32 {
        match self {
            Species::Hero => HERO_ENTITY_ID,
            _ => get_next_id()
        }
    }
}

impl Species {
    pub fn inventory_texture_offsets(&self) -> (i32, i32) {
        match self {
            Species::Hero => (0, 0),
            Species::Building(building_type) => match building_type {
                BuildingType::House(variant) => (4, variant * 2 + 1),
                BuildingType::HouseTwoFloors(variant) => (4, variant * 2 + 2),
            },
            Species::Npc(npc_type) => match npc_type {
                NpcType::OldMan => (2, 2),
                NpcType::OldWoman => (2, 3),
            },
            Species::HouseholdObject(item) => match item {
                HouseholdObject::StairsUp => (3, 1),
                HouseholdObject::StairsDown => (3, 2),
                HouseholdObject::SeatBrown => (3, 3),
                HouseholdObject::SeatGreen => (3, 4),
                HouseholdObject::SeatOrange => (3, 5),
                HouseholdObject::SeatPink => (3, 6),
                HouseholdObject::Table => (3, 7),
                HouseholdObject::Bed => (3, 8),
            },
            Species::PickableObject(pickable_object) => match pickable_object {
                PickableObject::Key => (5, 1),
            },
            Species::Teleporter => (0, 0),
        }
    }
}

impl Species {    
    fn default_name(&self) -> String {
        match self {
            Species::Hero => "".to_string(),
            Species::Npc(item) => match item {
                NpcType::OldMan => "npc.name.old_man".localized(),
                NpcType::OldWoman => "npc.name.old_woman".localized(),
            }
            Species::Building(item) => match item {
                BuildingType::House(_) => "building.name.house".localized(),
                BuildingType::HouseTwoFloors(_) => "building.name.house_two_floors".localized()
            }
            Species::HouseholdObject(item) => match item {
                HouseholdObject::StairsUp => "objects.name.stairs_up".localized(),
                HouseholdObject::StairsDown => "objects.name.stairs_down".localized(),
                HouseholdObject::SeatBrown => "objects.name.seat_brown".localized(),
                HouseholdObject::SeatGreen => "objects.name.seat_green".localized(),
                HouseholdObject::SeatOrange => "objects.name.seat_orange".localized(),
                HouseholdObject::SeatPink => "objects.name.seat_pink".localized(),
                HouseholdObject::Table => "objects.name.table".localized(),
                HouseholdObject::Bed => "objects.name.bed".localized(),
            }
            Species::PickableObject(item) => match item {
                PickableObject::Key => "objects.name.key".localized(),
            },
            Species::Teleporter => "teleporter.name".localized(),
        }        
    }

    fn z_index(&self) -> i32 {
        match self {
            Species::Hero => 150,
            Species::Npc(_) => 0,
            Species::Building(_) => 0,
            Species::HouseholdObject(item_type) => match item_type {
                HouseholdObject::StairsUp => 1000,
                HouseholdObject::StairsDown => 1000,
                HouseholdObject::SeatBrown => 100,
                HouseholdObject::SeatGreen => 100,
                HouseholdObject::SeatOrange => 100,
                HouseholdObject::SeatPink => 100,
                HouseholdObject::Table => 200,
                HouseholdObject::Bed => 200,
            }
            Species::PickableObject(_) => 200,
            Species::Teleporter => 200,
        }        
    }
    
    pub fn base_speed(&self) -> f32 {
        match self {
            Species::Hero => 3.0,
            Species::Npc(_) => 2.0,
            Species::Building(_) => 2.0,
            Species::HouseholdObject(_) => 0.0,
            Species::PickableObject(_) => 0.0,
            Species::Teleporter => 0.0,
        }
    }

    fn is_rigid(&self) -> bool {
        match self {
            Species::Hero => true,
            Species::Building(_) => true,
            Species::Npc(_) => true,
            Species::HouseholdObject(item) => match item {
                HouseholdObject::StairsUp => true,
                HouseholdObject::StairsDown => true,
                HouseholdObject::SeatBrown => false,
                HouseholdObject::SeatGreen => false,
                HouseholdObject::SeatOrange => false,
                HouseholdObject::SeatPink => false,
                HouseholdObject::Table => true,
                HouseholdObject::Bed => true,
            },
            Species::PickableObject(pickable_object) => match pickable_object {
                PickableObject::Key => false,
            },
            Species::Teleporter => false,
        }
    }
}