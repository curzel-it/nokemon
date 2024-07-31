mod constants;
mod game_behaviors;
mod game_engine;
mod features;
mod species;
mod sprites;
mod utils;

use constants::{FPS, LOG_GAME_STATE};
use game_engine::{game_engine::GameEngine, rendering::draw_frame};

fn main() {
    let mut engine = GameEngine::new();
    let (mut game, mut rl, thread) = engine.start(800, 600);
    let mut frames_counter = 0;

    while !rl.window_should_close() {     
        let time_since_last_update = rl.get_frame_time();
   
        engine.handle_inputs(&mut game, &rl);
        engine.update(&mut game, time_since_last_update);
        draw_frame(&mut rl, &thread, &game, &engine);

        frames_counter += 1;

        if LOG_GAME_STATE {
            println!("#{} {:#?}", frames_counter, game);
            // d.draw_text(format!("{:#?}", game).as_str(), 10, 10, 18, Color::WHITE);
        }
    }
}