use raylib::math::Rectangle;

use crate::game_engine::entity::Entity;

pub fn move_linearly_within_bounds(entity: &mut dyn Entity, game_bounds: &Rectangle, time_since_last_update: f32) {
    let frame = entity.frame();
    let offset = entity.direction() * entity.speed() * time_since_last_update;

    let mut expected_x = frame.x + offset.x;
    let mut expected_y = frame.y + offset.y;
    
    if expected_x < game_bounds.x {
        expected_x = game_bounds.x;
    }
    if (expected_x + frame.width) > (game_bounds.x + game_bounds.width) {
        expected_x = game_bounds.x + game_bounds.width - frame.width;
    }
    if expected_y < game_bounds.y {
        expected_y = game_bounds.y;
    }
    if (expected_y + frame.height) > (game_bounds.y + game_bounds.height) {
        expected_y = game_bounds.y + game_bounds.height - frame.height;
    }

    entity.place_at(expected_x, expected_y);
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::{BASE_ENTITY_SPEED, RECT_ORIGIN_SQUARE_100}, game_engine::{entity::Entity, entity_body::EmbodiedEntity, game::Game, simple_entity::SimpleEntity}};
    
    #[test]
    fn can_move_on_update() {
        let game = Game::test();
        
        let mut body = game.entity_factory.build("red");
        body.frame = RECT_ORIGIN_SQUARE_100;
        body.speed = BASE_ENTITY_SPEED;        
        
        let mut entity = SimpleEntity::new(body);
        entity.set_direction(Vector2::new(1.0, 1.0));  
        entity.update(&game, 1.0);

        assert_eq!(entity.frame().x, 30.0);
        assert_eq!(entity.frame().y, 30.0);
    }
}