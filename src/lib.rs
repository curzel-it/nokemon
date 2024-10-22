pub mod constants;
pub mod dialogues;
pub mod entities;
pub mod features;
pub mod game_engine;
pub mod lang;
pub mod maps;
pub mod menus;
pub mod prefabs;
pub mod rendering;
pub mod ui;
pub mod utils;
pub mod worlds;

use serde::{Serialize, Deserialize};

#[no_mangle]
pub extern "C" fn initialize_game() {
    // Initialization code
}

#[no_mangle]
pub extern "C" fn update_game() {
    // Game update logic
}
