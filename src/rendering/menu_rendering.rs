use raylib::prelude::*;

use crate::{game_engine::game_engine::GameEngine, ui::ui::{render_from, AnchorPoint}, utils::vector::Vector2d};

pub fn render_menu(d: &mut RaylibDrawHandle, engine: &GameEngine) {
    let ui_config = engine.ui_config.as_ref().unwrap();

    render_from(
        AnchorPoint::BottomCenter,
        engine.menu.ui(&engine.camera_viewport_offset),
        d, 
        ui_config, 
        &Vector2d::new(
            ui_config.canvas_size.x / 2.0, 
            ui_config.canvas_size.y
        )
    );
}