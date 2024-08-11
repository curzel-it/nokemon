use std::collections::HashMap;

use crate::{constants::{ASSETS_PATH, FPS}, utils::file_utils::list_files};

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

    pub fn start_rl(&mut self) -> (Game, RaylibHandle, RaylibThread) {        
        let all_assets = list_files(ASSETS_PATH, "png");

        let mut game = Game::new(EntityFactory::new(all_assets.clone()));
        game.setup();

        let (mut rl, thread) = raylib::init()
            .size(game.camera_viewport.width as i32, game.camera_viewport.height as i32)
            .title("Tower Defense")
            .build();
    
        rl.set_target_fps(FPS);
        self.load_textures(&all_assets, &mut rl, &thread);

        (game, rl, thread)
    }

    pub fn update_rl(
        &self, 
        game: &mut Game, 
        time_since_last_update: f32,
        keyboard_events: &dyn KeyboardEventsProvider
    ) {
        game.update_rl(time_since_last_update, keyboard_events);
    } 

    fn load_textures(&mut self, all_assets: &Vec<String>, rl: &mut RaylibHandle, thread: &RaylibThread) {    
        for asset in all_assets {
            let texture = rl.load_texture(thread, asset).unwrap();
            self.textures.insert(asset.clone(), texture);
        }
    }
}

#[cfg(test)]
mod tests {    
    use crate::{constants::ASSETS_PATH, game_engine::{entity_factory::EntityFactory, game::Game}, utils::file_utils::list_files};

    use super::GameEngine;

    impl GameEngine {
        pub fn start_headless(&mut self) -> Game {
            let all_assets = list_files(ASSETS_PATH, "png");
            let mut game = Game::new(EntityFactory::new(all_assets));
            game.setup();            
            game
        }

        pub fn update(
            &self, 
            game: &mut Game, 
            time_since_last_update: f32
        ) {
            game.update(time_since_last_update);
        } 
    }

    #[test]
    fn can_launch_game_headless() {
        let mut engine = GameEngine::new();
        let game = engine.start_headless();
        assert_ne!(game.bounds.width, 10.0);
        assert_ne!(game.bounds.height, 10.0);
    }
}