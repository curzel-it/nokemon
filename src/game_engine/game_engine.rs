use std::collections::HashMap;

use crate::{constants::{ASSETS_PATH, FPS, INITIAL_CAMERA_VIEWPORT}, utils::file_utils::list_files};

use super::{entity_factory::EntityFactory, game::Game, keyboard_events_provider::KeyboardEventsProvider};
use raylib::prelude::*;

pub struct GameEngine {
    pub camera_viewport: Rectangle,
    pub textures: HashMap<String, Texture2D>
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            camera_viewport: INITIAL_CAMERA_VIEWPORT,
            textures: HashMap::new()
        }
    }

    pub fn start_rl(&mut self) -> (Game, RaylibHandle, RaylibThread) {
        let (mut rl, thread) = raylib::init()
            .size(self.camera_viewport.width as i32, self.camera_viewport.height as i32)
            .title("Tower Defense")
            .build();
    
        rl.set_target_fps(FPS);
        
        let all_assets = list_files(ASSETS_PATH, "png");
        self.load_textures(&all_assets, &mut rl, &thread);

        let mut game = Game::new(EntityFactory::new(all_assets));
        game.setup();

        (game, rl, thread)
    }

    pub fn update(
        &mut self, 
        game: &mut Game, 
        time_since_last_update: f32,
        keyboard_events: &dyn KeyboardEventsProvider
    ) {
        game.update_rl(time_since_last_update, keyboard_events);

        self.camera_viewport = Rectangle::new(
            game.cached_hero_position.x - self.camera_viewport.width / 2.0,
            game.cached_hero_position.y - self.camera_viewport.height / 2.0,
            self.camera_viewport.width,
            self.camera_viewport.height
        );
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
    

    use crate::{constants::{ASSETS_PATH, GAME_SIZE}, game_engine::{entity_factory::EntityFactory, game::Game}, utils::file_utils::list_files};

    use super::GameEngine;

    impl GameEngine {
        fn start_headless(&mut self) -> Game {
            let all_assets = list_files(ASSETS_PATH, "png");
            let mut game = Game::new(EntityFactory::new(all_assets));
            game.setup();            
            game
        }
    }

    #[test]
    fn can_launch_game_headless() {
        let mut engine = GameEngine::new();
        let game = engine.start_headless();
        assert_eq!(game.bounds.width, GAME_SIZE.x);
        assert_eq!(game.bounds.height, GAME_SIZE.y);
    }
}