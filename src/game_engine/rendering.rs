
use std::{borrow::Borrow, cmp::Ordering};

use raylib::prelude::*;

use crate::{constants::{SCALE, TILE_SIZE}, maps::tiles::SpriteTile};

use super::{entity::Entity, world::World, game_engine::GameEngine};

pub fn draw_frame(rl: &mut RaylibHandle, thread: &RaylibThread, world: &World, engine: &GameEngine) {
    let fps = rl.get_fps();
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::BLACK);
    draw_tiles(&mut d, world, engine);
    draw_entities(&mut d, world, engine);
    draw_debug_info(&mut d, world, fps);
}

fn draw_debug_info(d: &mut RaylibDrawHandle, _: &World, fps: u32) {
    d.draw_text(&format!("FPS: {}", fps), 10, 10, 20, Color::RED);
    // d.draw_text(format!("Entities: {:#?}", world).as_str(), 10, 50, 20, Color::RED);
}

fn draw_tiles(d: &mut RaylibDrawHandle, world: &World, engine: &GameEngine) {    
    for tile in world.visible_tiles() {
        let variant = world.tiles.current_variant(tile.row, tile.column);
        draw_tile(d, tile, variant, &world.camera_viewport, engine);
    }
}

fn draw_entities(d: &mut RaylibDrawHandle, world: &World, engine: &GameEngine) {
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
        draw_item(d, item.borrow(), &world.camera_viewport, engine);
    }
}

fn draw_item(
    d: &mut RaylibDrawHandle, 
    item: &dyn Entity,
    camera_viewport: &Rectangle,
    engine: &GameEngine
) {
    let sprite_path = item.body().current_sprite_frame();
    let frame = item.body().frame;
    let position = Vector2::new(frame.x - camera_viewport.x, frame.y - camera_viewport.y);
    
    if let Some(texture) = engine.textures.get(sprite_path) {
        let source_rect = Rectangle {
            x: 0.0,
            y: 0.0,
            width: texture.width as f32,
            height: texture.height as f32,
        };

        let dest_rect = Rectangle {
            x: position.x,
            y: position.y,
            width: frame.width,
            height: frame.height,
        };

        d.draw_texture_pro(
            texture,
            source_rect,
            dest_rect,
            Vector2::new(0.0, 0.0), 
            0.0,
            Color::WHITE,
        );
    }
}

fn draw_tile<T: SpriteTile>(
    d: &mut RaylibDrawHandle, 
    tile: &T,
    variant: u32,
    camera_viewport: &Rectangle,
    engine: &GameEngine
) {
    let sprite_path = tile.sprite_name();  
    let source_rect = tile.sprite_source_rect(variant);
    
    if let Some(texture) = engine.textures.get(&sprite_path) {
        let dest_rect = Rectangle {
            x: tile.column() as f32 * TILE_SIZE - camera_viewport.x, 
            y: tile.row() as f32 * TILE_SIZE - camera_viewport.y,
            width: TILE_SIZE,
            height: TILE_SIZE,
        };

        d.draw_texture_pro(
            texture,
            source_rect,
            dest_rect,
            Vector2::new(0.0, 0.0), 
            0.0,
            Color::WHITE,
        );
    }
    /* 
    let sprite_path = tile.sprite_name(variant);    
    let position = Vector2::new(
        tile.column() as f32 * TILE_SIZE - camera_viewport.x, 
        tile.row() as f32 * TILE_SIZE - camera_viewport.y
    );
    
    if let Some(texture) = engine.textures.get(&sprite_path) {
        d.draw_texture_ex(
            texture,
            position,
            0.0,
            SCALE,
            Color::WHITE 
        );
    }*/
}