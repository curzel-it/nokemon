use raylib::prelude::*;

use crate::{constants::TILE_SIZE, features::inventory::Stockable, game_engine::game_engine::GameEngine, utils::geometry_utils::Scalable};

pub fn render_inventory(d: &mut RaylibDrawHandle, engine: &GameEngine) {
    if engine.inventory.is_open {
        let sprite_path = engine.inventory.sprite_sheet_path();

        for (index, item) in engine.inventory.visible_items().iter().enumerate() {
            draw_item(d, sprite_path, item, index, engine);
        }
    }
}

fn draw_item(
    d: &mut RaylibDrawHandle, 
    sprite_path: &str,
    item: &Stockable,
    index: usize,
    engine: &GameEngine
) {
    let source_rect = item.texture_source_rect();
    
    if let Some(texture) = engine.textures.get(sprite_path) {
        let dest_rect = Rectangle {
            x: index as f32 * TILE_SIZE + 100.0, 
            y: (engine.camera_viewport.height - TILE_SIZE * 2.0),
            width: TILE_SIZE,
            height: TILE_SIZE,
        }.scaled(engine.rendering_scale);

        d.draw_texture_pro(
            texture,
            source_rect,
            dest_rect,
            Vector2::zero(), 
            0.0,
            Color::WHITE,
        );
    }
}