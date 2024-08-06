use crate::game_engine::{entity::Entity, game::Game, game_state_update::GameStateUpdate};

pub fn linear_movement(entity: &mut dyn Entity, game: &Game, time_since_last_update: f32) -> Vec<GameStateUpdate> {
    let frame = entity.frame();
    let offset = entity.direction() * entity.speed() * time_since_last_update;

    let mut expected_x = frame.x + offset.x;
    let mut expected_y = frame.y + offset.y;
    
    if entity.species().stays_inside_screen_bounds() {
        if expected_x < game.bounds.x {
            expected_x = game.bounds.x;
        }
        if (expected_x + frame.width) > (game.bounds.x + game.bounds.width) {
            expected_x = game.bounds.x + game.bounds.width - frame.width;
        }
        if expected_y < game.bounds.y {
            expected_y = game.bounds.y;
        }
        if (expected_y + frame.height) > (game.bounds.y + game.bounds.height) {
            expected_y = game.bounds.y + game.bounds.height - frame.height;
        }
    }
    entity.place_at(expected_x, expected_y);
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