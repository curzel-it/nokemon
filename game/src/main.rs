mod constants;
mod entities;
mod features;
mod game_engine;
mod maps;
mod utils;

use game_engine::{game_engine::GameEngine, rendering::draw_frame};

fn main() {
    let mut engine = GameEngine::new();
    let (mut world, mut rl, thread) = engine.start_rl();

    while !rl.window_should_close() {     
        let time_since_last_update = rl.get_frame_time();

        if rl.is_window_resized() {
            world.adjust_camera_from_screen_size(rl.get_screen_width(), rl.get_screen_height());
        }

        engine.update_rl(&mut world, time_since_last_update, &rl);
        draw_frame(&mut rl, &thread, &world, &engine);  
    }
}