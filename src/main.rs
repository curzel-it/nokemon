mod constants;
mod entities;
mod features;
mod game_engine;
mod levels;
mod maps;
mod utils;

use std::env;

use game_engine::{game_engine::GameEngine, rendering::draw_frame};
use maps::worldgen::create_map_binaries;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"worldgen".to_owned()) {
        println!("Running world gen...");
        create_map_binaries();
        return;
    }

    let mut engine = GameEngine::new();
    let (mut rl, thread) = engine.start_rl();

    while !rl.window_should_close() {     
        let time_since_last_update = rl.get_frame_time();

        if rl.is_window_resized() {
            engine.adjust_camera_from_screen_size(rl.get_screen_width(), rl.get_screen_height());
        }

        engine.update_rl(time_since_last_update, &rl);
        draw_frame(&mut rl, &thread, &engine.current_world(), &engine);  
    }
}