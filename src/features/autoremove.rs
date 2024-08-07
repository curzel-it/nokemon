use raylib::math::Rectangle;

use crate::{game_engine::{entity::Entity, game::Game, game_state_update::GameStateUpdate}, species::species_model::INFINITE_LIFESPAN};

pub fn remove_automatically(entity: &dyn Entity, game: &Game) -> Vec<GameStateUpdate> {
    if should_remove(game, entity) {
        return vec![GameStateUpdate::RemoveEntity(entity.id())];
    }
    vec![]
}

fn should_remove(game: &Game, entity: &dyn Entity) -> bool {
    let lifespan = entity.species().lifespan;
    let age = game.total_elapsed_time - entity.creation_time();

    if lifespan != INFINITE_LIFESPAN && age > lifespan {
        return true;
    }
    if entity.hp() <= 0.0 {
        return true;
    }       
    if is_outside_of_enlarged_bounds(&game.bounds, &entity.frame()) {
        return true;
    }
    false
}

fn is_outside_of_enlarged_bounds(bounds: &Rectangle, rect: &Rectangle) -> bool {
    let margin = 100.0;
    if rect.x < bounds.x - rect.width - margin { return true; }
    if rect.y < bounds.y - rect.height - margin { return true; }
    if rect.x > bounds.x + bounds.width + margin { return true; }
    if rect.y > bounds.y + bounds.height + margin { return true; }
    false
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::RECT_ORIGIN_SQUARE_100, game_engine::{game::Game, simple_entity::SimpleEntity}};

    #[test]
    fn can_remove_entities_outside_of_screen() {
        let mut game = Game::test();
        
        let mut body = game.entity_factory.build("towerdart");
        body.frame = RECT_ORIGIN_SQUARE_100;
        body.speed = 100.0;  
        body.direction =  Vector2::new(-1.0, 0.0);
        game.add_entity(Box::new(SimpleEntity::new(body)));

        game.update(0.6);
        assert_eq!(game.entities.borrow().len(), 1);
                
        game.update(0.6);
        game.update(0.6);
        game.update(0.6);
        game.update(0.6);
        game.update(0.6);
        game.update(0.6);
        game.update(0.6);
        game.update(0.6);
        assert_eq!(game.entities.borrow().len(), 0);
    }

    #[test]
    fn can_remove_entities_with_no_hp_left() {
        let mut game = Game::test();
        
        let mut body = game.entity_factory.build("towerdart");
        body.frame = RECT_ORIGIN_SQUARE_100;
        body.speed = 100.0;   
        body.direction = Vector2::zero();
        body.hp = 0.0;
        game.add_entity(Box::new(SimpleEntity::new(body)));

        assert_eq!(game.entities.borrow().len(), 1);
        game.update(0.1);
        assert_eq!(game.entities.borrow().len(), 0);
    }

    #[test]
    fn can_remove_entities_with_passed_expiration_date() {
        let mut game = Game::test();
        
        let mut body = game.entity_factory.build("baseattack");
        let lifespan = body.species.lifespan;
        body.frame = RECT_ORIGIN_SQUARE_100;
        body.speed = 0.0;
        body.direction = Vector2::zero();
        game.add_entity(Box::new(SimpleEntity::new(body)));

        assert_eq!(game.entities.borrow().len(), 1);
        game.update(lifespan + 1.0);
        assert_eq!(game.entities.borrow().len(), 0);
    }
}