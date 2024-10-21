use common_macros::hash_map;
use raylib::prelude::*;

use crate::{game_engine::engine::GameEngine, ui::layouts::{AnchorPoint, Layout}};

pub fn render_menu(d: &mut RaylibDrawHandle, engine: &GameEngine) {
    let w = d.get_screen_width();
    let h = d.get_screen_height();

    Layout::new(w, h, hash_map! {
        AnchorPoint::BottomCenter => vec![
            engine.menu.ui(&engine.camera_viewport),
            engine.entity_options_menu.ui(),
            engine.dialogue_menu.ui(),
            engine.confirmation_dialog.ui(),
            engine.long_text_display.ui(),
        ],
        AnchorPoint::TopRight => vec![
            engine.toast.regular_toast_ui()
        ],
        AnchorPoint::TopLeft => vec![
            engine.toast.important_toast_ui()
        ]
    }).render(d);
}