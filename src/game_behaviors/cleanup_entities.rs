use crate::game_engine::{entity::Entity, game::Game, game_behavior::GameBehavior};

#[derive(Debug)]
pub struct CleanupEntities;

impl CleanupEntities {
    pub fn new() -> Self {
        Self {}
    }
}

impl GameBehavior for CleanupEntities {
    fn update(&self, entity_id: &u32, game: &mut Game, _: f32) {
        let entity = game.entities.get(entity_id).unwrap();
        
        if self.should_remove(game, entity) {
            game.remove_entity(entity_id);
        }
    }
}

impl CleanupEntities {
    fn should_remove(&self, game: &Game, entity: &Entity) -> bool {
        if entity.hp <= 0.0 {
            return true;
        }        
        if game.bounds.get_collision_rec(&entity.frame).is_none() {
            return true;
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::RECT_ORIGIN_SQUARE_100, game_engine::{game::Game, game_engine::GameEngine}};

    #[test]
    fn can_remove_entities_outside_of_screen() {
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

    #[test]
    fn can_remove_entities_with_no_hp_left() {
        let engine = GameEngine::new();
        let mut game = Game::test();
        
        let mut entity = game.entity_factory.build("towerdart");
        entity.frame = RECT_ORIGIN_SQUARE_100;
        entity.speed = 100.0;
        entity.direction = Vector2::new(-1.0, 0.0); 
        entity.hp = 0.0; 
        game.add_entity(entity);      

        assert_eq!(game.entities.len(), 1);
        engine.update(&mut game, 0.1);
        assert_eq!(game.entities.len(), 0);
    }
}