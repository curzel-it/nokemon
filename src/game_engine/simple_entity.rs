use crate::{features::{autoremove::remove_automatically, linear_movement::{self, move_linearly}}, impl_embodied_entity};

use super::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, game::Game, game_state_update::GameStateUpdate};

#[derive(Debug)]
pub struct SimpleEntity {
    body: EntityBody,
    time_to_next_shot: f32
}

impl SimpleEntity {
    pub fn new(body: EntityBody) -> Self {
        let time_to_next_shot = body.species.time_between_shots;

        return Self { 
            body,
            time_to_next_shot
        };
    }
}

impl_embodied_entity!(SimpleEntity);

impl Entity for SimpleEntity {
    fn update(&mut self, game: &Game, time_since_last_update: f32) -> Vec<GameStateUpdate> {
        let mut game_updates: Vec<GameStateUpdate> = vec![];
        game_updates.append(&mut move_linearly(self, time_since_last_update));
        game_updates.append(&mut remove_automatically(self, game));
        game_updates
    }
}