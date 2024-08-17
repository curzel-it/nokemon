
use std::{borrow::Borrow, cmp::Ordering};

use raylib::prelude::*;

use crate::{constants::TILE_SIZE, maps::tiles::SpriteTile, utils::geometry_utils::Scalable};

use super::{entity::Entity, world::World, game_engine::GameEngine};

pub fn draw_frame(rl: &mut RaylibHandle, thread: &RaylibThread, world: &World, engine: &GameEngine) {
    let fps = rl.get_fps();
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::new(0, 105, 170, 255));
    draw_biome(&mut d, world, engine);
    draw_constructions(&mut d, world, engine);
    draw_entities(&mut d, world, engine);
    draw_debug_info(&mut d, world, fps);
}

fn draw_debug_info(d: &mut RaylibDrawHandle, _: &World, fps: u32) {
    d.draw_text(&format!("FPS: {}", fps), 10, 10, 20, Color::RED);
    // d.draw_text(format!("Entities: {:#?}", world).as_str(), 10, 50, 20, Color::RED);
}

fn draw_biome(d: &mut RaylibDrawHandle, world: &World, engine: &GameEngine) {    
    let sprite_path = world.biome_tiles.sheet_path.as_str();

    for tile in world.visible_biome_tiles(&engine.camera_viewport) {
        draw_tile(
            d, 
            sprite_path, 
            tile, 
            world.biome_tiles.current_variant(tile.row, tile.column), 
            engine
        );
    }
}

fn draw_constructions(d: &mut RaylibDrawHandle, world: &World, engine: &GameEngine) {    
    let sprite_path = world.constructions_tiles.sheet_path.as_str();

    for tile in world.visible_construction_tiles(&engine.camera_viewport) {
        draw_tile(
            d, 
            sprite_path, 
            tile, 
            0, 
            engine
        );
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
        draw_item(d, item.borrow(), engine);
    }
}

fn draw_item(d: &mut RaylibDrawHandle, item: &dyn Entity, engine: &GameEngine) {
    let sprite_path = item.sprite_sheet_path();
    let frame = item.body().frame;
    
    if let Some(texture) = engine.textures.get(sprite_path) {
        let source_rect = item.texture_source_rect();

        let dest_rect = Rectangle {
            x: frame.x - engine.camera_viewport.x, 
            y: frame.y - engine.camera_viewport.y,
            width: frame.width,
            height: frame.height,
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

fn draw_tile<T: SpriteTile>(
    d: &mut RaylibDrawHandle, 
    sprite_path: &str,
    tile: &T,
    variant: u32,
    engine: &GameEngine
) {
    let source_rect = tile.texture_source_rect(variant);
    
    if let Some(texture) = engine.textures.get(sprite_path) {
        let dest_rect = Rectangle {
            x: tile.column() as f32 * TILE_SIZE - engine.camera_viewport.x, 
            y: tile.row() as f32 * TILE_SIZE - engine.camera_viewport.y,
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