use raylib::prelude::*;

use crate::game_engine::{game_engine::GameEngine, world::World};

use super::{entities_rendering::render_entities, inventory_rendering::render_inventory, tiles_rendering::render_tiles};

pub fn render(rl: &mut RaylibHandle, thread: &RaylibThread, world: &World, engine: &GameEngine) {
    let fps = rl.get_fps();
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::BLACK);
    render_tiles(&mut d, world, engine);
    render_entities(&mut d, world, engine);
    render_inventory(&mut d, engine);
    draw_debug_info(&mut d, fps);
}

fn draw_debug_info(d: &mut RaylibDrawHandle, fps: u32) {
    d.draw_text(&format!("FPS: {}", fps), 10, 10, 20, Color::RED);
    // d.draw_text(format!("Entities: {:#?}", world).as_str(), 10, 50, 20, Color::RED);
}