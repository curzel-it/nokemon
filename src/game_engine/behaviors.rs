use super::{game::Game, game_state_update::GameStateUpdate};

pub trait GameBehavior {
    fn update(&self, game: &Game, time_since_last_update: f32) -> Vec<GameStateUpdate>;
}