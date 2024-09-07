use crate::{constants::SPRITE_SHEET_HOUSEHOLD_OBJECTS, game_engine::entity_body::EmbodiedEntity, utils::rect::Rect};

use super::simple::SimpleEntity;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum HouseholdObject {
    StairsUp,
    StairsDown,
    SeatBrown,
    SeatGreen,
    SeatOrange,
    SeatPink,
    Table,
    Bed,
}

impl HouseholdObject {
    pub fn make_entity(&self) -> SimpleEntity {
        let is_rigid = self.is_rigid();
        let frame = self.texture_source_rect();
        
        let mut entity = SimpleEntity::new(
            is_rigid, 
            frame.w, frame.h, 
            SPRITE_SHEET_HOUSEHOLD_OBJECTS, 
            frame
        );
        entity.body_mut().z_index = self.z_index();
        entity
    }

    fn z_index(&self) -> i32 {
        match self {
            HouseholdObject::StairsUp => 1000,
            HouseholdObject::StairsDown => 1000,
            HouseholdObject::SeatBrown => 100,
            HouseholdObject::SeatGreen => 100,
            HouseholdObject::SeatOrange => 100,
            HouseholdObject::SeatPink => 100,
            HouseholdObject::Table => 200,
            HouseholdObject::Bed => 200,
        }
    }

    fn is_rigid(&self) -> bool {
        match self {
            HouseholdObject::StairsUp => true,
            HouseholdObject::StairsDown => true,
            HouseholdObject::SeatBrown => false,
            HouseholdObject::SeatGreen => false,
            HouseholdObject::SeatOrange => false,
            HouseholdObject::SeatPink => false,
            HouseholdObject::Table => true,
            HouseholdObject::Bed => true,
        }
    }

    fn texture_source_rect(&self) -> Rect {
        let (x, y, w, h) = match self {
            HouseholdObject::StairsUp => (1, 0, 1, 2),
            HouseholdObject::StairsDown => (2, 0, 1, 2),
            HouseholdObject::SeatBrown => (3, 0, 1, 1),
            HouseholdObject::SeatGreen => (3, 1, 1, 1),
            HouseholdObject::SeatOrange => (3, 2, 1, 1),
            HouseholdObject::SeatPink => (3, 3, 1, 1),
            HouseholdObject::Table => (4, 0, 2, 2),
            HouseholdObject::Bed => (0, 2, 1, 2),
        };
        Rect::new(x, y, w, h)
    }
}