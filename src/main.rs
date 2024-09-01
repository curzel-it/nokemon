mod constants;
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

use std::env;

use game_engine::game_engine::GameEngine;
use rendering::worlds_rendering::render;

fn main() {
    let mut creative_mode = false;

    let args: Vec<String> = env::args().collect();
    if args.contains(&"creative".to_owned()) {
        creative_mode = true;
    }

    let mut engine = GameEngine::new();
    engine.set_creative_mode(creative_mode);
    
    let (mut rl, thread) = engine.start_rl();
    
    while engine.is_running {
        let time_since_last_update = rl.get_frame_time();

        if rl.is_window_resized() {
            engine.window_size_changed(rl.get_screen_width(), rl.get_screen_height());
        }
        if rl.window_should_close() && !rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
            engine.is_running = false;
        }

        engine.update_rl(&rl, time_since_last_update);
        render(&mut rl, &thread, &engine.world, &engine);  
    }
}