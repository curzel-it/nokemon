use crate::{constants::SPRITE_SHEET_HOUSEHOLD_OBJECTS, utils::rect::Rect};

use super::simple::SimpleEntity;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum HouseholdObject {
    Stairs,
}

impl HouseholdObject {
    pub fn make_entity(&self) -> SimpleEntity {
        let is_rigid = self.is_rigid();
        let (width, height) = self.entity_size();
        
        SimpleEntity::new(
            is_rigid, 
            width, height, 
            SPRITE_SHEET_HOUSEHOLD_OBJECTS, 
            self.texture_source_rect()
        )
    }

    fn is_rigid(&self) -> bool {
        match self {
            HouseholdObject::Stairs => true,
        }
    }

    fn entity_size(&self) -> (u32, u32) {
        match self {
            HouseholdObject::Stairs => (1, 1),
        }
    }

    fn texture_source_rect(&self) -> Rect {
        let (x, y, w, h) = match self {
            HouseholdObject::Stairs => (0, 0, 1, 1),
        };
        Rect::new(x, y, w, h)
    }
}