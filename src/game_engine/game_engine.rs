use std::collections::HashMap;
use common_macros::hash_map;
use raylib::prelude::*;
use uuid::Uuid;

use crate::{constants::{ASSETS_PATH, FONT, FONT_BOLD, INITIAL_CAMERA_VIEWPORT, SPRITE_SHEET_BASE_ATTACK, SPRITE_SHEET_BIOME_TILES, SPRITE_SHEET_BUILDINGS, SPRITE_SHEET_CONSTRUCTION_TILES, SPRITE_SHEET_HUMANOIDS, SPRITE_SHEET_INVENTORY, SPRITE_SHEET_TELEPORTER, TILE_SIZE}, menus::menu::Menu, ui::ui::RenderingConfig, utils::{rect::Rect, vector::Vector2d}, worlds::constants::{WORLD_ID_DEMO, WORLD_ID_NONE}};

use super::{keyboard_events_provider::{KeyboardEventsProvider, KeyboardState}, state_updates::EngineStateUpdate, world::World};

pub struct GameEngine {
    pub menu: Menu,
    pub world: World,
    pub camera_viewport: Rect,
    pub camera_viewport_offset: Vector2d,
    pub ui_config: Option<RenderingConfig>,
    creative_mode: bool,
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            menu: Menu::new(),
            world: World::load_or_create(WORLD_ID_NONE),
            camera_viewport: INITIAL_CAMERA_VIEWPORT,
            camera_viewport_offset: Vector2d::zero(),
            ui_config: None,
            creative_mode: false
        }
    }

    pub fn set_creative_mode(&mut self, enabled: bool) {
        self.menu.set_creative_mode(enabled);
        self.world.creative_mode = enabled;
        self.creative_mode = enabled;
    }

    pub fn start_rl(&mut self) -> (RaylibHandle, RaylibThread) {
        let width = (TILE_SIZE * self.camera_viewport.w as f32) as i32;
        let height = (TILE_SIZE * self.camera_viewport.h as f32) as i32;

        let (mut rl, thread) = raylib::init()
            .size(width, height)
            .resizable()
            .title("Totally not Pokemon")
            .build();        
    
        let font = rl.load_font(&thread, FONT).unwrap();
        let font_bold = rl.load_font(&thread, FONT_BOLD).unwrap();            

        // rl.set_target_fps(FPS);
        
        self.switch_world(WORLD_ID_DEMO);

        let textures = self.load_textures(&mut rl, &thread);
        self.ui_config = Some(RenderingConfig { 
            font, 
            font_bold, 
            textures,
            rendering_scale: 2.0,
            font_rendering_scale: 2.0,
            canvas_size: Vector2d::new(1.0, 1.0)
        });

        self.window_size_changed(rl.get_screen_width(), rl.get_screen_height());

        (rl, thread)
    }

    pub fn update_rl(
        &mut self, 
        time_since_last_update: f32,
        keyboard_events: &dyn KeyboardEventsProvider
    ) {
        let mut engine_updates: Vec<EngineStateUpdate> = vec![];
        let camera_viewport = self.camera_viewport;
        let keyboard_state = keyboard_events.state();

        let menu_update = self.menu.update(&self.camera_viewport, &keyboard_state);

        let mut menu_engine_updates = self.world.apply_state_updates(menu_update.state_updates);
        engine_updates.append(&mut menu_engine_updates);

        let (world_keyboard_state, game_update_time) = if menu_update.game_paused {
            (KeyboardState::nothing(), time_since_last_update/10.0)
        } else {
            (keyboard_state, time_since_last_update)
        };
        let mut updates = self.world.update_rl(game_update_time, &camera_viewport, world_keyboard_state);
        engine_updates.append(&mut updates);

        self.apply_state_updates(engine_updates);
    } 

    fn load_textures(&self, rl: &mut RaylibHandle, thread: &RaylibThread) -> HashMap<u32, Texture2D> {    
        let mut textures: HashMap<u32, Texture2D> = hash_map!();
        textures.insert(SPRITE_SHEET_INVENTORY, texture(rl, thread, "inventory").unwrap());
        textures.insert(SPRITE_SHEET_BIOME_TILES, texture(rl, thread, "tiles_biome").unwrap());
        textures.insert(SPRITE_SHEET_CONSTRUCTION_TILES, texture(rl, thread, "tiles_constructions").unwrap());
        textures.insert(SPRITE_SHEET_BUILDINGS, texture(rl, thread, "buildings").unwrap());
        textures.insert(SPRITE_SHEET_BASE_ATTACK, texture(rl, thread, "baseattack").unwrap());
        textures.insert(SPRITE_SHEET_TELEPORTER, texture(rl, thread, "teleporter").unwrap());
        textures.insert(SPRITE_SHEET_HUMANOIDS, texture(rl, thread, "humanoids").unwrap());
        textures
    }

    pub fn window_size_changed(&mut self, width: i32, height: i32) {
        let (scale, font_scale) = self.rendering_scale_for_screen_width(width);
        self.ui_config.as_mut().unwrap().rendering_scale = scale;
        self.ui_config.as_mut().unwrap().font_rendering_scale = font_scale;
        self.ui_config.as_mut().unwrap().canvas_size.x = width as f32;
        self.ui_config.as_mut().unwrap().canvas_size.y = height as f32;
        self.camera_viewport.w = (width as f32 / (scale * TILE_SIZE)) as u32;
        self.camera_viewport.h = (height as f32 / (scale * TILE_SIZE)) as u32;
    }

    fn rendering_scale_for_screen_width(&self, width: i32) -> (f32, f32) {
        if width < 500 {
            (1.0, 1.0)
        } else if width < 1400 {
            (2.0, 2.0)
        } else {
            ((width as f32 / 1000.0).ceil(), 3.0)
        }
    }

    fn apply_state_updates(&mut self, updates: Vec<EngineStateUpdate>) {
        let mut sorted_updates = updates.clone();

        sorted_updates.sort_by(|a, b| {
            use EngineStateUpdate::*;
            
            match (a, b) {
                (SwitchWorld(_), SwitchWorld(_)) => std::cmp::Ordering::Equal,
                (SwitchWorld(_), _) => std::cmp::Ordering::Greater,
                (_, SwitchWorld(_)) => std::cmp::Ordering::Less,
                _ => std::cmp::Ordering::Equal,
            }
        });
        
        sorted_updates.iter().for_each(|u| self.apply_state_update(u));
    }

    fn apply_state_update(&mut self, update: &EngineStateUpdate) {
        match update {
            EngineStateUpdate::CenterCamera(x, y, offset) => self.center_camera_at(*x, *y, offset),            
            EngineStateUpdate::SwitchWorld(id) => self.switch_world(*id),
            EngineStateUpdate::SaveGame => self.save()
        }
    }

    fn save(&self) {
        self.world.save();
    }

    fn switch_world(&mut self, id: Uuid) {
        self.world.save();
        
        let mut new_world = World::load_or_create(id);
        new_world.creative_mode = self.creative_mode;
        new_world.setup(&self.world.id, &self.world.cached_hero_props.direction);
        new_world.update(0.001);
        let hero_frame = new_world.cached_hero_props.frame;
        self.world = new_world;
        self.center_camera_in(&hero_frame);
    }

    fn center_camera_in(&mut self, frame: &Rect) {
        self.camera_viewport.center_in(frame);
    }

    fn center_camera_at(&mut self, x: u32, y: u32, offset: &Vector2d) {
        self.camera_viewport.center_at(&Vector2d::new(x as f32, y as f32));
        self.camera_viewport_offset = *offset;
    }

    pub fn rendering_scale(&self) -> f32 {
        self.ui_config.as_ref().unwrap().rendering_scale
    }
}

fn texture(rl: &mut RaylibHandle, thread: &RaylibThread, name: &str) -> Option<Texture2D> {
    let path = format!("{}/{}.png", ASSETS_PATH, name);
    let result = rl.load_texture(thread, &path);
    
    match result {
        Ok(texture) => Some(texture),
        Err(err) => {
            eprintln!("Failed to load texture at {}: {:#?}", path, err);
            None
        }
    }
}

#[cfg(test)]
mod tests {    
    use crate::{game_engine::{keyboard_events_provider::NoKeyboardEvents, world::World}, utils::vector::Vector2d, worlds::constants::{WORLD_ID_DEMO, WORLD_ID_NONE}};

    use super::GameEngine;

    impl GameEngine {
        pub fn start_headless(&mut self) -> World {
            let mut world = World::new(WORLD_ID_DEMO);
            world.setup(&WORLD_ID_NONE, &Vector2d::zero());            
            world
        }
    }

    #[test]
    fn can_launch_game_headless() {
        let mut engine = GameEngine::new();
        let world = engine.start_headless();
        assert_ne!(world.bounds.w, 10);
        assert_ne!(world.bounds.h, 10);
    }
}