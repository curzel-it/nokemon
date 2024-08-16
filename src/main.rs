mod constants;
mod entities;
mod features;
mod game_engine;
mod maps;
mod utils;

use constants::SCALE;
use game_engine::{game_engine::GameEngine, rendering::draw_frame};

fn main() {
    let mut engine = GameEngine::new();
    let (mut world, mut rl, thread) = engine.start_rl();

    while !rl.window_should_close() {     
        let time_since_last_update = rl.get_frame_time();

        if rl.is_window_resized() {
            println!(
                "Resized to {}x{}, which scales to {}x{} in game", 
                rl.get_screen_width() as f32, 
                rl.get_screen_height() as f32, 
                rl.get_screen_width() as f32 / SCALE, 
                rl.get_screen_height() as f32 / SCALE
            );
            world.camera_viewport.width = rl.get_screen_width() as f32 / SCALE;
            world.camera_viewport.height = rl.get_screen_height() as f32 / SCALE;
        }

        engine.update_rl(&mut world, time_since_last_update, &rl);
        draw_frame(&mut rl, &thread, &world, &engine);  
    }
}