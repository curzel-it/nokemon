use crate::{constants::{BASE_ENTITY_SPEED, SCALE}, game_engine::entity::Entity};

pub fn move_linearly(entity: &mut dyn Entity, time_since_last_update: f32) {
    let frame = entity.body().frame;
    let offset = entity.body().direction * entity.body().current_speed * time_since_last_update * SCALE * BASE_ENTITY_SPEED;
    entity.place_at(frame.x + offset.x, frame.y + offset.y);
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::{BASE_ENTITY_SPEED, RECT_ORIGIN_SQUARE_100, SCALE}, game_engine::{entity::Entity, entity_body::EmbodiedEntity, game::Game, simple_entity::SimpleEntity}};
    
    #[test]
    fn can_move_on_update() {
        let game = Game::test();
        
        let mut body = game.entity_factory.build("red");
        body.frame = RECT_ORIGIN_SQUARE_100;
        body.current_speed = 1.0;        
        
        let mut entity = SimpleEntity::new(body);
        entity.body_mut().direction = Vector2::new(1.0, 1.0);  
        entity.update(&game, 1.0);

        assert_eq!(entity.body().frame.x, SCALE * BASE_ENTITY_SPEED);
        assert_eq!(entity.body().frame.y, SCALE * BASE_ENTITY_SPEED);
    }

    #[test]
    fn can_move_outside_of_bounds() {
        let game = Game::test();
        
        let mut body = game.entity_factory.build("red");
        body.frame = RECT_ORIGIN_SQUARE_100;
        body.current_speed = 1.0;
        
        let mut entity = SimpleEntity::new(body);
        entity.body_mut().direction = Vector2::new(-1.0, 1.0);  
        entity.update(&game, 1.0);

        assert_eq!(entity.body().frame.x, -SCALE * BASE_ENTITY_SPEED);
        assert_eq!(entity.body().frame.y, SCALE * BASE_ENTITY_SPEED);
    }
}