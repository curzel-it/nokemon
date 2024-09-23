use raylib::prelude::*;

use crate::{constants::{SPRITE_SHEET_BIOME_TILES, TILE_SIZE}, game_engine::{engine::GameEngine, world::World}, maps::{biome_tiles::{Biome, BiomeTile}, constructions_tiles::Construction, tiles::SpriteTile}};

const WATER: BiomeTile = BiomeTile { 
    tile_type: Biome::Water, 
    column: 0, 
    row: 0, 
    tile_up_type: Biome::Water,
    tile_right_type: Biome::Water,
    tile_down_type: Biome::Water,
    tile_left_type: Biome::Water,
    texture_offset_x: 0, 
    texture_offset_y: 0
};

const NOTHING: BiomeTile = BiomeTile { 
    tile_type: Biome::Nothing, 
    column: 0, 
    row: 0, 
    tile_up_type: Biome::Nothing,
    tile_right_type: Biome::Nothing,
    tile_down_type: Biome::Nothing,
    tile_left_type: Biome::Nothing,
    texture_offset_x: 0, 
    texture_offset_y: 7
};

pub fn render_tiles(d: &mut RaylibDrawHandle, world: &World, engine: &GameEngine) {
    draw_tiles_in_viewport(d, world, engine);
}

fn draw_tiles_in_viewport(d: &mut RaylibDrawHandle, world: &World, engine: &GameEngine) {
    let sprite_key_biome = world.biome_tiles.sheet_id;
    let sprite_key_constructions = world.constructions_tiles.sheet_id;

    let x_start = engine.camera_viewport.x - 1;
    let y_start = engine.camera_viewport.y - 1;
    let x_end = x_start + engine.camera_viewport.w + 3;
    let y_end = y_start + engine.camera_viewport.h + 3;

    for col in x_start..x_end {
        for row in y_start..y_end {
            if col < 0 || row < 0 || col >= world.bounds.w || row >= world.bounds.h {
                draw_default_biome(d, world.is_interior, row, col, engine);
            } else {
                let row = row as usize;
                let col = col as usize;

                if !matches!(world.biome_tiles.tiles[row][col].tile_type, Biome::Nothing) {
                    draw_tile(
                        d, 
                        sprite_key_biome, 
                        &world.biome_tiles.tiles[row][col], 
                        world.biome_tiles.current_variant(row, col), 
                        engine
                    );
                }
                if !matches!(world.constructions_tiles.tiles[row][col].tile_type, Construction::Nothing) {
                    draw_tile(
                        d, 
                        sprite_key_constructions, 
                        &world.constructions_tiles.tiles[row][col], 
                        0, 
                        engine
                    );
                }
            }
        }
    }
}

fn draw_default_biome(d: &mut RaylibDrawHandle, interior: bool, row: i32, col: i32, engine: &GameEngine) { 
    draw_tile_row_col(
        d, 
        SPRITE_SHEET_BIOME_TILES, 
        if interior { &NOTHING } else { &WATER },
        row as f32, 
        col as f32,
        0, 
        engine
    );
}

fn draw_tile<T: SpriteTile>(
    d: &mut RaylibDrawHandle, 
    sprite_key: u32,
    tile: &T,
    variant: i32,
    engine: &GameEngine
) {    
    draw_tile_row_col(d, sprite_key, tile, tile.row() as f32, tile.column() as f32, variant, engine)
}

fn draw_tile_row_col<T: SpriteTile>(
    d: &mut RaylibDrawHandle, 
    sprite_key: u32,
    tile: &T,
    row: f32,
    column: f32,
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

        let actual_row = row - engine.camera_viewport.y as f32;
        let actual_col = column - engine.camera_viewport.x as f32;

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