use raylib::math::{Rectangle, Vector2};
use std::fmt::Debug;

use crate::{constants::RECT_ORIGIN_FULL_HD, entities::entity::Entity, game::game_capability::GameStateUpdate};

use super::{entity::EntityStateSnapshot, factory::EntityDescriptor};

pub trait EntityCapability: Debug {
    fn update(&mut self, entity: &EntityStateSnapshot, game_state: &GameStateSnapshot, time_since_last_update: f32) -> EntityStateUpdate;
}

pub struct GameStateSnapshot {
    pub bounds: Rectangle,
    pub enemies: Vec<EntityStateSnapshot>
}

pub struct EntityStateUpdate {
    pub frame: Option<Rectangle>,
    pub direction: Option<Vector2>,
    pub sprite_name: Option<String>,
    pub game_update: GameStateUpdate
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

    pub fn remove_entity(id: u32) -> Self {
        Self {
            frame: None,
            direction: None,
            sprite_name: None,
            game_update: GameStateUpdate::remove_entity(id)
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
        println!("Detected unknown capability! {:#?}", entity);
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

#[cfg(test)]
mod tests {
    use crate::constants::RECT_ORIGIN_FULL_HD;

    use super::GameStateSnapshot;

    impl GameStateSnapshot {
        pub fn nothing() -> Self {
            Self {
                bounds: RECT_ORIGIN_FULL_HD,
                enemies: vec![]
            }
        }
    }
}