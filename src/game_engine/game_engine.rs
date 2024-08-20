use std::collections::HashMap;

use crate::{constants::{ASSETS_PATH, FONT, FONT_BOLD, FPS, INITIAL_CAMERA_VIEWPORT}, features::{interactions::handle_interactions, inventory::Inventory}, levels::constants::LEVEL_DEMO_WORLD, ui::ui::UiConfig, utils::file_utils::list_files};

use super::{keyboard_events_provider::{KeyboardEventsProvider, KeyboardState}, state_updates::EngineStateUpdate, world::World};
use common_macros::hash_map;
use raylib::prelude::*;

pub struct GameEngine {
    pub inventory: Inventory,
    pub worlds: Vec<World>,
    // pub textures: HashMap<String, Texture2D>,
    pub camera_viewport: Rectangle,
    pub rendering_scale: f32,
    pub ui_config: Option<UiConfig>
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            inventory: Inventory::new(),
            worlds: vec![],
            // textures: hash_map![],
            camera_viewport: INITIAL_CAMERA_VIEWPORT,
            rendering_scale: 2.0,
            ui_config: None
        }
    }

    pub fn with_options(creative_mode: bool) -> Self {
        let mut engine = Self::new();
        engine.inventory.set_creative_mode(creative_mode);
        engine
    }

    pub fn start_rl(&mut self) -> (RaylibHandle, RaylibThread) {
        let (mut rl, thread) = raylib::init()
            .size(self.camera_viewport.width as i32, self.camera_viewport.height as i32)
            .resizable()
            .title("Totally not Pokemon")
            .build();        
    
        let font = rl.load_font(&thread, FONT).unwrap();
        let font_bold = rl.load_font(&thread, FONT_BOLD).unwrap();            

        // rl.set_target_fps(FPS);

        self.adjust_camera_from_screen_size(rl.get_screen_width(), rl.get_screen_height());
        self.push_world(LEVEL_DEMO_WORLD);

        let all_assets = list_files(ASSETS_PATH, "png");
        let textures = self.load_textures(&all_assets, &mut rl, &thread);
        self.ui_config = Some(UiConfig { font, font_bold, textures });

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
        let world = self.current_world();
        let camera_viewport = self.camera_viewport;
        let inventory_is_open = self.inventory.is_open;

        let handled = !inventory_is_open && handle_interactions(world);

        let inventory_keyboard_state = if handled {
            KeyboardState::nothing()
        } else {
            keyboard_events.state()
        };

        let world_keyboard_state = if inventory_is_open || handled {
            KeyboardState::nothing()
        } else {
            keyboard_events.state()
        };

        let world = self.current_world_mut();
        let state_updates = world.update_rl(time_since_last_update, &camera_viewport, world_keyboard_state);
                
        let world_updates = self.inventory.update(&camera_viewport, &inventory_keyboard_state);
        let world = self.current_world_mut();
        world.apply_state_updates(world_updates);

        self.apply_state_updates(state_updates);
    } 

    fn load_textures(&self, all_assets: &Vec<String>, rl: &mut RaylibHandle, thread: &RaylibThread) -> HashMap<String, Texture2D> {    
        let mut textures: HashMap<String, Texture2D> = hash_map!();
        for asset in all_assets {
            println!("{}", asset);
            let texture = rl.load_texture(thread, asset).unwrap();
            textures.insert(asset.clone(), texture);
        }
        textures
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
            EngineStateUpdate::CenterCamera(x, y) => self.center_camera_at(*x, *y),            
            EngineStateUpdate::PushWorld(id) => self.push_world(*id),
            EngineStateUpdate::PopWorld => self.pop_world(),
            EngineStateUpdate::ToggleWorld(id) => self.toggle_world(*id),
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
    use crate::{game_engine::{keyboard_events_provider::NoKeyboardEvents, world::World}, levels::constants::LEVEL_DEMO_WORLD};

    use super::GameEngine;

    impl GameEngine {
        pub fn start_headless(&mut self) -> World {
            let mut world = World::new(LEVEL_DEMO_WORLD);
            world.setup();            
            world
        }

        pub fn update(&mut self, time_since_last_update: f32) {
            let nokb = NoKeyboardEvents {};
            self.update_rl(time_since_last_update, &nokb);
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