use raylib::prelude::*;

use crate::{constants::TILE_SIZE, game_engine::{game_engine::GameEngine, world::World}, maps::tiles::SpriteTile};

pub fn render_tiles(d: &mut RaylibDrawHandle, world: &World, engine: &GameEngine) {
    draw_biome(d, world, engine);
    draw_constructions(d, world, engine);
}

fn draw_biome(d: &mut RaylibDrawHandle, world: &World, engine: &GameEngine) {    
    let sprite_path = world.biome_tiles.sheet_id;
    let scale = engine.ui_config.as_ref().unwrap().rendering_scale * TILE_SIZE;

    for tile in world.visible_biome_tiles(&engine.camera_viewport) {
        draw_tile(
            d, 
            sprite_path, 
            scale,
            tile, 
            world.biome_tiles.current_variant(tile.row, tile.column), 
            engine
        );
    }
}

fn draw_constructions(d: &mut RaylibDrawHandle, world: &World, engine: &GameEngine) {    
    let sprite_path = world.constructions_tiles.sheet_id;
    let scale = engine.ui_config.as_ref().unwrap().rendering_scale * TILE_SIZE;

    for tile in world.visible_construction_tiles(&engine.camera_viewport) {
        draw_tile(
            d, 
            sprite_path, 
            scale,
            tile, 
            0, 
            engine
        );
    }
}

fn draw_tile<T: SpriteTile>(
    d: &mut RaylibDrawHandle, 
    sprite_path: u32,
    scale: f32,
    tile: &T,
    variant: u32,
    engine: &GameEngine
) {
    let source_rect = tile.texture_source_rect(variant).as_rr();
    
    if let Some(texture) = engine.ui_config.as_ref().unwrap().get_texture(sprite_path) {
        let dest_rect = Rectangle {
            x: (tile.column() - engine.camera_viewport.x) as f32 * scale, 
            y: (tile.row() - engine.camera_viewport.y) as f32 * scale, 
            width: scale,
            height: scale,
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