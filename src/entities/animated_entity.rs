use std::any::Any;

use serde::{Deserialize, Serialize};
use crate::{dialogues::models::EntityDialogues, features::animated_sprite::AnimatedSprite, game_engine::{entity::Entity, entity_body::EntityBody, state_updates::WorldStateUpdate, world::World}, impl_embodied_entity, utils::{directions::Direction, ids::get_next_id, rect::Rect, vector::Vector2d}};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimatedEntity {
    body: EntityBody,
    sprite: AnimatedSprite,
}

impl AnimatedEntity {
    pub fn new(is_rigid: bool, width: i32, height: i32, sprite: AnimatedSprite) -> Self {
        Self { 
            body: EntityBody {
                id: get_next_id(),
                frame: Rect::new(0, 0, width, height),
                offset: Vector2d::zero(),
                direction: Direction::Unknown,
                current_speed: 0.0,
                base_speed: 0.0,
                is_rigid,
                z_index: 0,
                dialogues: EntityDialogues::empty(),
            },
            sprite
        }
    }
}

impl_embodied_entity!(AnimatedEntity);

impl Entity for AnimatedEntity {
    fn update(&mut self, _: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        self.sprite.update(time_since_last_update);
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
