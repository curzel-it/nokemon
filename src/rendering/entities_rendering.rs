use std::{borrow::Borrow, cmp::Ordering};

use raylib::prelude::*;

use crate::{constants::TILE_SIZE, game_engine::{entity::Entity, game_engine::GameEngine, world::World}};

pub fn render_entities(d: &mut RaylibDrawHandle, world: &World, engine: &GameEngine) {
    let visible_entities = &world.visible_entities;
    let entities_map = world.entities.borrow();    
    
    let mut entities: Vec<&Box<dyn Entity>> = entities_map
        .iter()
        .filter_map(|(id, e)| {
            if visible_entities.contains(id) {
                Some(e)
            } else {
                None
            }
        })
        .collect();
    
    entities.sort_by(|entity_a, entity_b| {
        let a = entity_a.body();
        let b = entity_b.body();

        if a.z_index < b.z_index { return Ordering::Less; }
        if a.z_index > b.z_index { return Ordering::Greater; }
        if a.frame.y < b.frame.y { return Ordering::Less; }
        if a.frame.y > b.frame.y { return Ordering::Greater; }
        if a.frame.x < b.frame.x { return Ordering::Less; }
        if a.frame.x > b.frame.x { return Ordering::Greater; }
        Ordering::Equal
    });

    for item in entities {
        draw_item(d, item.borrow(), engine);
    }
}

fn draw_item(d: &mut RaylibDrawHandle, item: &dyn Entity, engine: &GameEngine) {
    let sprite_key = item.sprite_sheet();
    let scale = engine.rendering_scale();
    let tile_scale = TILE_SIZE * scale;
    
    if let Some(texture) = engine.ui_config.as_ref().unwrap().get_texture(sprite_key) {
        let source = item.texture_source_rect();
        let offset = item.body().offset;
        let frame = item.body().frame;

        let source_rect = Rectangle {
            x: source.x as f32 * TILE_SIZE, 
            y: source.y as f32 * TILE_SIZE,
            width: source.w as f32 * TILE_SIZE,
            height: source.h as f32 * TILE_SIZE,
        };

        let dest_rect = Rectangle {
            x: (frame.x - engine.camera_viewport.x) as f32 * tile_scale + (offset.x - engine.camera_viewport_offset.x) * scale,
            y: (frame.y - engine.camera_viewport.y) as f32 * tile_scale + (offset.y - engine.camera_viewport_offset.y) * scale,
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