use std::fmt::{self, Debug};

use raylib::math::{Rectangle, Vector2};

use crate::constants::ANIMATIONS_FPS;
use crate::game::rendered_item::RenderedItem;
use crate::sprites::sprite::Sprite;
use crate::sprites::sprite_set::SpriteSet;

use super::entity_capability::{EntityCapability, EntityStateUpdate, GameStateSnapshot};

pub struct Entity {
    pub id: u32,
    pub frame: Rectangle,
    pub direction: Vector2,
    pub speed: f32,
    pub species: String,
    pub sprite_set: SpriteSet,
    pub current_sprite: Sprite,
    pub capabilities: Vec<Box<dyn EntityCapability>>,
    pub is_enemy: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct EntityStateSnapshot {
    pub id: u32,
    pub frame: Rectangle,
}

impl Entity {
    pub fn update(&mut self, game_state: &GameStateSnapshot, time_since_last_update: f32) {
        let mut updates: Vec<EntityStateUpdate> = vec![];
        for capabilty in &self.capabilities {
            let update = capabilty.update(self, game_state, time_since_last_update);
            updates.push(update);
        }
        for update in updates {
            self.apply(update);
        }

        self.current_sprite.update(time_since_last_update);
    }

    pub fn state_snapshot(&self) -> EntityStateSnapshot {
        EntityStateSnapshot {
            id: self.id, 
            frame: self.frame
        }
    }

    fn apply(&mut self, update: EntityStateUpdate) {
        if let Some(value) = update.frame {
            self.frame = value;
        }
        if let Some(value) = update.direction {
            self.direction = value;
        }
        if let Some(value) = update.sprite_name {
            self.change_sprite(value.as_str());
        }
    }

    pub fn change_sprite(&mut self, animation_name: &str) -> u32 {
        if self.current_sprite.animation_name != animation_name {
            self.current_sprite = self.sprite_set.sprite(&animation_name, ANIMATIONS_FPS);
        }
        ((self.current_sprite.number_of_frames() as f32) / ANIMATIONS_FPS) as u32
    }

    pub fn render(&self) -> RenderedItem {
        return RenderedItem::new(
            self.id, 
            self.current_sprite_frame(), 
            self.frame, 
            self.direction.x < 0.0, 
            0.0
        );
    }

    fn current_sprite_frame(&self) -> String {
        self.current_sprite.current_frame().to_string()
    }
}

impl Debug for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entity")
            .field("id", &self.id)
            .field("sprite", &self.current_sprite_frame())
            .field("speed", &self.speed)
            .field("dx", &self.direction.x)
            .field("dy", &self.direction.y)        
            .field("x", &self.frame.x)
            .field("y", &self.frame.y)
            .field("width", &self.frame.width)
            .field("height", &self.frame.height)
            .field("is_enemy", &self.is_enemy)
            .finish()
    }
}