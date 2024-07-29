use raylib::math::{Rectangle, Vector2};
use std::fmt::Debug;

use crate::{entities::entity::Entity, game::game_capability::GameStateUpdate};

use super::{entity::EntityStateSnapshot, factory::EntityDescriptor};

pub trait EntityCapability: Debug {
    fn update(&mut self, entity: &EntityStateSnapshot, game_state: &GameStateSnapshot, time_since_last_update: f32) -> EntityStateUpdate;
}

pub struct GameStateSnapshot {
    pub enemies: Vec<EntityStateSnapshot>
}

pub struct EntityStateUpdate {
    pub frame: Option<Rectangle>,
    pub direction: Option<Vector2>,
    pub sprite_name: Option<String>,
    pub game_update: GameStateUpdate
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
            game_update: GameStateUpdate::nothing()
        }
    }

    pub fn frame(frame: Rectangle) -> Self {
        Self {
            frame: Some(frame),
            direction: None,
            sprite_name: None,
            game_update: GameStateUpdate::nothing()
        }
    }

    pub fn new_entity(entity: EntityDescriptor) -> Self {
        Self {
            frame: None,
            direction: None,
            sprite_name: None,
            game_update: GameStateUpdate::new_entity(entity)
        }
    }
}

pub struct UnknownCapability {
    name: String
}

impl UnknownCapability {
    pub fn new(name: &String) -> Self {
        Self {
            name: name.clone()
        }
    }
}

impl EntityCapability for UnknownCapability {
    fn update(&mut self, entity: &EntityStateSnapshot, _: &GameStateSnapshot, _: f32) -> EntityStateUpdate {
        println!("Detected unknwon capability! {:#?}", entity);
        return EntityStateUpdate::nothing();
    }
}

impl Debug for UnknownCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnknownCapability")
            .field("name", &self.name)
            .finish()
    }
}