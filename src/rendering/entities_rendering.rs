use std::{borrow::Borrow, cmp::Ordering};

use raylib::prelude::*;

use crate::{game_engine::{entity::Entity, game_engine::GameEngine, world::World}, utils::rect::Rect};

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
    let sprite_path = item.sprite_sheet();
    let frame = item.body().frame;
    
    if let Some(texture) = engine.ui_config.as_ref().unwrap().get_texture(sprite_path) {
        let source_rect = item.texture_source_rect().as_rr();

        let dest_rect = Rect {
            x: frame.x - engine.camera_viewport.x, 
            y: frame.y - engine.camera_viewport.y,
            w: frame.w,
            h: frame.h,
        }
        .scaled(engine.ui_config.as_ref().unwrap().rendering_scale)
        .as_rr();

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