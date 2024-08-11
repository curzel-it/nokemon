use std::cmp::Ordering;

use super::{entity_body::EmbodiedEntity, game::Game, game_state_update::GameStateUpdate};

pub trait Entity: EmbodiedEntity {
    fn update(&mut self, game: &Game, time_since_last_update: f32) -> Vec<GameStateUpdate>;
}

impl PartialEq for dyn Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for dyn Entity {}

impl PartialOrd for dyn Entity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for dyn Entity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.body().frame.y
            .partial_cmp(&other.body().frame.y)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.body().z_index.cmp(&other.body().z_index))
            .then_with(|| self.body().creation_time.partial_cmp(&other.body().creation_time).unwrap_or(Ordering::Equal))
    }
}