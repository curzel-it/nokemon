mod constants;
mod entities;
mod features;
mod game_engine;
mod sprites;
mod utils;

use game_engine::{game_engine::GameEngine, rendering::draw_frame};

fn main() {
    let mut engine = GameEngine::new();
    let (mut game, mut rl, thread) = engine.start_rl();

    while !rl.window_should_close() {     
        let time_since_last_update = rl.get_frame_time();

        if rl.is_window_resized() {
            game.camera_viewport.width = rl.get_screen_width() as f32;
            game.camera_viewport.height = rl.get_screen_height() as f32;
        }

        engine.update_rl(&mut game, time_since_last_update, &rl);
        draw_frame(&mut rl, &thread, &game, &engine);  
    }
}