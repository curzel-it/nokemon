use raylib::math::Vector2;

use crate::game_engine::{behaviors::GameBehavior, game::Game};

pub struct CreepSpawner;

impl GameBehavior for CreepSpawner {
    fn update(&self, game: &mut Game, _: f32) {   
        let ms = (game.total_elapsed_time * 1000.0).floor() as i32;
        
        if ms % 2000 < 10 {
            game.add_entity_by_species("white");
        }
    }
}

impl CreepSpawner {
    pub fn new() -> Self {
        Self {}
    }
}