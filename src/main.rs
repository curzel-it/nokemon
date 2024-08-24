mod constants;
mod entities;
mod features;
mod game_engine;
mod worlds;
mod maps;
mod rendering;
mod ui;
mod utils;

use std::env;

use game_engine::game_engine::GameEngine;
use rendering::worlds_rendering::render;

fn main() {
    let mut creative_mode = false;

    let args: Vec<String> = env::args().collect();
    if true || args.contains(&"creative".to_owned()) {
        println!("Running in creative mode...");
        creative_mode = true;
    }

    let mut engine = GameEngine::with_options(creative_mode);
    let (mut rl, thread) = engine.start_rl();

    while !(rl.window_should_close() && !rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE)) {     
        let time_since_last_update = rl.get_frame_time();

        if rl.is_window_resized() {
            engine.window_size_changed(rl.get_screen_width(), rl.get_screen_height());
        }

        engine.update_rl(time_since_last_update, &rl);
        render(&mut rl, &thread, engine.current_world(), &engine);  
    }

    engine.current_world().save();
}