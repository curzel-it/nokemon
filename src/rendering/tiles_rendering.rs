use raylib::prelude::*;

use crate::{constants::TILE_SIZE, game_engine::{engine::GameEngine, world::World}, maps::tiles::SpriteTile};

pub fn render_tiles(d: &mut RaylibDrawHandle, world: &World, engine: &GameEngine) {
    draw_biome(d, world, engine);
    draw_constructions(d, world, engine);
}

fn draw_biome(d: &mut RaylibDrawHandle, world: &World, engine: &GameEngine) {    
    let sprite_key = world.biome_tiles.sheet_id;

    for tile in world.visible_biome_tiles(&engine.camera_viewport) {
        draw_tile(
            d, 
            sprite_key, 
            tile, 
            world.biome_tiles.current_variant(tile.row, tile.column), 
            engine
        );
    }
}

fn draw_constructions(d: &mut RaylibDrawHandle, world: &World, engine: &GameEngine) {    
    let sprite_key = world.constructions_tiles.sheet_id;

    for tile in world.visible_construction_tiles(&engine.camera_viewport) {
        draw_tile(
            d, 
            sprite_key, 
            tile, 
            0, 
            engine
        );
    }
}

fn draw_tile<T: SpriteTile>(
    d: &mut RaylibDrawHandle, 
    sprite_key: u32,
    tile: &T,
    variant: i32,
    engine: &GameEngine
) {    
    if let Some(texture) = engine.ui_config.as_ref().unwrap().get_texture(sprite_key) {
        let source = tile.texture_source_rect(variant);
        let scale = engine.rendering_scale();
        let tile_scale = scale * TILE_SIZE;

        let source_rect = Rectangle {
            x: TILE_SIZE * source.x as f32,
            y: TILE_SIZE * source.y as f32,
            width: TILE_SIZE * source.w as f32,
            height: TILE_SIZE * source.h as f32,
        };

        let actual_row = tile.row() as f32 - engine.camera_viewport.y as f32;
        let actual_col = tile.column() as f32 - engine.camera_viewport.x as f32;

        let dest_rect = Rectangle {
            x: actual_col * tile_scale - engine.camera_viewport_offset.x * scale, 
            y: actual_row * tile_scale - engine.camera_viewport_offset.y * scale, 
            width: tile_scale,
            height: tile_scale,
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