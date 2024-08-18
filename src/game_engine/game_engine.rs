use std::collections::HashMap;

use crate::{constants::{ASSETS_PATH, INITIAL_CAMERA_VIEWPORT}, levels::constants::LEVEL_DEMO_WORLD, utils::file_utils::list_files};

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
        let (mut rl, thread) = raylib::init()
            .size(self.camera_viewport.width as i32, self.camera_viewport.height as i32)
            .resizable()
            .title("Totally not Pokemon")
            .build();        
    
        // rl.set_target_fps(FPS);

        let all_assets = list_files(ASSETS_PATH, "png");
        self.load_textures(&all_assets, &mut rl, &thread);
        self.adjust_camera_from_screen_size(rl.get_screen_width(), rl.get_screen_height());
        self.push_world(LEVEL_DEMO_WORLD);

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
        let mut sorted_updates = updates.clone();

        sorted_updates.sort_by(|a, b| {
            use EngineStateUpdate::*;
            
            match (a, b) {
                (ToggleWorld(_), ToggleWorld(_)) => std::cmp::Ordering::Equal,
                (ToggleWorld(_), _) => std::cmp::Ordering::Greater,
                (_, ToggleWorld(_)) => std::cmp::Ordering::Less,
                _ => std::cmp::Ordering::Equal,
            }
        });
        
        sorted_updates.iter().for_each(|u| self.apply_state_update(u));
    }

    fn apply_state_update(&mut self, update: &EngineStateUpdate) {
        match update {
            EngineStateUpdate::CenterCamera(x, y) => self.center_camera_at(x.clone(), y.clone()),            
            EngineStateUpdate::PushWorld(id) => self.push_world(id.clone()),
            EngineStateUpdate::PopWorld => self.pop_world(),
            EngineStateUpdate::ToggleWorld(id) => self.toggle_world(id.clone()),
        }
    }

    fn toggle_world(&mut self, id: u32) {
        if self.current_world().level_id == id {
            self.pop_world();
        } else {
            self.push_world(id);
        }
    }

    fn push_world(&mut self, id: u32) {
        println!("Pushing world {}", id);
        if !self.worlds.is_empty() {
            self.current_world_mut().move_hero_one_tile_down();
        }

        let mut new_level = World::new(id);
        new_level.setup();
        new_level.update(0.001);
        let hero_frame = new_level.cached_hero_props.frame;
        self.worlds.push(new_level);
        self.center_camera_in(&hero_frame);
    }

    fn pop_world(&mut self) {
        println!("Popping world...");
        self.worlds.pop();
        let hero_frame = self.current_world().cached_hero_props.frame;
        self.center_camera_in(&hero_frame);
    }

    fn center_camera_in(&mut self, frame: &Rectangle) {
        self.center_camera_at(
            frame.x + frame.width / 2.0,
            frame.y + frame.height / 2.0
        );
    }

    fn center_camera_at(&mut self, x: f32, y: f32) {
        self.camera_viewport = Rectangle::new(
            x - self.camera_viewport.width / 2.0,
            y - self.camera_viewport.height / 2.0,
            self.camera_viewport.width,
            self.camera_viewport.height
        );
    }
}

#[cfg(test)]
mod tests {    
    use crate::{levels::constants::LEVEL_DEMO_WORLD, game_engine::world::World};

    use super::GameEngine;

    impl GameEngine {
        pub fn start_headless(&mut self) -> World {
            let mut world = World::new(LEVEL_DEMO_WORLD);
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