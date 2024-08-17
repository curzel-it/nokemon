use std::collections::HashMap;

use crate::{constants::{ASSETS_PATH, INITIAL_CAMERA_VIEWPORT}, utils::file_utils::list_files};

use super::{world::World, keyboard_events_provider::KeyboardEventsProvider};
use raylib::prelude::*;

pub struct GameEngine {
    pub textures: HashMap<String, Texture2D>,
    pub camera_viewport: Rectangle,
    pub rendering_scale: f32,
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            camera_viewport: INITIAL_CAMERA_VIEWPORT,
            rendering_scale: 2.0,
        }
    }

    pub fn start_rl(&mut self) -> (World, RaylibHandle, RaylibThread) {                
        let mut world = World::new();
        world.setup();

        let (mut rl, thread) = raylib::init()
            .size(self.camera_viewport.width as i32, self.camera_viewport.height as i32)
            .resizable()
            .title("Tower Defense")
            .build();        
    
        // rl.set_target_fps(FPS);
        let all_assets = list_files(ASSETS_PATH, "png");
        self.load_textures(&all_assets, &mut rl, &thread);

        (world, rl, thread)
    }

    pub fn update_rl(
        &mut self, 
        world: &mut World, 
        time_since_last_update: f32,
        keyboard_events: &dyn KeyboardEventsProvider
    ) {
        world.update_rl(time_since_last_update, &self.camera_viewport, keyboard_events);

        self.camera_viewport = Rectangle::new(
            world.cached_hero_position.x - self.camera_viewport.width / 2.0,
            world.cached_hero_position.y - self.camera_viewport.height / 2.0,
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

    pub fn adjust_camera_from_screen_size(&mut self, width: i32, height: i32) {
        self.rendering_scale = self.rendering_scale_for_screen_width(width);
        self.camera_viewport.width = width as f32 / self.rendering_scale;
        self.camera_viewport.height = height as f32 / self.rendering_scale;
    }

    fn rendering_scale_for_screen_width(&self, width: i32) -> f32 {
        if width < 500 {
            1.0
        } else if width < 1400 {
            2.0
        } else {
            (width as f32 / 1000.0).ceil()
        }
    }
}

#[cfg(test)]
mod tests {    
    use crate::game_engine::world::World;

    use super::GameEngine;

    impl GameEngine {
        pub fn start_headless(&mut self) -> World {
            let mut world = World::new();
            world.setup();            
            world
        }

        pub fn update(&self, world: &mut World, time_since_last_update: f32) {
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