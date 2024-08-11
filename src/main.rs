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
   
        engine.update_rl(&mut game, time_since_last_update, &rl);
        draw_frame(&mut rl, &thread, &game, &engine);  
    }
}