use raylib::prelude::*;

use crate::{constants::TILE_SIZE, features::inventory::InventoryItem, game_engine::game_engine::GameEngine, utils::geometry_utils::Scalable};

pub fn render_inventory(d: &mut RaylibDrawHandle, engine: &GameEngine) {
    if engine.inventory.is_open {
        let sprite_path = engine.inventory.sprite_sheet_path();

        for (index, entry) in engine.inventory.stock.iter().enumerate() {
            draw_item(d, sprite_path, entry, index == engine.inventory.selected_index, index, engine);
        }
    }
}

fn draw_item(
    d: &mut RaylibDrawHandle, 
    sprite_path: &str,
    entry: &InventoryItem,
    is_selected: bool,
    index: usize,
    engine: &GameEngine
) {
    let source_rect = entry.item.texture_source_rect();
    
    if let Some(texture) = engine.textures.get(sprite_path) {
        let dest_rect = Rectangle {
            x: index as f32 * (2.0 + TILE_SIZE) + 100.0, 
            y: (engine.camera_viewport.height - TILE_SIZE * 2.0),
            width: TILE_SIZE,
            height: TILE_SIZE,
        }.scaled(engine.rendering_scale);

        d.draw_rectangle(
            dest_rect.x as i32 - 4, 
            dest_rect.y as i32 - 4, 
            dest_rect.width as i32 + 8, 
            dest_rect.height as i32 + 8, 
            item_background_color(is_selected)
        );

        d.draw_texture_pro(
            texture,
            source_rect,
            dest_rect,
            Vector2::zero(), 
            0.0,
            Color::WHITE,
        );

        d.draw_text(format!("{}", entry.stock).as_str(), dest_rect.x as i32, dest_rect.y as i32, 12, Color::PINK);
    }
}

fn item_background_color(is_selected: bool) -> Color {
    if is_selected {
        Color::YELLOW
    } else {
        Color::BLACK
    }
}