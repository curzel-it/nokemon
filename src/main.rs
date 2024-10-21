mod constants;
mod dialogues;
mod entities;
mod features;
mod game_engine;
mod lang;
mod maps;
mod menus;
mod prefabs;
mod rendering;
mod ui;
mod utils;
mod worlds;

use std::{collections::HashMap, env};

use common_macros::hash_map;
use constants::{ASSETS_PATH, FONT, FONT_BOLD, INITIAL_CAMERA_VIEWPORT, SPRITE_SHEET_ANIMATED_OBJECTS, SPRITE_SHEET_AVATARS, SPRITE_SHEET_BASE_ATTACK, SPRITE_SHEET_BIOME_TILES, SPRITE_SHEET_BUILDINGS, SPRITE_SHEET_CONSTRUCTION_TILES, SPRITE_SHEET_FARM_PLANTS, SPRITE_SHEET_HUMANOIDS_1X1, SPRITE_SHEET_HUMANOIDS_1X2, SPRITE_SHEET_HUMANOIDS_2X2, SPRITE_SHEET_HUMANOIDS_2X3, SPRITE_SHEET_INVENTORY, SPRITE_SHEET_MENU, SPRITE_SHEET_STATIC_OBJECTS, TILE_SIZE};
use game_engine::{engine::GameEngine, keyboard_events_provider::KeyboardEventsProvider, mouse_events_provider::MouseEventsProvider};
use raylib::{ffi::{KeyboardKey, MouseButton}, texture::Texture2D, RaylibHandle, RaylibThread};
use rendering::{ui::{get_rendering_config, init_rendering_config, RenderingConfig}, worlds::render};
use utils::vector::Vector2d;

fn main() {
    let mut creative_mode = false;

    let args: Vec<String> = env::args().collect();
    if args.contains(&"creative".to_owned()) {
        creative_mode = true;
    }

    let mut engine = GameEngine::new();
    engine.set_creative_mode(creative_mode);
    
    let (mut rl, thread) = start_rl();
    rl.set_window_min_size(360, 240);
    engine.start(rl.get_screen_width(), rl.get_screen_height());
    
    while engine.is_running {
        let time_since_last_update = rl.get_frame_time().min(0.1);

        if rl.is_window_resized() {
            println!("Window resized to {}x{}", rl.get_screen_width(), rl.get_screen_height());
            engine.window_size_changed(rl.get_screen_width(), rl.get_screen_height());
        }
        if rl.window_should_close() && !rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
            engine.is_running = false;
        }

        update_keyboard(&mut rl, &mut engine.keyboard, time_since_last_update);
        update_mouse(&mut rl, &mut engine.mouse, get_rendering_config().rendering_scale);
        engine.update(time_since_last_update);
        render(&mut rl, &thread, &engine.world, &engine);  
    }
}

fn start_rl() -> (RaylibHandle, RaylibThread) {
    let width = (TILE_SIZE * INITIAL_CAMERA_VIEWPORT.w as f32) as i32;
    let height = (TILE_SIZE * INITIAL_CAMERA_VIEWPORT.h as f32) as i32;

    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .resizable()
        .title("Nokemon")
        .build();        

    let font = rl.load_font(&thread, FONT).unwrap();
    let font_bold = rl.load_font(&thread, FONT_BOLD).unwrap();            

    // rl.set_target_fps(60.0);    

    let textures: HashMap<u32, Texture2D> = load_textures(&mut rl, &thread);
    init_rendering_config(RenderingConfig {
        font,
        font_bold,
        textures,
        rendering_scale: 2.0,
        font_rendering_scale: 2.0,
        canvas_size: Vector2d::new(1.0, 1.0)
    });

    (rl, thread)
}

fn load_textures(rl: &mut RaylibHandle, thread: &RaylibThread) -> HashMap<u32, Texture2D> {    
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
    textures.insert(SPRITE_SHEET_HUMANOIDS_2X3, texture(rl, thread, "humanoids_2x3").unwrap());
    textures.insert(SPRITE_SHEET_AVATARS, texture(rl, thread, "avatars").unwrap());     
    textures.insert(SPRITE_SHEET_FARM_PLANTS, texture(rl, thread, "farm_plants").unwrap());             
    textures
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

fn update_mouse(rl: &mut RaylibHandle, mouse: &mut MouseEventsProvider, rendering_scale: f32) {
    mouse.update(
        rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT), 
        rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT), 
        rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT), 
        rl.get_mouse_position().x,
        rl.get_mouse_position().y, 
        rendering_scale
    );
}

fn update_keyboard(rl: &mut RaylibHandle, keyboard: &mut KeyboardEventsProvider, time_since_last_update: f32) {
    let is_up_pressed = rl.is_key_pressed(KeyboardKey::KEY_W) || rl.is_key_pressed(KeyboardKey::KEY_UP);
    let is_up_down = rl.is_key_down(KeyboardKey::KEY_W) || rl.is_key_down(KeyboardKey::KEY_UP);
    let is_right_pressed = rl.is_key_pressed(KeyboardKey::KEY_D) || rl.is_key_pressed(KeyboardKey::KEY_RIGHT);
    let is_right_down = rl.is_key_down(KeyboardKey::KEY_D) || rl.is_key_down(KeyboardKey::KEY_RIGHT);
    let is_down_pressed = rl.is_key_pressed(KeyboardKey::KEY_S) || rl.is_key_pressed(KeyboardKey::KEY_DOWN);
    let is_down_down = rl.is_key_down(KeyboardKey::KEY_S) || rl.is_key_down(KeyboardKey::KEY_DOWN);
    let is_left_pressed = rl.is_key_pressed(KeyboardKey::KEY_A) || rl.is_key_pressed(KeyboardKey::KEY_LEFT);
    let is_left_down = rl.is_key_down(KeyboardKey::KEY_A) || rl.is_key_down(KeyboardKey::KEY_LEFT);

    keyboard.discard_direction_events_until_next_arrow_key_is_pressed = 
    keyboard.discard_direction_events_until_next_arrow_key_is_pressed &&
        !is_up_pressed &&
        !is_right_pressed &&
        !is_down_pressed &&
        !is_left_pressed;

    keyboard.has_back_been_pressed = rl.is_key_pressed(KeyboardKey::KEY_ESCAPE);
    keyboard.has_menu_been_pressed = rl.is_key_pressed(KeyboardKey::KEY_ENTER);
    keyboard.has_confirmation_been_pressed = rl.is_key_pressed(KeyboardKey::KEY_E);
    keyboard.has_attack_key_been_pressed = rl.is_key_pressed(KeyboardKey::KEY_F);
    keyboard.has_backspace_been_pressed = rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE);

    keyboard.direction_up.update(is_up_pressed, is_up_down, time_since_last_update);
    keyboard.direction_right.update(is_right_pressed, is_right_down, time_since_last_update);
    keyboard.direction_down.update(is_down_pressed, is_down_down, time_since_last_update);
    keyboard.direction_left.update(is_left_pressed, is_left_down, time_since_last_update);

    keyboard.currently_pressed_character = rl.get_char_pressed();
}
