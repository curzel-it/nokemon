use raylib::prelude::*;

use crate::features::loading_screen::LoadingScreen;

pub fn render_loading_screen(d: &mut RaylibDrawHandle, loading_screen: &LoadingScreen) {
    let progress = loading_screen.progress();
    let alpha = if progress <= 0.5 { progress * 3.0 } else { 1.0 - (progress - 0.5) * 2.0 };

    d.draw_rectangle(
        0, 
        0, 
        d.get_screen_width(), 
        d.get_screen_height(), 
        Color::BLACK.alpha(alpha)
    );
    d.draw_text(&loading_screen.text, 10, 10, 20, Color::WHITE);
}
