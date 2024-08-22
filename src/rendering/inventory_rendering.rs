use raylib::prelude::*;

use crate::{features::inventory::InventoryItemBeingPlaced, game_engine::game_engine::GameEngine, ui::ui::{render_from, Corner}, utils::geometry_utils::Scalable};

pub fn render_inventory(d: &mut RaylibDrawHandle, engine: &GameEngine) {
    let ui_config = engine.ui_config.as_ref().unwrap();

    if !engine.inventory.is_open {
        return 
    }
    if !engine.inventory.is_placing_item {
        render_from(
            Corner::TopRight,
            engine.inventory.ui(),
            d, 
            &ui_config, 
            &Vector2::new(ui_config.canvas_size.x, 0.0)
        );
    }
    if let Some(item_being_placed) = engine.inventory.item_being_placed {
        draw_placement_indicator(d, item_being_placed, engine);
    }
}

fn draw_placement_indicator(
    d: &mut RaylibDrawHandle, 
    item: InventoryItemBeingPlaced,
    engine: &GameEngine
) {
    let dest_rect = item.frame.scaled(engine.ui_config.as_ref().unwrap().rendering_scale);

    d.draw_rectangle(
        dest_rect.x as i32, 
        dest_rect.y as i32, 
        dest_rect.width as i32, 
        dest_rect.height as i32, 
        Color::RED
    );
}
