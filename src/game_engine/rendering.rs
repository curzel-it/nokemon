use raylib::prelude::*;

use super::{entity::Entity, game::Game, game_engine::GameEngine};

pub fn draw_frame(rl: &mut RaylibHandle, thread: &RaylibThread, game: &Game, engine: &GameEngine) {
    let mut d = rl.begin_drawing(thread);

    d.clear_background(Color::BLACK);

    let mut sorted_entities: Vec<&Entity> = game.entities.values().collect();
    sorted_entities.sort_by_key(|e| e.z_index);

    for item in sorted_entities {
        draw_item(&mut d, &item, &engine);
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