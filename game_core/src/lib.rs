use game_engine::engine::GameEngine;

pub mod constants;
pub mod dialogues;
pub mod entities;
pub mod features;
pub mod game_engine;
pub mod lang;
pub mod maps;
pub mod menus;
pub mod prefabs;
pub mod ui;
pub mod utils;
pub mod worlds;

static mut ENGINE: *mut GameEngine = std::ptr::null_mut();

pub fn engine() -> &'static GameEngine {
    unsafe {
        &*ENGINE
    }
}

fn engine_mut() -> &'static mut GameEngine {
    unsafe {
        &mut *ENGINE
    }
}

#[no_mangle]
pub extern "C" fn test_integration() {
    println!("Helloooo");
}

#[no_mangle]
pub extern "C" fn initialize_game(creative_mode: bool) {
    unsafe {
        let boxed = Box::new(GameEngine::new());
        ENGINE = Box::into_raw(boxed);      
    }
    let engine = engine_mut();
    engine.set_creative_mode(creative_mode);
    engine.start();
}

#[no_mangle]
pub extern "C" fn is_creative_mode() -> bool {
    engine().creative_mode
}

#[no_mangle]
pub extern "C" fn is_game_running() -> bool {
    engine().is_running
}

#[no_mangle]
pub extern "C" fn stop_game() {
    engine_mut().is_running = false
}

#[no_mangle]
pub extern "C" fn window_size_changed(width: f32, height: f32, scale: f32, font_size: f32, line_spacing: f32) {
    engine_mut().window_size_changed(width, height, scale, font_size, line_spacing)
}

#[no_mangle]
pub extern "C" fn update_game(time_since_last_update: f32) {
    engine_mut().update(time_since_last_update)
}

#[no_mangle]
pub extern "C" fn update_keyboard(
    up_pressed: bool,
    right_pressed: bool,
    down_pressed: bool,
    left_pressed: bool,
    up_down: bool,
    right_down: bool,
    down_down: bool,
    left_down: bool,
    escape_pressed: bool,
    menu_pressed: bool,
    confirm_pressed: bool,
    attack_pressed: bool,
    backspace_pressed: bool,
    current_char: u32,
    time_since_last_update: f32
) {
    engine_mut().keyboard.update(
        up_pressed, right_pressed, down_pressed, left_pressed, 
        up_down, right_down, down_down, left_down, 
        escape_pressed, menu_pressed, confirm_pressed, attack_pressed, backspace_pressed, 
        if current_char == 0 { None } else { char::from_u32(current_char) }, 
        time_since_last_update
    );
}

#[no_mangle]
pub extern "C" fn update_mouse(
    mouse_left_down: bool, 
    mouse_left_pressed: bool, 
    mouse_right_pressed: bool, 
    mouse_x: f32,
    mouse_y: f32,
    rendering_scale: f32
) {
    engine_mut().mouse.update(
        mouse_left_down, 
        mouse_left_pressed, mouse_right_pressed, 
        mouse_x, mouse_y, 
        rendering_scale
    );
}