use crate::game_engine::{world::Game, behaviors::EntityBehavior};

#[derive(Debug)]
pub struct LinearMovement;

impl LinearMovement {
    pub fn new() -> Self {
        Self {}
    }
}

impl EntityBehavior for LinearMovement {
    fn update(&self, entity_id: &u32, world: &mut Game, time_since_last_update: f32) {
        let entity = world.entities.get_mut(entity_id).unwrap();
        let offset = entity.direction * entity.speed * time_since_last_update;

        let mut expected_x = entity.frame.x + offset.x;
        let mut expected_y = entity.frame.y + offset.y;
        
        if entity.species.stays_inside_screen_bounds() {
            if expected_x < world.bounds.x {
                expected_x = world.bounds.x;
            }
            if (expected_x + entity.frame.width) > (world.bounds.x + world.bounds.width) {
                expected_x = world.bounds.x + world.bounds.width - entity.frame.width;
            }
            if expected_y < world.bounds.y {
                expected_y = world.bounds.y;
            }
            if (expected_y + entity.frame.height) > (world.bounds.y + world.bounds.height) {
                expected_y = world.bounds.y + world.bounds.height - entity.frame.height;
            }
        }
        entity.frame.x = expected_x;
        entity.frame.y = expected_y;
    }
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::{BASE_ENTITY_SPEED, RECT_ORIGIN_SQUARE_100}, game_engine::{world::Game, game_engine::GameEngine, keyboard_events_provider::NoKeyboard}};
    
    #[test]
    fn can_move_on_update() {
        let engine = GameEngine::new();
        let mut world = Game::test();
        let nokb = NoKeyboard {};
        
        let mut entity = world.entity_factory.build("red");
        let entity_id = entity.id;
        entity.frame = RECT_ORIGIN_SQUARE_100;
        entity.speed = BASE_ENTITY_SPEED;
        entity.change_direction(Vector2::new(1.0, 1.0));  
        world.add_entity(entity);
                
        engine.update_rl(&mut world, 1.0, &nokb);
        let result = world.frame_of_entity(&entity_id);
        assert_eq!(result.x, 30.0);
        assert_eq!(result.y, 30.0);
    }
}