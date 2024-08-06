use raylib::math::Rectangle;

use crate::{game_engine::{behaviors::EntityBehavior, entity::Entity, game::Game}, species::species_model::INFINITE_LIFESPAN};

#[derive(Debug)]
pub struct CleanupEntities;

impl CleanupEntities {
    pub fn new() -> Self {
        Self {}
    }
}

impl EntityBehavior for CleanupEntities {
    fn update(&self, entity_id: &u32, game: &mut Game, _: f32) {
        let entity = game.entities.get(entity_id).unwrap();
        
        if self.should_remove(game, entity) {
            game.remove_entity(entity_id);
        }
    }
}

impl CleanupEntities {
    fn should_remove(&self, game: &Game, entity: &Entity) -> bool {
        if entity.species.lifespan != INFINITE_LIFESPAN && game.total_elapsed_time - entity.creation_time > entity.species.lifespan {
            return true;
        }
        if entity.hp <= 0.0 {
            return true;
        }       
        if self.is_outside_of_enlarged_bounds(&game.bounds, &entity.frame) {
            return true;
        }
        return false;
    }

    fn is_outside_of_enlarged_bounds(&self, bounds: &Rectangle, rect: &Rectangle) -> bool {
        let margin = 100.0;
        if rect.x < bounds.x - rect.width - margin { return true; }
        if rect.y < bounds.y - rect.height - margin { return true; }
        if rect.x > bounds.x + bounds.width + margin { return true; }
        if rect.y > bounds.y + bounds.height + margin { return true; }
        return false
    }
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::RECT_ORIGIN_SQUARE_100, game_engine::{game::Game, game_engine::GameEngine, keyboard_events_provider::NoKeyboard}};

    #[test]
    fn can_remove_entities_outside_of_screen() {
        let engine = GameEngine::new();
        let mut game = Game::test();
        let nokb = NoKeyboard {};
        
        let mut entity = game.entity_factory.build("towerdart");
        entity.frame = RECT_ORIGIN_SQUARE_100;
        entity.speed = 100.0;
        entity.change_direction(Vector2::new(-1.0, 0.0));  
        game.add_entity(entity);      

        engine.update(&mut game, 0.6, &nokb);
        assert_eq!(game.entities.len(), 1);
                
        engine.update(&mut game, 0.6, &nokb);
        assert_eq!(game.entities.len(), 0);
    }

    #[test]
    fn can_remove_entities_with_no_hp_left() {
        let engine = GameEngine::new();
        let mut game = Game::test();
        let nokb = NoKeyboard {};
        
        let mut entity = game.entity_factory.build("towerdart");
        entity.frame = RECT_ORIGIN_SQUARE_100;
        entity.speed = 100.0;
        entity.change_direction(Vector2::new(-1.0, 0.0));  
        entity.hp = 0.0; 
        game.add_entity(entity);      

        assert_eq!(game.entities.len(), 1);
        engine.update(&mut game, 0.1, &nokb);
        assert_eq!(game.entities.len(), 0);
    }

    #[test]
    fn can_remove_entities_with_passed_expiration_date() {
        let engine = GameEngine::new();
        let mut game = Game::test();
        let nokb = NoKeyboard {};
        
        let mut entity = game.entity_factory.build("baseattack");
        let lifespan = entity.species.lifespan;
        entity.frame = RECT_ORIGIN_SQUARE_100;
        entity.speed = 0.0;
        entity.change_direction(Vector2::zero());  

        game.add_entity(entity);      

        assert_eq!(game.entities.len(), 1);
        engine.update(&mut game, lifespan + 1.0, &nokb);
        assert_eq!(game.entities.len(), 0);
    }
}