use std::cmp::Ordering;

use game_core::{constants::TILE_SIZE, game_engine::{engine::GameEngine, entity::Entity, world::World}};
use raylib::prelude::*;

use super::ui::get_rendering_config;

pub fn render_entities(d: &mut RaylibDrawHandle, world: &World, engine: &GameEngine) {
    let visible_entities = &world.visible_entities;
    let entities_map = world.entities.borrow();    
    
    let mut entities: Vec<&Entity> = visible_entities.iter()
        .map(|(index, _)| &entities_map[*index])
        .collect();
    
    entities.sort_by(|entity_a, entity_b| {
        let a = entity_a;
        let b = entity_b;

        let ay = a.frame.y + if a.frame.h > 1 { 1 } else { 0 };
        let by = b.frame.y + if b.frame.h > 1 { 1 } else { 0 };

        if a.z_index < b.z_index && a.z_index < 0 { return Ordering::Less; }
        if a.z_index > b.z_index && b.z_index < 0 { return Ordering::Greater; }
        if ay < by { return Ordering::Less; }
        if ay > by { return Ordering::Greater; }
        if a.z_index < b.z_index { return Ordering::Less; }
        if a.z_index > b.z_index { return Ordering::Greater; }
        if a.frame.x < b.frame.x { return Ordering::Less; }
        if a.frame.x > b.frame.x { return Ordering::Greater; }
        Ordering::Equal
    });

    for item in entities {
        draw_item(d, item, engine);
    }
}

fn draw_item(d: &mut RaylibDrawHandle, item: &Entity, engine: &GameEngine) {
    let sprite_key = item.sprite_sheet();
    let scale = get_rendering_config().rendering_scale;
    let tile_scale = TILE_SIZE * scale;
    
    if let Some(texture) = get_rendering_config().get_texture(sprite_key) {
        let source = item.texture_source_rect();
        let offset = item.offset; // rendering_position_offset();
        let frame = item.frame; // rendering_rect();

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