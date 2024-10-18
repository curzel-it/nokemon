use std::collections::HashMap;
use common_macros::hash_map;
use raylib::prelude::*;
use crate::{constants::{ASSETS_PATH, FONT, FONT_BOLD, INITIAL_CAMERA_VIEWPORT, SPRITE_SHEET_ANIMATED_OBJECTS, SPRITE_SHEET_AVATARS, SPRITE_SHEET_BASE_ATTACK, SPRITE_SHEET_BIOME_TILES, SPRITE_SHEET_BUILDINGS, SPRITE_SHEET_CONSTRUCTION_TILES, SPRITE_SHEET_FARM_PLANTS, SPRITE_SHEET_HUMANOIDS_1X1, SPRITE_SHEET_HUMANOIDS_1X2, SPRITE_SHEET_HUMANOIDS_2X2, SPRITE_SHEET_INVENTORY, SPRITE_SHEET_MENU, SPRITE_SHEET_STATIC_OBJECTS, TILE_SIZE, WORLD_ID_NONE}, dialogues::{menu::DialogueMenu, models::Dialogue}, features::{creep_spawner::CreepSpawner, death_screen::DeathScreen, destination::Destination, loading_screen::LoadingScreen}, menus::{confirmation::ConfirmationDialog, entity_options::EntityOptionsMenu, game_menu::GameMenu, long_text_display::LongTextDisplay, toasts::{Toast, ToastDisplay}}, ui::components::{RenderingConfig, Typography}, utils::{rect::Rect, vector::Vector2d}};

use super::{inventory::{add_to_inventory, remove_from_inventory}, keyboard_events_provider::{KeyboardEventsProvider, NO_KEYBOARD_EVENTS}, mouse_events_provider::MouseEventsProvider, state_updates::{EngineStateUpdate, WorldStateUpdate}, storage::{get_value_for_key, set_value_for_key, StorageKey}, world::World};

pub struct GameEngine {
    pub menu: GameMenu,
    pub world: World,
    pub loading_screen: LoadingScreen,
    pub long_text_display: LongTextDisplay,
    pub confirmation_dialog: ConfirmationDialog,
    pub death_screen: DeathScreen,
    pub dialogue_menu: DialogueMenu,
    pub toast: ToastDisplay,
    pub creep_spawner: CreepSpawner,
    pub entity_options_menu: EntityOptionsMenu,
    pub keyboard: KeyboardEventsProvider,
    pub mouse: MouseEventsProvider,
    pub camera_viewport: Rect,
    pub camera_viewport_offset: Vector2d,
    pub ui_config: Option<RenderingConfig>,
    pub is_running: bool,
    pub creative_mode: bool,
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            menu: GameMenu::new(),
            world: World::load_or_create(WORLD_ID_NONE),
            loading_screen: LoadingScreen::new(),
            long_text_display: LongTextDisplay::new(50, 9),
            confirmation_dialog: ConfirmationDialog::new(),
            death_screen: DeathScreen::new(),
            dialogue_menu: DialogueMenu::new(),
            toast: ToastDisplay::new(),
            creep_spawner: CreepSpawner::new(),
            entity_options_menu: EntityOptionsMenu::new(),
            keyboard: KeyboardEventsProvider::new(),
            mouse: MouseEventsProvider::new(),
            camera_viewport: INITIAL_CAMERA_VIEWPORT,
            camera_viewport_offset: Vector2d::zero(),
            ui_config: None,
            is_running: true,
            creative_mode: false
        }
    }

    pub fn set_creative_mode(&mut self, enabled: bool) {
        self.menu.set_creative_mode(enabled);
        self.world.set_creative_mode(enabled);
        self.creative_mode = enabled;
    }

    pub fn start_rl(&mut self) -> (RaylibHandle, RaylibThread) {
        let width = (TILE_SIZE * self.camera_viewport.w as f32) as i32;
        let height = (TILE_SIZE * self.camera_viewport.h as f32) as i32;

        let (mut rl, thread) = raylib::init()
            .size(width, height)
            .resizable()
            .title("Nokemon")
            .build();        
    
        let font = rl.load_font(&thread, FONT).unwrap();
        let font_bold = rl.load_font(&thread, FONT_BOLD).unwrap();            

        // rl.set_target_fps(60.0);
        
        self.teleport_to_previous();

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

    pub fn update_rl(&mut self, rl: &mut RaylibHandle, real_time_since_last_update: f32) {
        let time_since_last_update = real_time_since_last_update.min(0.1);
        self.keyboard.update(rl, time_since_last_update);
        self.mouse.update(rl, self.ui_config.as_ref().unwrap().rendering_scale);
        self.update(time_since_last_update);
    } 

    fn update(&mut self, time_since_last_update: f32) {        
        self.toast.update(time_since_last_update);

        if self.death_screen.is_open {
            return;
        }

        self.loading_screen.update(time_since_last_update);
        if self.loading_screen.progress() < 0.4 { 
            return;
        }

        let camera_viewport = self.camera_viewport;
        let is_game_paused = self.update_menus(time_since_last_update);

        let (world_keyboard, game_update_time) = if is_game_paused {
            (&NO_KEYBOARD_EVENTS, time_since_last_update/20.0)
        } else {
            (&self.keyboard, time_since_last_update)
        };

        let updates = self.world.update_rl(game_update_time, &camera_viewport, world_keyboard);
        self.apply_state_updates(updates);

        let creeps_world_updates = self.creep_spawner.update(&self.world, time_since_last_update);
        let creeps_engine_updates = self.world.apply_state_updates(creeps_world_updates);
        self.apply_state_updates(creeps_engine_updates);
    } 

    fn update_menus(&mut self, time_since_last_update: f32) -> bool {
        let mut is_game_paused = false;

        if !is_game_paused {
            let keyboard = if self.long_text_display.is_open { &self.keyboard } else { &NO_KEYBOARD_EVENTS };
            let is_reading = self.long_text_display.update(keyboard, time_since_last_update);
            is_game_paused = is_game_paused || is_reading;
        }

        if !is_game_paused {
            let keyboard = if self.confirmation_dialog.is_open() { &self.keyboard } else { &NO_KEYBOARD_EVENTS };
            let (pause, world_updates) = self.confirmation_dialog.update(keyboard, time_since_last_update);
            is_game_paused = is_game_paused || pause;
            let engine_updates = self.world.apply_state_updates(world_updates);
            self.apply_state_updates(engine_updates);
        }

        if !is_game_paused {
            let keyboard = if self.dialogue_menu.is_open() { &self.keyboard } else { &NO_KEYBOARD_EVENTS };
            let (pause, world_updates) = self.dialogue_menu.update(keyboard, time_since_last_update);
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
            let can_handle = self.menu.is_open() || self.keyboard.has_menu_been_pressed;
            let keyboard = if can_handle { &self.keyboard } else { &NO_KEYBOARD_EVENTS };
            let (pause, world_updates) = self.menu.update(&self.camera_viewport, keyboard, &self.mouse, time_since_last_update);
            is_game_paused = is_game_paused || pause;
            let engine_updates = self.world.apply_state_updates(world_updates);
            self.apply_state_updates(engine_updates);
        }
        
        is_game_paused
    }

    fn teleport_to_previous(&mut self) {
        if let Some(world) = get_value_for_key(&StorageKey::latest_world()) {
            self.teleport(&Destination::new(world, 0, 0));
        } else {
            self.teleport(&Destination::default());
        }
    }

    fn load_textures(&self, rl: &mut RaylibHandle, thread: &RaylibThread) -> HashMap<u32, Texture2D> {    
        let mut textures: HashMap<u32, Texture2D> = hash_map!();
        textures.insert(SPRITE_SHEET_INVENTORY, texture(rl, thread, "inventory").unwrap());
        textures.insert(SPRITE_SHEET_BIOME_TILES, texture(rl, thread, "tiles_biome").unwrap());
        textures.insert(SPRITE_SHEET_CONSTRUCTION_TILES, texture(rl, thread, "tiles_constructions").unwrap());
        textures.insert(SPRITE_SHEET_BUILDINGS, texture(rl, thread, "buildings").unwrap());
        textures.insert(SPRITE_SHEET_BASE_ATTACK, texture(rl, thread, "baseattack").unwrap());
        textures.insert(SPRITE_SHEET_STATIC_OBJECTS, texture(rl, thread, "static_objects").unwrap());
        textures.insert(SPRITE_SHEET_MENU, texture(rl, thread, "menu").unwrap());        
        textures.insert(SPRITE_SHEET_ANIMATED_OBJECTS, texture(rl, thread, "animated_objects").unwrap());     
        textures.insert(SPRITE_SHEET_HUMANOIDS_1X1, texture(rl, thread, "humanoids_1x1").unwrap());      
        textures.insert(SPRITE_SHEET_HUMANOIDS_1X2, texture(rl, thread, "humanoids_1x2").unwrap());
        textures.insert(SPRITE_SHEET_HUMANOIDS_2X2, texture(rl, thread, "humanoids_2x2").unwrap());
        textures.insert(SPRITE_SHEET_AVATARS, texture(rl, thread, "avatars").unwrap());     
        textures.insert(SPRITE_SHEET_FARM_PLANTS, texture(rl, thread, "farm_plants").unwrap());             
        textures
    }

    pub fn window_size_changed(&mut self, width: i32, height: i32) {
        println!("Window size changed to {}x{}", width, height);
        let (scale, font_scale) = self.rendering_scale_for_screen_width(width);
        
        println!("Updated rendering scale to {}", scale);
        println!("Updated font scale to {}", scale);
        
        self.ui_config.as_mut().unwrap().rendering_scale = scale;
        self.ui_config.as_mut().unwrap().font_rendering_scale = font_scale;
        self.ui_config.as_mut().unwrap().canvas_size.x = width as f32;
        self.ui_config.as_mut().unwrap().canvas_size.y = height as f32;

        self.camera_viewport.w = (width as f32 / (scale * TILE_SIZE)) as i32;
        self.camera_viewport.h = (height as f32 / (scale * TILE_SIZE)) as i32;

        let font_size = self.ui_config.as_ref().unwrap().scaled_font_size(&Typography::Regular);
        let line_spacing = self.ui_config.as_ref().unwrap().font_lines_spacing(&Typography::Regular);
        self.long_text_display.max_line_length = (width as f32 / font_size).floor() as usize;
        self.long_text_display.visible_line_count = (0.3 * height as f32 / (line_spacing + font_size)).floor() as usize;
    }

    fn rendering_scale_for_screen_width(&self, width: i32) -> (f32, f32) {
        if self.creative_mode {
            return (1.0, 2.0)
        }
        if width < 500 {
            (1.0, 1.0)
        } else if width < 1400 || self.creative_mode {
            (2.0, 2.0)
        } else {
            let scale = (width as f32 / 1000.0).ceil();
            (scale, scale)
        }
    }

    fn apply_state_updates(&mut self, updates: Vec<EngineStateUpdate>) {
        let mut sorted_updates = updates.clone();

        sorted_updates.sort_by(|a, b| {
            use EngineStateUpdate::*;
            
            match (a, b) {
                (Teleport(_), Teleport(_)) => std::cmp::Ordering::Equal,
                (Teleport(_), _) => std::cmp::Ordering::Greater,
                (_, Teleport(_)) => std::cmp::Ordering::Less,
                _ => std::cmp::Ordering::Equal,
            }
        });
        
        sorted_updates.iter().for_each(|u| self.apply_state_update(u));
    }

    fn log_update(&self, update: &EngineStateUpdate) {
        match update {
            EngineStateUpdate::CenterCamera(_, _, _) => {},
            _ => println!("Engine update: {:#?}", update)
        }     
    }

    fn apply_state_update(&mut self, update: &EngineStateUpdate) {   
        self.log_update(update);

        match update {
            EngineStateUpdate::ShowDialogue(npc_id, npc_name, dialogue) => {
                self.show_dialogue(npc_id, npc_name, dialogue)
            }
            EngineStateUpdate::CenterCamera(x, y, offset) => {
                self.center_camera_at(*x, *y, offset)
            }
            EngineStateUpdate::Teleport(destination) => {
                self.teleport(destination)
            }
            EngineStateUpdate::SaveGame => {
                self.save()
            }
            EngineStateUpdate::Exit => {
                self.exit()
            }
            EngineStateUpdate::ShowEntityOptions(entity) => {
                self.entity_options_menu.show(entity.clone(), self.creative_mode, false)
            }
            EngineStateUpdate::ShowInventoryOptions(entity) => {
                self.entity_options_menu.show(entity.clone(), false, true)
            }
            EngineStateUpdate::AddToInventory(entity) => {
                add_to_inventory(*entity.clone())
            }
            EngineStateUpdate::RemoveFromInventory(entity_id) => {
                remove_from_inventory(*entity_id)
            }
            EngineStateUpdate::ResumeGame => {
                self.menu.close()
            }
            EngineStateUpdate::Toast(toast) => {
                self.show_toast(toast)
            }
            EngineStateUpdate::Confirmation(title, text, on_confirm) => {
                self.ask_for_confirmation(title, text, on_confirm)
            }
            EngineStateUpdate::DisplayLongText(contents) => {
                self.long_text_display.show(contents.clone())
            }
            EngineStateUpdate::DeathScreen => {
                self.death_screen.show()
            }
        }
    }
    
    fn ask_for_confirmation(&mut self, title: &str, text: &str, on_confirm: &[WorldStateUpdate]) {
        self.confirmation_dialog.show(title, text, on_confirm)
    }

    fn show_toast(&mut self, toast: &Toast) {
        self.toast.show(toast);
    }

    fn show_dialogue(&mut self, npc_id: &u32, npc_name: &str, dialogue: &Dialogue) {
        if self.dialogue_menu.is_open() {
            return
        }
        self.dialogue_menu.show(*npc_id, npc_name, dialogue);
    }    

    fn exit(&mut self) {
        println!("Got exit request!");
        self.is_running = false;
    }

    fn save(&self) {
        if self.creative_mode {
            set_value_for_key(&StorageKey::latest_world(), self.world.id);     
            self.world.save();
        }
    }

    fn teleport(&mut self, destination: &Destination) {
        self.loading_screen.animate_world_transition();

        if self.creative_mode {
            self.world.save();
        }
            
        if self.world.id != WORLD_ID_NONE {
            set_value_for_key(&StorageKey::previous_world(), self.world.id);
        }
        
        let mut new_world = World::load_or_create(destination.world);
        new_world.set_creative_mode(self.creative_mode);
        new_world.setup(
            self.previous_world(), 
            &self.world.cached_hero_props.direction, 
            destination.x, 
            destination.y
        );
        new_world.update(0.001);
        let hero_frame = new_world.cached_hero_props.frame;
        self.world = new_world;
        self.center_camera_in(&hero_frame);

        self.menu.current_world_id = self.world.id;
        self.keyboard.on_world_changed();
        self.mouse.on_world_changed();

        set_value_for_key(&StorageKey::latest_world(), self.world.id);
    }

    fn previous_world(&self) -> u32 {
        if self.world.id == WORLD_ID_NONE { 
            get_value_for_key(&StorageKey::previous_world()).unwrap_or(WORLD_ID_NONE)
        } else {
            self.world.id
        }
    }

    fn center_camera_in(&mut self, frame: &Rect) {
        self.camera_viewport.center_in(frame);
    }

    fn center_camera_at(&mut self, x: i32, y: i32, offset: &Vector2d) {
        self.camera_viewport.center_at(&Vector2d::new(x as f32, y as f32));
        self.camera_viewport_offset = *offset;
        self.world.visible_bounds = self.camera_viewport;
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
    use crate::{constants::{WORLD_ID_DEMO, WORLD_ID_NONE, WORLD_SIZE_COLUMNS, WORLD_SIZE_ROWS}, game_engine::world::World, utils::directions::Direction};

    use super::GameEngine;

    impl GameEngine {
        pub fn start_headless(&mut self) -> World {
            let mut world = World::new(WORLD_ID_DEMO);
            world.setup(
                WORLD_ID_NONE,
                &Direction::Unknown, 
                WORLD_SIZE_COLUMNS as i32 / 2, 
                WORLD_SIZE_ROWS as i32 / 2
            );
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