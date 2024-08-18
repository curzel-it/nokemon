use raylib::prelude::*;

use crate::game_engine::{game_engine::GameEngine, world::World};

pub fn render_inventory(d: &mut RaylibDrawHandle, world: &World, _: &GameEngine) {
    let inventory = 0;
    d.draw_text(&format!("Inventory: {:#?}", inventory), 10, 100, 20, Color::RED);
}