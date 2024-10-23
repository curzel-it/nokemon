use game_core::{constants::TILE_SIZE, game_engine::engine::GameEngine, renderables_vec, RenderableItem};
use raylib::prelude::*;

use super::ui::get_rendering_config;

pub fn render_entities(d: &mut RaylibDrawHandle, engine: &GameEngine) {
    for item in &renderables_vec() {
        draw_item(d, item, engine);
    }
}

fn draw_item(d: &mut RaylibDrawHandle, item: &RenderableItem, engine: &GameEngine) {
    let sprite_key = item.sprite_sheet_id;
    let scale = get_rendering_config().rendering_scale;
    let tile_scale = TILE_SIZE * scale;
    
    if let Some(texture) = get_rendering_config().get_texture(sprite_key) {
        let source = item.texture_rect;
        let offset = item.offset;
        let frame = item.frame;

        let source_rect = Rectangle {
            x: source.x as f32 * TILE_SIZE, 
            y: source.y as f32 * TILE_SIZE,
            width: source.w as f32 * TILE_SIZE,
            height: source.h as f32 * TILE_SIZE,
        };

        let actual_col = frame.x as f32 - engine.camera_viewport.x as f32;
        let actual_offset_x = offset.x - engine.camera_viewport_offset.x;

        let actual_row = frame.y as f32 - engine.camera_viewport.y as f32;
        let actual_offset_y = offset.y - engine.camera_viewport_offset.y;

        let dest_rect = Rectangle {
            x: actual_col * tile_scale + actual_offset_x * scale,
            y: actual_row * tile_scale + actual_offset_y * scale,
            width: frame.w as f32 * tile_scale,
            height: frame.h as f32 * tile_scale,
        };

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