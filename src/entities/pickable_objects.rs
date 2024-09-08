use crate::{constants::SPRITE_SHEET_ANIMATED_OBJECTS, features::animated_sprite::AnimatedSprite, game_engine::{entity::{Entity, EntityConvertible}, entity_body::EmbodiedEntity}, utils::rect::Rect};

use super::animated_entity::AnimatedEntity;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum PickableObject {
    Key
}

impl EntityConvertible for PickableObject {
    fn make_entity(&self) -> Box<dyn Entity> {
        let frame = self.texture_source_rect();

        let sprite = AnimatedSprite::new(
            SPRITE_SHEET_ANIMATED_OBJECTS, 
            self.number_of_frames(), 
            frame.w, 
            frame.h
        );
        
        let mut entity = AnimatedEntity::new(
            self.is_rigid(), 
            frame.w, 
            frame.h, 
            sprite
        );
        entity.body_mut().z_index = 200;
        Box::new(entity)
    }
}

impl PickableObject {
    fn is_rigid(&self) -> bool {
        match self {
            PickableObject::Key => false,
            _ => false
        }
    }

    fn texture_source_rect(&self) -> Rect {
        let (x, y, w, h) = match self {
            PickableObject::Key => (0, 0, 1, 1),
        };
        Rect::new(x, y, w, h)
    }

    fn number_of_frames(&self) -> i32 {
        match self {
            PickableObject::Key => 8,
            _ => 1
        }
    }
}