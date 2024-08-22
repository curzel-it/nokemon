use raylib::prelude::*;

use crate::{constants::TILE_SIZE, game_engine::{game_engine::GameEngine, world::World}, maps::tiles::SpriteTile, utils::rect::Rect};

pub fn render_tiles(d: &mut RaylibDrawHandle, world: &World, engine: &GameEngine) {
    draw_biome(d, world, engine);
    draw_constructions(d, world, engine);
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

fn draw_tile<T: SpriteTile>(
    d: &mut RaylibDrawHandle, 
    sprite_path: &str,
    tile: &T,
    variant: u32,
    engine: &GameEngine
) {
    let source_rect = tile.texture_source_rect(variant).as_rr();
    
    if let Some(texture) = engine.ui_config.as_ref().unwrap().get_texture(sprite_path) {
        let dest_rect = Rect {
            x: tile.column() as f32 * TILE_SIZE - engine.camera_viewport.x, 
            y: tile.row() as f32 * TILE_SIZE - engine.camera_viewport.y,
            w: TILE_SIZE,
            h: TILE_SIZE,
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