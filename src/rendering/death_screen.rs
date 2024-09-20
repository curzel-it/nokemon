use common_macros::hash_map;
use raylib::prelude::*;

use crate::{game_engine::engine::GameEngine, ui::layouts::{AnchorPoint, Layout}};

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
    
    Layout::new(
        d.get_screen_width(),
        d.get_screen_height(),
        hash_map! {
            AnchorPoint::Center => vec![engine.death_screen.ui()]
        }
    ).render(d, ui_config)
}
