use crate::game_engine::{entity::Entity, game_state_update::GameStateUpdate};

pub fn move_linearly(entity: &mut dyn Entity, time_since_last_update: f32) -> Vec<GameStateUpdate> {
    let frame = entity.frame();
    let offset = entity.direction() * entity.speed() * time_since_last_update;
    entity.place_at(frame.x + offset.x, frame.y + offset.y);
    vec![]
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