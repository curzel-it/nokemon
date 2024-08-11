
use super::{entity_body::EmbodiedEntity, game::Game, game_state_update::GameStateUpdate};

pub trait Entity: EmbodiedEntity {
    fn update(&mut self, game: &Game, time_since_last_update: f32) -> Vec<GameStateUpdate>;
}

impl PartialEq for dyn Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}