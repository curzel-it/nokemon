use std::collections::HashMap;

use crate::{constants::{ASSETS_PATH, FPS, SPECIES_PATH}, utils::file_utils::list_files};

use super::{entity_factory::EntityFactory, game::Game, keyboard_events_provider::KeyboardEventsProvider};
use raylib::prelude::*;

pub struct GameEngine {
    pub textures: HashMap<String, Texture2D>
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new()
        }
    }

    pub fn start_rl(&mut self, width: i32, height: i32) -> (Game, RaylibHandle, RaylibThread) {
        let (mut rl, thread) = raylib::init()
            .size(width, height)
            .title("Tower Defense")
            .build();
    
        rl.set_target_fps(FPS);
        
        let all_assets = list_files(ASSETS_PATH, "png");
        let all_species = list_files(SPECIES_PATH, "json");
        self.load_textures(&all_assets, &mut rl, &thread);

        let mut game = Game::new(
            EntityFactory::new(all_species, all_assets),
            Rectangle::new(0.0, 0.0, width as f32, height as f32)
        );
        // self.game_defaults.update(&mut game, 0.0);

        return (game, rl, thread);
    }

    pub fn update(
        &self, 
        game: &mut Game, 
        time_since_last_update: f32,
        keyboard_events: &dyn KeyboardEventsProvider
    ) {
        game.total_elapsed_time += time_since_last_update;
        game.keyboard_state = keyboard_events.keyboard_state();
        /* 
        for id in &game.entity_ids() {
            for behavior in &self.entity_behaviors {
                behavior.update(id, game, time_since_last_update);
            }        
        }
        for behavior in &self.game_behaviors {
            behavior.update(game, time_since_last_update);
        }*/
    } 

    fn load_textures(&mut self, all_assets: &Vec<String>, rl: &mut RaylibHandle, thread: &RaylibThread) {    
        for asset in all_assets {
            let texture = rl.load_texture(&thread, asset).unwrap();
            self.textures.insert(asset.clone(), texture);
        }
    }
}

#[cfg(test)]
mod tests {
    use raylib::math::Rectangle;

    use crate::{constants::{ASSETS_PATH, SPECIES_PATH}, game_engine::{entity_factory::EntityFactory, game::Game}, utils::file_utils::list_files};

    use super::GameEngine;

    impl GameEngine {
        fn start_headless(&mut self, width: i32, height: i32) -> Game {
            let all_assets = list_files(ASSETS_PATH, "png");
            let all_species = list_files(SPECIES_PATH, "json");

            let mut game = Game::new(
                EntityFactory::new(all_species, all_assets),
                Rectangle::new(0.0, 0.0, width as f32, height as f32)
            );
            // self.game_defaults.update(&mut game, 0.0);

            return game;
        }
    }

    #[test]
    fn can_launch_game_headless() {
        let mut engine = GameEngine::new();
        let game = engine.start_headless(600, 900);
        assert_eq!(game.bounds.width, 600.0);
        assert_eq!(game.bounds.height, 900.0);
    }
}