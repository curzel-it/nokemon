use raylib::prelude::*;

use crate::{game_engine::engine::GameEngine, ui::components::{render_from, AnchorPoint}, utils::vector::Vector2d};

pub fn render_death_screen(d: &mut RaylibDrawHandle, engine: &GameEngine) {
    if !engine.death_screen.is_open {
        return
    }

    let ui_config = engine.ui_config.as_ref().unwrap();

    d.draw_rectangle(
        0, 
        0, 
        d.get_screen_width(), 
        d.get_screen_height(), 
        Color::BLACK.alpha(0.6)
    );
    
    render_from(
        AnchorPoint::Center, 
        &engine.death_screen.ui(), 
        d, 
        ui_config, 
        &Vector2d::new(
            ui_config.canvas_size.x / 2.0,
            ui_config.canvas_size.y / 2.0
        )
    )
}
