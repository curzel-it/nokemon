use crate::game_engine::{behaviors::GameBehavior, game::Game};

pub struct CreepSpawner;

impl GameBehavior for CreepSpawner {
    fn update(&self, game: &mut Game, _: f32) {   
        if game.is_every_n_seconds(2) {
            game.add_entity_by_species("white");
        }
    }
}

impl CreepSpawner {
    pub fn new() -> Self {
        Self {}
    }
}