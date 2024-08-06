use crate::{entity_behaviors::{autoremove::autoremove, linear_movement::linear_movement}, impl_embodied_entity};

use super::{entity::Entity, entity_body::EntityBody, game::Game, game_state_update::GameStateUpdate};

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
    fn update(&mut self, game: &Game, time_since_last_update: f32) -> Vec<GameStateUpdate> {
        let mut game_updates: Vec<GameStateUpdate> = vec![];

        game_updates.append(&mut linear_movement(self, game, time_since_last_update));
        game_updates.append(&mut autoremove(self, game));
        game_updates
    }
}