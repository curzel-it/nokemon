use crate::{entity_behaviors::{autoremove::autoremove, linear_movement::linear_movement, shooter::{shoot_stuff, Shooter}}, impl_embodied_entity, species::species_model::SpeciesCapability};

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

impl Shooter for SimpleEntity {
    fn time_to_next_shot(&self) -> f32 {
        return self.time_to_next_shot;
    }

    fn inc_time_to_next_shot(&mut self, delta: f32) {
        self.time_to_next_shot += delta;
    }

    fn reset_time_to_next_shot(&mut self) {
        self.time_to_next_shot = self.species().time_between_shots;
    }
}


impl Entity for SimpleEntity {
    fn update(&mut self, game: &Game, time_since_last_update: f32) -> Vec<GameStateUpdate> {
        let capabilities = self.species().capabilities.clone();
        let mut game_updates: Vec<GameStateUpdate> = vec![];

        if capabilities.contains(&SpeciesCapability::LinearMovement) {
            game_updates.append(&mut linear_movement(self, game, time_since_last_update));
        }
        if capabilities.contains(&SpeciesCapability::Shooter) {
            game_updates.append(&mut shoot_stuff(self, game, time_since_last_update));
        }

        game_updates.append(&mut autoremove(self, game));
        game_updates
    }
}