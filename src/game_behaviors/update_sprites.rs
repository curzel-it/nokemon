use crate::game_engine::{game::Game, game_behavior::GameBehavior};

#[derive(Debug)]
pub struct UpdateSprites;

impl UpdateSprites {
    pub fn new() -> Self {
        Self {}
    }
}

impl GameBehavior for UpdateSprites {
    fn update(&self, entity_id: &u32, game: &mut Game, time_since_last_update: f32) {
        let entity = game.entities.get_mut(entity_id).unwrap();
        entity.current_sprite.update(time_since_last_update);
    }
}