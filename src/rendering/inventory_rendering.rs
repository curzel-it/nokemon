use raylib::prelude::*;

use crate::{game_engine::game_engine::GameEngine, ui::ui::render};

pub fn render_inventory(d: &mut RaylibDrawHandle, engine: &GameEngine) {
    if engine.inventory.is_open {
        render(
            // showcase_view(),
            engine.inventory.ui(),
            d, 
            &engine.ui_config.as_ref().unwrap(), 
            &Vector2::zero()
        );
    }
}