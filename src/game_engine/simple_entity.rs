use crate::{entity_behaviors::linear_movement::linear_movement, impl_embodied_entity};

use super::{entity::Entity, entity_body::EntityBody, game::Game};

#[derive(Debug)]
pub struct SimpleEntity {
    body: EntityBody
}

impl SimpleEntity {
    pub fn new(body: EntityBody) -> Self {
        Self { 
            body
        }
    }
}

impl_embodied_entity!(SimpleEntity);

impl Entity for SimpleEntity {
    fn update(&mut self, game: &Game, time_since_last_update: f32) {
        linear_movement(self, game, time_since_last_update);
    }
}