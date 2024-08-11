use raylib::math::Rectangle;

use crate::{constants::{BASE_ENTITY_SPEED, SCALE}, game_engine::entity::Entity};

pub fn move_linearly_within_bounds(entity: &mut dyn Entity, game_bounds: &Rectangle, time_since_last_update: f32) {
    let frame = entity.body().frame;
    let offset = entity.body().direction * entity.body().current_speed * time_since_last_update * SCALE * BASE_ENTITY_SPEED;

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

    use crate::{constants::{BASE_ENTITY_SPEED, RECT_ORIGIN_SQUARE_100, SCALE}, game_engine::{entity::Entity, entity_body::EmbodiedEntity, world::World, simple_entity::SimpleEntity}};
    
    #[test]
    fn can_move_on_update() {
        let world = World::test();
        
        let mut body = world.entity_factory.build("red");
        body.frame = RECT_ORIGIN_SQUARE_100;
        body.current_speed = 1.0;
        
        let mut entity = SimpleEntity::new(body);
        entity.body_mut().direction = Vector2::new(1.0, 1.0);  
        entity.update(&world, 1.0);

        assert_eq!(entity.body().frame.x, SCALE * BASE_ENTITY_SPEED);
        assert_eq!(entity.body().frame.y, SCALE * BASE_ENTITY_SPEED);
    }
}