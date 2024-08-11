use std::collections::HashSet;

use raylib::math::Rectangle;

use super::game::Game;

pub fn compute_visible_entities(game: &Game) -> HashSet<u32> {
    game.entities.borrow()
        .values()
        .filter(|e| is_visible(&e.body().frame, game))
        .map(|e| e.id())
        .collect()        
}

pub fn is_visible(rect: &Rectangle, game: &Game) -> bool {
    game.camera_viewport.check_collision_recs(rect)
}