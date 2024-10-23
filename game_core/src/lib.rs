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

pub fn engine_mut() -> &'static mut GameEngine {
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
