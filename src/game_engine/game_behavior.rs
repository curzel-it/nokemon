use super::game::Game;

pub trait GameBehavior {
    fn update(&self, entity_id: &u32, game: &mut Game, time_since_last_update: f32);
}