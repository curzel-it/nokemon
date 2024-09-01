use crate::{constants::SPRITE_SHEET_HOUSEHOLD_OBJECTS, utils::rect::Rect};

use super::simple::SimpleEntity;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum HouseholdObject {
    Stairs,
    StairsUp,
    StairsDown,
}

impl HouseholdObject {
    pub fn make_entity(&self) -> SimpleEntity {
        let is_rigid = self.is_rigid();
        let frame = self.texture_source_rect();
        
        SimpleEntity::new(
            is_rigid, 
            frame.w, frame.h, 
            SPRITE_SHEET_HOUSEHOLD_OBJECTS, 
            frame
        )
    }

    fn is_rigid(&self) -> bool {
        match self {
            HouseholdObject::Stairs => true,
            HouseholdObject::StairsUp => true,
            HouseholdObject::StairsDown => true,
        }
    }

    fn texture_source_rect(&self) -> Rect {
        let (x, y, w, h) = match self {
            HouseholdObject::Stairs => (0, 0, 1, 1),
            HouseholdObject::StairsUp => (1, 0, 1, 2),
            HouseholdObject::StairsDown => (2, 0, 1, 2),
        };
        Rect::new(x, y, w, h)
    }
}