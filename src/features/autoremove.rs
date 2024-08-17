
use crate::{constants::INFINITE_LIFESPAN, game_engine::{entity::Entity, world::World, state_updates::WorldStateUpdate}};

pub fn remove_automatically(entity: &dyn Entity, world: &World) -> Vec<WorldStateUpdate> {
    if should_remove(world, entity) {
        return vec![WorldStateUpdate::RemoveEntity(entity.id())];
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
    false
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::RECT_ORIGIN_SQUARE_100, game_engine::{entity_body::EntityBody, simple_entity::SimpleEntity, world::World}};

    #[test]
    fn can_remove_entities_with_no_hp_left() {
        let mut world = World::new();
        
        let mut body = EntityBody::test();
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
        let mut world = World::new();
        
        let mut body = EntityBody::test();
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