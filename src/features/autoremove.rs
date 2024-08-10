use raylib::math::Rectangle;

use crate::{constants::INFINITE_LIFESPAN, game_engine::{entity::Entity, game::Game, game_state_update::GameStateUpdate}};

pub fn remove_automatically(entity: &dyn Entity, game: &Game) -> Vec<GameStateUpdate> {
    if should_remove(game, entity) {
        return vec![GameStateUpdate::RemoveEntity(entity.id())];
    }
    vec![]
}

fn should_remove(game: &Game, entity: &dyn Entity) -> bool {
    let lifespan = entity.body().lifespan;
    let age = game.total_elapsed_time - entity.body().creation_time;

    if lifespan != INFINITE_LIFESPAN && age > lifespan {
        return true;
    }
    if entity.body().current_hp <= 0.0 {
        return true;
    }       
    if !game.outer_bounds.check_collision_recs(&entity.body().frame) {
        return true;
    }
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
        body.current_speed = 100.0;  
        body.direction =  Vector2::new(-1.0, 0.0);
        game.add_entity(Box::new(SimpleEntity::new(body)));

        assert_eq!(game.entities.borrow().len(), 1);
        game.update(1.0);
        assert_eq!(game.entities.borrow().len(), 0);
    }

    #[test]
    fn can_remove_entities_with_no_hp_left() {
        let mut game = Game::test();
        
        let mut body = game.entity_factory.build("towerdart");
        body.frame = RECT_ORIGIN_SQUARE_100;
        body.current_speed = 100.0;   
        body.direction = Vector2::zero();
        body.current_hp = 0.0;
        game.add_entity(Box::new(SimpleEntity::new(body)));

        assert_eq!(game.entities.borrow().len(), 1);
        game.update(0.1);
        assert_eq!(game.entities.borrow().len(), 0);
    }

    #[test]
    fn can_remove_entities_with_passed_expiration_date() {
        let mut game = Game::test();
        
        let mut body = game.entity_factory.build("baseattack");
        body.lifespan = 10.0;
        body.frame = RECT_ORIGIN_SQUARE_100;
        body.current_speed = 0.0;
        body.direction = Vector2::zero();
        game.add_entity(Box::new(SimpleEntity::new(body)));

        assert_eq!(game.entities.borrow().len(), 1);
        game.update(11.0);
        assert_eq!(game.entities.borrow().len(), 0);
    }
}