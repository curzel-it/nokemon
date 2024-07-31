use crate::game_engine::{game::Game, game_behavior::GameBehavior};

#[derive(Debug)]
pub struct LinearMovement;

impl LinearMovement {
    pub fn new() -> Self {
        Self {}
    }
}

impl GameBehavior for LinearMovement {
    fn update(&self, entity_id: &u32, game: &mut Game, time_since_last_update: f32) {
        let entity = game.entities.get_mut(entity_id).unwrap();
        let offset = entity.direction * entity.speed * time_since_last_update;
        entity.frame.x += offset.x;
        entity.frame.y += offset.y;
    }
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::RECT_ORIGIN_SQUARE_100, game_engine::{entity_factory::EntityFactory, game::Game}};
    
    #[test]
    fn can_move_on_update() {
        let mut game = Game::test();
        let entity_factory = EntityFactory::test();
        
        let mut entity = entity_factory.build("ape");
        entity.frame = RECT_ORIGIN_SQUARE_100;
        entity.direction = Vector2::new(1.0, 1.0);
        game.add_entity(entity);
                
        // game.update(1.0);
        let result = game.frame_of_first_entity();
        assert_eq!(result.x, 30.0);
        assert_eq!(result.y, 30.0);
    }
}