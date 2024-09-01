use std::collections::HashMap;
use common_macros::hash_map;
use raylib::prelude::*;
use crate::{constants::{ASSETS_PATH, FONT, FONT_BOLD, INITIAL_CAMERA_VIEWPORT, SPRITE_SHEET_BASE_ATTACK, SPRITE_SHEET_BIOME_TILES, SPRITE_SHEET_BUILDINGS, SPRITE_SHEET_CONSTRUCTION_TILES, SPRITE_SHEET_HOUSEHOLD_OBJECTS, SPRITE_SHEET_HUMANOIDS, SPRITE_SHEET_INVENTORY, SPRITE_SHEET_MENU, SPRITE_SHEET_TELEPORTER, TILE_SIZE, WORLD_ID_DEMO, WORLD_ID_NONE}, dialogues::dialogue_menu::DialogueMenu, features::loading_screen::LoadingScreen, menus::{entity_options::EntityOptionsMenu, game_menu::GameMenu, npc_options::NpcOptionsMenu}, ui::components::RenderingConfig, utils::{rect::Rect, vector::Vector2d}};

use super::{keyboard_events_provider::{KeyboardEventsProvider, NO_KEYBOARD_EVENTS}, state_updates::EngineStateUpdate, world::World};

pub struct GameEngine {
    pub menu: GameMenu,
    pub world: World,
    pub loading_screen: LoadingScreen,
    pub npc_options_menu: NpcOptionsMenu,
    pub dialogue_menu: DialogueMenu,
    pub entity_options_menu: EntityOptionsMenu,
    pub keyboard: KeyboardEventsProvider,
    pub camera_viewport: Rect,
    pub camera_viewport_offset: Vector2d,
    pub ui_config: Option<RenderingConfig>,
    pub is_running: bool,
    creative_mode: bool,
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            menu: GameMenu::new(),
            world: World::load_or_create(WORLD_ID_NONE),
            loading_screen: LoadingScreen::new(),
            npc_options_menu: NpcOptionsMenu::new(),
            dialogue_menu: DialogueMenu::new(),
            entity_options_menu: EntityOptionsMenu::new(),
            keyboard: KeyboardEventsProvider::new(),
            camera_viewport: INITIAL_CAMERA_VIEWPORT,
            camera_viewport_offset: Vector2d::zero(),
            ui_config: None,
            is_running: true,
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
        rl: &mut RaylibHandle,
        time_since_last_update: f32,
    ) {
        self.keyboard.update(rl, time_since_last_update);
        self.update(time_since_last_update);
    } 

    fn update(&mut self, time_since_last_update: f32,) {
        self.loading_screen.update(time_since_last_update);
        if self.loading_screen.progress() < 0.4 { 
            return;
        }

        let camera_viewport = self.camera_viewport;
        let is_game_paused = self.update_menus(time_since_last_update);

        let (world_keyboard, game_update_time) = if is_game_paused {
            (&NO_KEYBOARD_EVENTS, time_since_last_update/10.0)
        } else {
            (&self.keyboard, time_since_last_update)
        };

        let updates = self.world.update_rl(game_update_time, &camera_viewport, world_keyboard);
        self.apply_state_updates(updates);
    } 

    fn update_menus(&mut self, time_since_last_update: f32) -> bool {
        let mut is_game_paused = false;

        if !is_game_paused {
            let keyboard = if self.dialogue_menu.is_open() { &self.keyboard } else { &NO_KEYBOARD_EVENTS };
            let (pause, world_updates) = self.dialogue_menu.update(keyboard, time_since_last_update);
            is_game_paused = is_game_paused || pause;
            let engine_updates = self.world.apply_state_updates(world_updates);
            self.apply_state_updates(engine_updates);
        }

        if !is_game_paused {
            let keyboard = if self.menu.is_open() { &self.keyboard } else { &NO_KEYBOARD_EVENTS };
            let (pause, world_updates) = self.menu.update(&self.camera_viewport, keyboard, time_since_last_update);
            is_game_paused = is_game_paused || pause;
            let engine_updates = self.world.apply_state_updates(world_updates);
            self.apply_state_updates(engine_updates);
        }

        if !is_game_paused {
            let keyboard = if self.entity_options_menu.is_open() { &self.keyboard } else { &NO_KEYBOARD_EVENTS };
            let (pause, world_updates) = self.entity_options_menu.update(keyboard, time_since_last_update);
            is_game_paused = is_game_paused || pause;
            let engine_updates = self.world.apply_state_updates(world_updates);
            self.apply_state_updates(engine_updates);
        }

        if !is_game_paused {
            let keyboard = if self.npc_options_menu.is_open() { &self.keyboard } else { &NO_KEYBOARD_EVENTS };
            let (pause, world_updates) = self.npc_options_menu.update(keyboard, time_since_last_update);
            is_game_paused = is_game_paused || pause;
            let engine_updates = self.world.apply_state_updates(world_updates);
            self.apply_state_updates(engine_updates);
        }
        
        is_game_paused
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
        textures.insert(SPRITE_SHEET_HOUSEHOLD_OBJECTS, texture(rl, thread, "household_objects").unwrap());
        textures.insert(SPRITE_SHEET_MENU, texture(rl, thread, "menu").unwrap());        
        textures
    }

    pub fn window_size_changed(&mut self, width: i32, height: i32) {
        let (scale, font_scale) = self.rendering_scale_for_screen_width(width);
        self.ui_config.as_mut().unwrap().rendering_scale = scale;
        self.ui_config.as_mut().unwrap().font_rendering_scale = font_scale;
        self.ui_config.as_mut().unwrap().canvas_size.x = width as f32;
        self.ui_config.as_mut().unwrap().canvas_size.y = height as f32;
        self.camera_viewport.w = (width as f32 / (scale * TILE_SIZE)) as i32;
        self.camera_viewport.h = (height as f32 / (scale * TILE_SIZE)) as i32;
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
            EngineStateUpdate::ShowDialogue(npc_id, dialogue_id) => self.show_dialogue(npc_id, dialogue_id),
            EngineStateUpdate::ShowNpcOptions(id, dialogue_id) => self.npc_options_menu.show(*id, *dialogue_id),
            EngineStateUpdate::CenterCamera(x, y, offset) => self.center_camera_at(*x, *y, offset),            
            EngineStateUpdate::SwitchWorld(id) => self.switch_world(*id),
            EngineStateUpdate::SaveGame => self.save(),
            EngineStateUpdate::Exit => self.exit(),
            EngineStateUpdate::ShowEntityOptions(id) => self.entity_options_menu.show(*id),
        }
    }

    fn show_dialogue(&mut self, npc_id: &u32, dialogue_id: &u32) {
        let config = self.ui_config.as_ref().unwrap();
        self.dialogue_menu.show(*npc_id, *dialogue_id, config);
    }    

    fn exit(&mut self) {
        println!("Got exit request!");
        self.is_running = false;
    }

    fn save(&self) {
        self.world.save();
    }

    fn switch_world(&mut self, id: u32) {
        self.loading_screen.animate_world_transition();
        self.world.save();
        
        let mut new_world = World::load_or_create(id);
        new_world.creative_mode = self.creative_mode;
        new_world.setup(&self.world.id, &self.world.cached_hero_props.direction);
        new_world.update(0.001);
        let hero_frame = new_world.cached_hero_props.frame;
        self.world = new_world;
        self.center_camera_in(&hero_frame);

        self.menu.current_world_id = self.world.id;
        self.keyboard.on_world_changed();
    }

    fn center_camera_in(&mut self, frame: &Rect) {
        self.camera_viewport.center_in(frame);
    }

    fn center_camera_at(&mut self, x: i32, y: i32, offset: &Vector2d) {
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
    use crate::{constants::{WORLD_ID_DEMO, WORLD_ID_NONE}, game_engine::world::World, utils::directions::Direction};

    use super::GameEngine;

    impl GameEngine {
        pub fn start_headless(&mut self) -> World {
            let mut world = World::new(WORLD_ID_DEMO);
            world.setup(&WORLD_ID_NONE, &Direction::Unknown);
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