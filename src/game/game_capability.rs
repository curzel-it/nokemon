use crate::entities::entity::Entity;

use super::game::Game;

pub trait GameCapability {
    fn update(&self, game: &Game, time_since_last_update: f32) -> GameStateUpdate;
}

pub struct GameStateUpdate {
    pub new_entities: Vec<Entity>,
    pub entities_to_remove: Vec<u32>
}

impl GameStateUpdate {
    pub fn nothing() -> Self {
        Self {
            new_entities: vec![],
            entities_to_remove: vec![],
        }
    }
}