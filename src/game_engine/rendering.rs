
use raylib::prelude::*;

use crate::constants::{BACKGROUND_TILE_GRASS, SCALE};

use super::{entity::Entity, game::Game, game_engine::GameEngine};

pub fn draw_frame(rl: &mut RaylibHandle, thread: &RaylibThread, game: &Game, engine: &GameEngine) {
    let fps = rl.get_fps();
    let mut d = rl.begin_drawing(thread);
    draw_background(&mut d, game, engine);
    draw_entities(&mut d, game, engine);
    draw_debug_info(&mut d, fps);
}

fn draw_debug_info(d: &mut RaylibDrawHandle, fps: u32) {
    d.draw_text(&format!("FPS: {}", fps), 10, 10, 20, Color::BLACK);
}

fn draw_entities(d: &mut RaylibDrawHandle, game: &Game, engine: &GameEngine) {
    let entities = game.entities.borrow();
    let mut sorted_entities: Vec<&Box<dyn Entity>> = entities.values().collect();
    sorted_entities.sort();

    for item in sorted_entities {
        draw_item(d, item, engine);
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
    item: &Box<dyn Entity>,
    engine: &GameEngine
) {
    let sprite_path = item.body().current_sprite_frame();
    let frame = item.body().frame;
    
    if let Some(texture) = engine.textures.get(sprite_path) {
        d.draw_texture_ex(
            texture,
            Vector2::new(frame.x, frame.y),
            0.0,
            frame.width / texture.width as f32, 
            Color::WHITE 
        );
    }
}