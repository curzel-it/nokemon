use crate::game_engine::{game::Game, behaviors::EntityBehavior};

#[derive(Debug)]
pub struct LinearMovement;

impl LinearMovement {
    pub fn new() -> Self {
        Self {}
    }
}

impl EntityBehavior for LinearMovement {
    fn update(&self, entity_id: &u32, game: &mut Game, time_since_last_update: f32) {
        let entity = game.entities.get_mut(entity_id).unwrap();
        let offset = entity.direction * entity.speed * time_since_last_update;

        let mut expected_x = entity.frame.x + offset.x;
        let mut expected_y = entity.frame.y + offset.y;
        
        if entity.species.stays_inside_screen_bounds {
            if expected_x < game.bounds.x {
                expected_x = game.bounds.x;
            }
            if (expected_x + entity.frame.width) > (game.bounds.x + game.bounds.width) {
                expected_x = game.bounds.x + game.bounds.width - entity.frame.width;
            }
            if expected_y < game.bounds.y {
                expected_y = game.bounds.y;
            }
            if (expected_y + entity.frame.height) > (game.bounds.y + game.bounds.height) {
                expected_y = game.bounds.y + game.bounds.height - entity.frame.height;
            }
        }
        entity.frame.x = expected_x;
        entity.frame.y = expected_y;
    }
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::RECT_ORIGIN_SQUARE_100, game_engine::{game::Game, game_engine::GameEngine}};
    
    #[test]
    fn can_move_on_update() {
        let engine = GameEngine::new();
        let mut game = Game::test();
        
        let mut entity = game.entity_factory.build("ape");
        let entity_id = entity.id;
        entity.frame = RECT_ORIGIN_SQUARE_100;
        entity.change_direction(Vector2::new(1.0, 1.0));  
        game.add_entity(entity);
                
        engine.update(&mut game, 1.0);
        let result = game.frame_of_entity(&entity_id);
        assert_eq!(result.x, 30.0);
        assert_eq!(result.y, 30.0);
    }
}