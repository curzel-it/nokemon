mod constants;
mod entities;
mod features;
mod game_engine;
mod levels;
mod maps;
mod rendering;
mod ui;
mod utils;

use std::{env, fs::File, io::Write};

use game_engine::game_engine::GameEngine;
use rendering::levels_rendering::render;

fn main() {
    let save_file_path = "save_game.json";    
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

    if let Ok(serialized_world) = serde_json::to_string(&engine.current_world()) {
        if let Ok(mut file) = File::create(save_file_path) {
            if let Err(e) = file.write_all(serialized_world.as_bytes()) {
                eprintln!("Failed to write save file: {}", e);
            } else {
                println!("Game saved successfully to {}", save_file_path);
            }
        } else {
            eprintln!("Failed to create save file");
        }
    } else {
        eprintln!("Failed to serialize game world");
    }
}