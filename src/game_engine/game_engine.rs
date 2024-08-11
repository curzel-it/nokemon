use std::collections::HashMap;

use crate::{constants::{ASSETS_PATH, FPS}, utils::file_utils::list_files};

use super::{entity_factory::EntityFactory, world::World, keyboard_events_provider::KeyboardEventsProvider};
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

    pub fn start_rl(&mut self) -> (World, RaylibHandle, RaylibThread) {        
        let all_assets = list_files(ASSETS_PATH, "png");

        let mut world = World::new(EntityFactory::new(all_assets.clone()));
        world.setup();

        let (mut rl, thread) = raylib::init()
            .size(world.camera_viewport.width as i32, world.camera_viewport.height as i32)
            .resizable()
            .title("Tower Defense")
            .build();
    
        rl.set_target_fps(FPS);
        self.load_textures(&all_assets, &mut rl, &thread);

        (world, rl, thread)
    }

    pub fn update_rl(
        &self, 
        world: &mut World, 
        time_since_last_update: f32,
        keyboard_events: &dyn KeyboardEventsProvider
    ) {
        world.update_rl(time_since_last_update, keyboard_events);
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
    use crate::{constants::ASSETS_PATH, game_engine::{entity_factory::EntityFactory, world::World}, utils::file_utils::list_files};

    use super::GameEngine;

    impl GameEngine {
        pub fn start_headless(&mut self) -> World {
            let all_assets = list_files(ASSETS_PATH, "png");
            let mut world = World::new(EntityFactory::new(all_assets));
            world.setup();            
            world
        }

        pub fn update(
            &self, 
            world: &mut World, 
            time_since_last_update: f32
        ) {
            world.update(time_since_last_update);
        } 
    }

    #[test]
    fn can_launch_game_headless() {
        let mut engine = GameEngine::new();
        let world = engine.start_headless();
        assert_ne!(world.bounds.width, 10.0);
        assert_ne!(world.bounds.height, 10.0);
    }
}