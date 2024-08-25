use std::any::Any;

use uuid::Uuid;

use crate::{constants::{INFINITE_LIFESPAN, NO_PARENT}, features::animated_sprite::AnimatedSprite, impl_embodied_entity, utils::{geometry_utils::Insets, rect::Rect, vector::Vector2d}};

use super::{entity::Entity, entity_body::EntityBody, world::World, state_updates::WorldStateUpdate};

#[derive(Debug)]
pub struct StaticObstacle {
    body: EntityBody,
    sprite: AnimatedSprite,
}

impl StaticObstacle {
    pub fn new(sprite_sheet: u32, frame: Rect) -> Self {
        Self { 
            body: EntityBody {
                id: Uuid::new_v4(),
                parent_id: NO_PARENT,
                frame,
                collision_insets: Insets::zero(),
                direction: Vector2d::zero(),
                current_speed: 0.0,
                base_speed: 0.0,
                hp: 100.0,
                dp: 0.0,
                creation_time: 0.0,
                requires_collision_detection: false,
                is_rigid: true,
                z_index: 0,
                is_ally: false,
                lifespan: INFINITE_LIFESPAN,
            },
            sprite: AnimatedSprite::new(
                sprite_sheet, 
                1, 
                frame.w as u32, 
                frame.h as u32
            )
        }
    }
}

impl_embodied_entity!(StaticObstacle);

impl Entity for StaticObstacle {
    fn update(&mut self, _: &World, _: f32) -> Vec<WorldStateUpdate> {
        vec![]
    }

    fn texture_source_rect(&self) -> Rect {
        self.sprite.texture_source_rect()
    }

    fn sprite_sheet(&self) -> u32 {
        self.sprite.sheet_id
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}