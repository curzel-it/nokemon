
use crate::{constants::INFINITE_LIFESPAN, game_engine::{entity::Entity, world::World, game_state_update::GameStateUpdate}};

pub fn remove_automatically(entity: &dyn Entity, world: &World) -> Vec<GameStateUpdate> {
    if should_remove(world, entity) {
        return vec![GameStateUpdate::RemoveEntity(entity.id())];
    }
    vec![]
}

fn should_remove(world: &World, entity: &dyn Entity) -> bool {
    let lifespan = entity.body().lifespan;
    let age = world.total_elapsed_time - entity.body().creation_time;

    if lifespan != INFINITE_LIFESPAN && age > lifespan {
        return true;
    }
    if entity.body().hp <= 0.0 {
        return true;
    }       
    if !world.bounds.check_collision_recs(&entity.body().frame) {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::RECT_ORIGIN_SQUARE_100, game_engine::{world::World, simple_entity::SimpleEntity}};

    #[test]
    fn can_remove_entities_outside_of_screen() {
        let mut world = World::test();
        
        let mut body = world.entity_factory.build("towerdart");
        body.frame = RECT_ORIGIN_SQUARE_100;
        body.current_speed = 100.0;  
        body.direction =  Vector2::new(-1.0, 0.0);
        world.add_entity(Box::new(SimpleEntity::new(body)));

        assert_eq!(world.entities.borrow().len(), 1);
        world.update(1.0);
        assert_eq!(world.entities.borrow().len(), 0);
    }

    #[test]
    fn can_remove_entities_with_no_hp_left() {
        let mut world = World::test();
        
        let mut body = world.entity_factory.build("towerdart");
        body.frame = RECT_ORIGIN_SQUARE_100;
        body.current_speed = 100.0;   
        body.direction = Vector2::zero();
        body.hp = 0.0;
        world.add_entity(Box::new(SimpleEntity::new(body)));

        assert_eq!(world.entities.borrow().len(), 1);
        world.update(0.1);
        assert_eq!(world.entities.borrow().len(), 0);
    }

    #[test]
    fn can_remove_entities_with_passed_expiration_date() {
        let mut world = World::test();
        
        let mut body = world.entity_factory.build("baseattack");
        body.lifespan = 10.0;
        body.frame = RECT_ORIGIN_SQUARE_100;
        body.current_speed = 0.0;
        body.direction = Vector2::zero();
        world.add_entity(Box::new(SimpleEntity::new(body)));

        assert_eq!(world.entities.borrow().len(), 1);
        world.update(11.0);
        assert_eq!(world.entities.borrow().len(), 0);
    }
}