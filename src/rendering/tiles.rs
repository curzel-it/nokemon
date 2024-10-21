use raylib::prelude::*;

use crate::{constants::TILE_SIZE, game_engine::{engine::GameEngine, world::World}, maps::{biome_tiles::Biome, constructions_tiles::Construction, tiles::SpriteTile}};

use super::ui::{get_rendering_config, RENDERING_CONFIG};

pub fn render_tiles(d: &mut RaylibDrawHandle, world: &World, engine: &GameEngine) {
    draw_tiles_in_viewport(d, world, engine);
}

fn draw_tiles_in_viewport(d: &mut RaylibDrawHandle, world: &World, engine: &GameEngine) {
    let sprite_key_biome = world.biome_tiles.sheet_id;
    let sprite_key_constructions = world.constructions_tiles.sheet_id;
    let default_tile = world.default_tile();

    let config = get_rendering_config();
    let texture_biome = config.get_texture(sprite_key_biome).unwrap();
    let texture_constructions = config.get_texture(sprite_key_constructions).unwrap();
    let scale = config.rendering_scale;

    let tile_scale = scale * TILE_SIZE;
    let camera_offset_x = engine.camera_viewport_offset.x * scale;
    let camera_offset_y = engine.camera_viewport_offset.y * scale;

    let x_start = engine.camera_viewport.x - 1;
    let y_start = engine.camera_viewport.y - 1;
    let x_end = x_start + engine.camera_viewport.w + 3;
    let y_end = y_start + engine.camera_viewport.h + 3;

    let world_width = world.bounds.w;
    let world_height = world.bounds.h;

    for col in x_start..x_end {
        for row in y_start..y_end {
            let actual_row = row as f32 - engine.camera_viewport.y as f32;
            let actual_col = col as f32 - engine.camera_viewport.x as f32;

            let dest_rect = Rectangle {
                x: actual_col * tile_scale - camera_offset_x,
                y: actual_row * tile_scale - camera_offset_y,
                width: tile_scale,
                height: tile_scale,
            };

            if col < 0 || row < 0 || col >= world_width || row >= world_height {
                let variant = world.biome_tiles.current_variant(0, 0);
                let source = default_tile.texture_source_rect(variant);

                let source_rect = Rectangle {
                    x: TILE_SIZE * source.x as f32,
                    y: TILE_SIZE * source.y as f32,
                    width: TILE_SIZE * source.w as f32,
                    height: TILE_SIZE * source.h as f32,
                };

                d.draw_texture_pro(
                    texture_biome,
                    source_rect,
                    dest_rect,
                    Vector2::zero(),
                    0.0,
                    Color::WHITE,
                );
            } else {
                let row_usize = row as usize;
                let col_usize = col as usize;

                let biome_tile = &world.biome_tiles.tiles[row_usize][col_usize];
                let construction_tile = &world.constructions_tiles.tiles[row_usize][col_usize];

                if !matches!(biome_tile.tile_type, Biome::Nothing) {
                    let variant = world.biome_tiles.current_variant(row_usize, col_usize);
                    let source = biome_tile.texture_source_rect(variant);

                    let source_rect = Rectangle {
                        x: TILE_SIZE * source.x as f32,
                        y: TILE_SIZE * source.y as f32,
                        width: TILE_SIZE * source.w as f32,
                        height: TILE_SIZE * source.h as f32,
                    };

                    d.draw_texture_pro(
                        texture_biome,
                        source_rect,
                        dest_rect,
                        Vector2::zero(),
                        0.0,
                        Color::WHITE,
                    );
                }

                if !matches!(construction_tile.tile_type, Construction::Nothing) {
                    let source = construction_tile.texture_source_rect(0);

                    let source_rect = Rectangle {
                        x: TILE_SIZE * source.x as f32,
                        y: TILE_SIZE * source.y as f32,
                        width: TILE_SIZE * source.w as f32,
                        height: TILE_SIZE * source.h as f32,
                    };

                    d.draw_texture_pro(
                        texture_constructions,
                        source_rect,
                        dest_rect,
                        Vector2::zero(),
                        0.0,
                        Color::WHITE,
                    );
                }
            }
        }
    }
}
