use common_macros::hash_map;
use game_core::{game_engine::engine::GameEngine, ui::layouts::{AnchorPoint, Layout}};
use raylib::prelude::*;

use super::ui::render_layout;

pub fn render_death_screen(d: &mut RaylibDrawHandle, engine: &GameEngine) {
    if !engine.death_screen.is_open {
        return
    }

    d.draw_rectangle(
        0, 
        0, 
        d.get_screen_width(), 
        d.get_screen_height(), 
        Color::BLACK.alpha(0.6)
    );
    
    let layout = Layout::new(
        d.get_screen_width(),
        d.get_screen_height(),
        hash_map! {
            AnchorPoint::Center => vec![engine.death_screen.ui()]
        }
    );
    render_layout(&layout, d);
}
