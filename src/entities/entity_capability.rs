use raylib::math::{Rectangle, Vector2};

use crate::entities::entity::Entity;

use super::entity::EntityStateSnapshot;

pub trait EntityCapability {
    fn update(&self, entity: &Entity, game_state: &GameStateSnapshot, time_since_last_update: f32) -> EntityStateUpdate;
}

pub struct GameStateSnapshot {
    pub enemies: Vec<EntityStateSnapshot>
}

pub struct EntityStateUpdate {
    pub frame: Option<Rectangle>,
    pub direction: Option<Vector2>,
    pub sprite_name: Option<String>
}

impl GameStateSnapshot {
    pub fn nothing() -> Self {
        Self {
            enemies: vec![]
        }
    }
}

impl EntityStateUpdate {
    pub fn nothing() -> Self {
        Self {
            frame: None,
            direction: None,
            sprite_name: None,
        }
    }

    pub fn frame(frame: Rectangle) -> Self {
        Self {
            frame: Some(frame),
            direction: None,
            sprite_name: None,
        }
    }
}