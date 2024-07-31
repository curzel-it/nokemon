use crate::game_engine::{game::Game, game_behavior::GameBehavior};

#[derive(Debug)]
pub struct RemoveEntitiesOutsideOfBounds;

impl RemoveEntitiesOutsideOfBounds {
    pub fn new() -> Self {
        Self {}
    }
}

impl GameBehavior for RemoveEntitiesOutsideOfBounds {
    fn update(&self, entity_id: &u32, game: &mut Game, _: f32) {
        let entity = game.entities.get_mut(entity_id).unwrap();
        let is_outside = game.bounds.get_collision_rec(&entity.frame).is_none();
        
        if is_outside {
            game.remove_entity(entity_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::RECT_ORIGIN_SQUARE_100, game_engine::{game::Game, game_update::GameEngine}};

    #[test]
    fn can_remove_automatically_when_leaving_screen() {
        let engine = GameEngine::new();
        let mut game = Game::test();
        
        let mut entity = game.entity_factory.build("towerdart");
        entity.frame = RECT_ORIGIN_SQUARE_100;
        entity.speed = 100.0;
        entity.direction = Vector2::new(-1.0, 0.0);  
        game.add_entity(entity);      
                
        engine.update(&mut game, 0.6);
        assert_eq!(game.entities.len(), 1);
                
        engine.update(&mut game, 0.6);
        assert_eq!(game.entities.len(), 0);
    }
}