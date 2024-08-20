mod constants;
mod design_system;
mod entities;
mod features;
mod game_engine;
mod levels;
mod maps;
mod rendering;
mod utils;

use std::env;

use game_engine::game_engine::GameEngine;
use maps::worldgen::create_map_binaries;
use rendering::levels_rendering::render;

fn main() {
    let mut creative_mode = false;

    let args: Vec<String> = env::args().collect();
    if args.contains(&"worldgen".to_owned()) {
        println!("Running world gen...");
        create_map_binaries();
    }
    if args.contains(&"creative".to_owned()) {
        println!("Running in creative mode...");
        creative_mode = true;
    }

    let mut engine = GameEngine::with_options(creative_mode);
    let (mut rl, thread) = engine.start_rl();

    while !rl.window_should_close() {     
        let time_since_last_update = rl.get_frame_time();

        if rl.is_window_resized() {
            engine.adjust_camera_from_screen_size(rl.get_screen_width(), rl.get_screen_height());
        }

        engine.update_rl(time_since_last_update, &rl);
        render(&mut rl, &thread, engine.current_world(), &engine);  
    }
}