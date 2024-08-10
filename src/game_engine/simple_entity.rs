use crate::{features::{animated_sprite::{update_sprite}, autoremove::remove_automatically, linear_movement::{move_linearly}}, impl_animated_entity, impl_embodied_entity};

use super::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, game::Game, game_state_update::GameStateUpdate};

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
impl_animated_entity!(SimpleEntity);

impl Entity for SimpleEntity {
    fn update(&mut self, game: &Game, time_since_last_update: f32) -> Vec<GameStateUpdate> {
        let mut game_updates: Vec<GameStateUpdate> = vec![];
        move_linearly(self, time_since_last_update);
        update_sprite(self, time_since_last_update);
        game_updates.append(&mut remove_automatically(self, game));
        game_updates
    }
}