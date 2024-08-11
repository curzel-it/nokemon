
use std::{borrow::Borrow, cmp::Ordering};

use raylib::prelude::*;

use super::{entity::Entity, entity_body::EmbodiedEntity, game::Game, game_engine::GameEngine};

pub fn draw_frame(rl: &mut RaylibHandle, thread: &RaylibThread, game: &Game, engine: &GameEngine) {
    let fps = rl.get_fps();
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::BLACK);
    draw_tiles(&mut d, game, engine);
    draw_entities(&mut d, game, engine);
    draw_debug_info(&mut d, game, fps);
}

fn draw_debug_info(d: &mut RaylibDrawHandle, _: &Game, fps: u32) {
    d.draw_text(&format!("FPS: {}", fps), 10, 10, 20, Color::RED);
    // d.draw_text(format!("Entities: {:#?}", game).as_str(), 10, 50, 20, Color::RED);
}

fn draw_tiles(d: &mut RaylibDrawHandle, game: &Game, engine: &GameEngine) {
    for tile in &game.tiles {
        if engine.camera_viewport.check_collision_recs(&tile.body().frame) {
            draw_item(d, tile, engine);
        }
    }
}

fn draw_entities(d: &mut RaylibDrawHandle, game: &Game, engine: &GameEngine) {
    let entities_map = game.entities.borrow();
    let mut entities: Vec<&Box<dyn Entity>> = entities_map
        .values()
        .filter(|e| engine.camera_viewport.check_collision_recs(&e.body().frame))
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

fn draw_item(
    d: &mut RaylibDrawHandle, 
    item: &dyn Entity,
    engine: &GameEngine
) {
    let sprite_path = item.body().current_sprite_frame();
    let frame = item.body().frame;
    let position = Vector2::new(frame.x - engine.camera_viewport.x, frame.y - engine.camera_viewport.y);
    
    if let Some(texture) = engine.textures.get(sprite_path) {
        d.draw_texture_ex(
            texture,
            position,
            0.0,
            frame.width / texture.width as f32, 
            Color::WHITE 
        );
    }
}