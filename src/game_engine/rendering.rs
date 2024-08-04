use std::cmp::Ordering;

use raylib::prelude::*;

use crate::constants::{BACKGROUND_TILE_GRASS, SCALE};

use super::{entity::Entity, game::Game, game_engine::GameEngine};

pub fn draw_frame(rl: &mut RaylibHandle, thread: &RaylibThread, game: &Game, engine: &GameEngine) {
    let mut d = rl.begin_drawing(thread);
    draw_background(&mut d, game, engine);
    draw_entities(&mut d, game, engine);
}

fn draw_entities(d: &mut RaylibDrawHandle, game: &Game, engine: &GameEngine) {
    let mut sorted_entities: Vec<&Entity> = game.entities.values().collect();
    sorted_entities.sort_by(|a, b| {
        if a.frame.y < b.frame.y { return Ordering::Less; }
        if a.frame.y > b.frame.y { return Ordering::Greater; }
        if a.species.z_index < b.species.z_index { return Ordering::Less; }
        if a.species.z_index > b.species.z_index { return Ordering::Greater; }
        return Ordering::Equal;
    });

    for item in sorted_entities {
        draw_item(d, &item, &engine);
    }
}

fn draw_background(d: &mut RaylibDrawHandle, game: &Game, engine: &GameEngine) {
    if let Some(grasstile) = engine.textures.get(BACKGROUND_TILE_GRASS) {
        let tile_width = grasstile.width() as usize;
        let tile_height = grasstile.height() as usize;

        for x in (0..game.bounds.width as i32).step_by(tile_width) {
            for y in (0..game.bounds.height as i32).step_by(tile_height) {
                d.draw_texture_ex(
                    grasstile,
                    Vector2::new(x as f32, y as f32),
                    0.0,
                    SCALE, 
                    Color::WHITE 
                );
            }
        }
    }
}

fn draw_item(
    d: &mut RaylibDrawHandle, 
    item: &Entity,
    engine: &GameEngine
) {
    let sprite_path = item.current_sprite.current_frame();
    
    if let Some(texture) = engine.textures.get(sprite_path) {
        let is_being_dragged = engine.dragging_id == Some(item.id);
        let dx = if is_being_dragged { engine.drag_offset.x } else { 0.0 };
        let dy = if is_being_dragged { engine.drag_offset.y } else { 0.0 };

        d.draw_texture_ex(
            texture,
            Vector2::new(item.frame.x + dx, item.frame.y + dy),
            0.0,
            item.frame.width / texture.width as f32, 
            Color::WHITE 
        );
    }
}