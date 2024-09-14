use raylib::prelude::*;

use crate::{game_engine::engine::GameEngine, ui::components::{render_from, AnchorPoint, RenderingConfig, View}, utils::vector::Vector2d};

pub fn render_menu(d: &mut RaylibDrawHandle, engine: &GameEngine) {
    let ui_config = engine.ui_config.as_ref().unwrap();
    render_centered_menu(d, ui_config, &engine.menu.ui(&engine.camera_viewport_offset));
    render_centered_menu(d, ui_config, &engine.entity_options_menu.ui());
    render_centered_menu(d, ui_config, &engine.dialogue_menu.ui());
    render_centered_menu(d, ui_config, &engine.confirmation_dialog.ui());
    render_centered_menu(d, ui_config, &engine.long_text_display.ui());
    render_top_right_menu(d, ui_config, &engine.toast.ui());
}

fn render_top_right_menu(d: &mut RaylibDrawHandle, ui_config: &RenderingConfig, view: &View) {
    render_from(
        AnchorPoint::TopRight,
        view,
        d, 
        ui_config, 
        &Vector2d::new(
            ui_config.canvas_size.x, 
            0.0
        )
    );
}

fn render_centered_menu(d: &mut RaylibDrawHandle, ui_config: &RenderingConfig, view: &View) {
    render_from(
        AnchorPoint::BottomCenter,
        view,
        d, 
        ui_config, 
        &Vector2d::new(
            ui_config.canvas_size.x / 2.0, 
            ui_config.canvas_size.y
        )
    );
}