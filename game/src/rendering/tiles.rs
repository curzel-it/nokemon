use game_core::{constants::TILE_SIZE, maps::{biome_tiles::{Biome, BiomeTile}, constructions_tiles::{Construction, ConstructionTile}, tiles::SpriteTile}, utils::{rect::IntRect, vector::Vector2d}};
use raylib::prelude::*;

use super::ui::get_rendering_config;

pub fn render_tiles(
    d: &mut RaylibDrawHandle, 
    sprite_key_biome: u32,
    sprite_key_constructions: u32,
    variant: i32,
    world_width: i32,
    world_height: i32,
    camera_viewport: &IntRect, 
    camera_viewport_offset: &Vector2d,
    default_tile: &BiomeTile,
    biome_tiles: &[Vec<BiomeTile>],
    constructions_tiles: &[Vec<ConstructionTile>]
) {
    let config = get_rendering_config();
    let texture_biome = config.get_texture(sprite_key_biome).unwrap();
    let texture_constructions = config.get_texture(sprite_key_constructions).unwrap();
    let scale = config.rendering_scale;

    let tile_scale = scale * TILE_SIZE;
    let camera_offset_x = camera_viewport_offset.x * scale;
    let camera_offset_y = camera_viewport_offset.y * scale;

    let x_start = camera_viewport.x - 1;
    let y_start = camera_viewport.y - 1;
    let x_end = x_start + camera_viewport.w + 3;
    let y_end = y_start + camera_viewport.h + 3;

    for col in x_start..x_end {
        for row in y_start..y_end {
            let actual_row = row as f32 - camera_viewport.y as f32;
            let actual_col = col as f32 - camera_viewport.x as f32;

            let dest_rect = Rectangle {
                x: actual_col * tile_scale - camera_offset_x,
                y: actual_row * tile_scale - camera_offset_y,
                width: tile_scale,
                height: tile_scale,
            };

            if col < 0 || row < 0 || col >= world_width || row >= world_height {
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

                let biome_tile = &biome_tiles[row_usize][col_usize];
                let construction_tile = &constructions_tiles[row_usize][col_usize];

                if !matches!(biome_tile.tile_type, Biome::Nothing) {
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
