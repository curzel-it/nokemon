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

use game_engine::engine::GameEngine;
use std::cell::RefCell;

thread_local! {
    static ENGINE: RefCell<Option<GameEngine>> = RefCell::new(None);
}

#[no_mangle]
pub extern "C" fn initialize_game(width: i32, height: i32, creative_mode: bool) {
    ENGINE.with(|engine| {
        *engine.borrow_mut() = Some(GameEngine::new());
        if let Some(ref mut eng) = *engine.borrow_mut() {
            eng.set_creative_mode(creative_mode);
            eng.start();
        }
    });
}

#[no_mangle]
pub extern "C" fn update_game(time_since_last_update: f32) {
    ENGINE.with(|engine| {
        if let Some(ref mut eng) = *engine.borrow_mut() {
            eng.update(time_since_last_update);
        }
    });
}

#[no_mangle]
pub extern "C" fn is_game_running() -> bool {
    ENGINE.with(|engine| {
        if let Some(ref eng) = *engine.borrow() {
            eng.is_running
        } else {
            false
        }
    })
}