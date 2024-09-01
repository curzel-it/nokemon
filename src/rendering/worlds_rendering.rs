use raylib::prelude::*;

use crate::game_engine::{engine::GameEngine, world::World};

use super::{entities_rendering::render_entities, menu_rendering::render_menu, tiles_rendering::render_tiles, loading_screen_rendering::render_loading_screen};

pub fn render(rl: &mut RaylibHandle, thread: &RaylibThread, world: &World, engine: &GameEngine) {
    let fps = rl.get_fps();
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::BLACK);
    
    if !engine.loading_screen.is_in_progress() || engine.loading_screen.progress() > 0.4 {
        render_tiles(&mut d, world, engine);
        render_entities(&mut d, world, engine);
    }
    render_menu(&mut d, engine);
    render_loading_screen(&mut d, &engine.loading_screen);
    draw_debug_info(&mut d, fps);
}

fn draw_debug_info(d: &mut RaylibDrawHandle, fps: u32) {
    d.draw_text(&format!("FPS: {}", fps), 10, 10, 20, Color::RED);
    // d.draw_text(format!("Entities: {:#?}", world).as_str(), 10, 50, 20, Color::RED);
}