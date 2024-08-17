use std::collections::HashMap;

use crate::{constants::{ASSETS_PATH, INITIAL_CAMERA_VIEWPORT}, utils::file_utils::list_files};

use super::{keyboard_events_provider::KeyboardEventsProvider, state_updates::EngineStateUpdate, world::World};
use common_macros::hash_map;
use raylib::prelude::*;

pub struct GameEngine {
    pub worlds: Vec<World>,
    pub textures: HashMap<String, Texture2D>,
    pub camera_viewport: Rectangle,
    pub rendering_scale: f32,
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            worlds: vec![],
            textures: hash_map![],
            camera_viewport: INITIAL_CAMERA_VIEWPORT,
            rendering_scale: 2.0,
        }
    }

    pub fn start_rl(&mut self) -> (RaylibHandle, RaylibThread) {                
        let mut world = World::new();
        world.setup();
        self.worlds.push(world);

        let (mut rl, thread) = raylib::init()
            .size(self.camera_viewport.width as i32, self.camera_viewport.height as i32)
            .resizable()
            .title("Tower Defense")
            .build();        
    
        // rl.set_target_fps(FPS);
        let all_assets = list_files(ASSETS_PATH, "png");
        self.load_textures(&all_assets, &mut rl, &thread);

        (rl, thread)
    }

    pub fn current_world_mut(&mut self) -> &mut World {
        self.worlds.last_mut().unwrap()
    }


    pub fn current_world(&self) -> &World {
        self.worlds.last().unwrap()
    }

    pub fn update_rl(
        &mut self, 
        time_since_last_update: f32,
        keyboard_events: &dyn KeyboardEventsProvider
    ) {
        let viewport = self.camera_viewport.clone();
        let world = self.current_world_mut();
        
        let state_updates = world.update_rl(time_since_last_update, &viewport, keyboard_events);

        self.camera_viewport = Rectangle::new(
            world.cached_hero_position.x - viewport.width / 2.0,
            world.cached_hero_position.y - viewport.height / 2.0,
            viewport.width,
            viewport.height
        );

        self.apply_state_updates(state_updates);
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

    fn apply_state_updates(&mut self, updates: Vec<EngineStateUpdate>) {
        updates.iter().for_each(|u| self.apply_state_update(u));
    }

    fn apply_state_update(&mut self, update: &EngineStateUpdate) {
        match update {
            EngineStateUpdate::PushWorld(name) => println!("Need to push level `{}`", name),
            EngineStateUpdate::PopWorld => println!("Need to pop level"),
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