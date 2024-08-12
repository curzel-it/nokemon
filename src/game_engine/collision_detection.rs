use std::collections::HashMap;

use raylib::math::Rectangle;

use super::{entity::Entity, world::World};

pub struct Collision {
    pub other_id: u32,
    pub other_was_rigid: bool,
    pub area: Rectangle
}

pub fn compute_collisions(world: &World) -> HashMap<u32, Vec<Collision>> {
    let mut collisions: HashMap<u32, Vec<Collision>> = HashMap::new();
    let visible_entities = &world.visible_entities;
    let entities = world.entities.borrow();
        
    let mut handlers_entity_ids: Vec<u32> = vec![];
    for entity in entities.values() {
        if entity.body().requires_collision_detection {
            handlers_entity_ids.push(entity.id());
        }
    }

    for id1 in handlers_entity_ids {
        if let Some(entity1) = entities.get(&id1) {
            for &id2 in visible_entities {
                if let Some(entity2) = entities.get(&id2) {
                    if let Some(area) = collision_area(entity1, entity2) {
                        let (first, second) = collisions_pair(entity1, entity2, area);
                        collisions.entry(id1).or_default().push(first);
                        collisions.entry(id2).or_default().push(second);
                    }
                }
            }
        }
    }

    collisions
}

fn collision_area(entity1: &Box<dyn Entity>, entity2: &Box<dyn Entity>) -> Option<Rectangle> {
    if entity1.parent_id() == entity2.id() || entity2.parent_id() == entity1.id() {
        return None;
    }
    if entity1.body().is_ally == entity2.body().is_ally {
        return None;
    }             
    entity1.body().frame.get_collision_rec(&entity2.body().frame)
}

fn collisions_pair(first: &Box<dyn Entity>, second: &Box<dyn Entity>, area: Rectangle) -> (Collision, Collision) {
    let first_collision = Collision { 
        other_id: second.id(), 
        other_was_rigid: second.body().is_rigid, 
        area 
    };
    let second_collision = Collision { 
        other_id: first.id(), 
        other_was_rigid: first.body().is_rigid, 
        area 
    };
    (first_collision, second_collision)
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::RECT_ORIGIN_SQUARE_100, features::shooter::Shooter, game_engine::{entity::Entity, entity_body::EmbodiedEntity, visible_entities::compute_visible_entities, world::World}};

    use super::{collision_area, compute_collisions};

    fn is_valid_collision(entity1: &Box<dyn Entity>, entity2: &Box<dyn Entity>) -> bool {
        collision_area(entity1, entity2).is_some()
    }

    #[test]
    fn can_detect_collisions_of_entities_inside_camera_viewport() {
        let mut world = World::test();
        world.camera_viewport = RECT_ORIGIN_SQUARE_100;

        let tower = world.entity_factory.build_tower();
        let mut towerdart = tower.create_bullet(&world.entity_factory);
        towerdart.body_mut().id = 1;
        towerdart.body_mut().direction = Vector2::new(0.0, 0.0);
        towerdart.place_at(0.0, 0.0);
        world.add_entity(towerdart);

        let mut hero = world.entity_factory.build_hero();
        hero.body_mut().id = 2;
        hero.body_mut().direction = Vector2::new(0.0, 0.0);
        hero.place_at(0.0, 0.0);
        world.add_entity(Box::new(hero));

        world.visible_entities = compute_visible_entities(&world);

        let entities = world.entities.borrow();
        let do_collide = is_valid_collision(entities.get(&1).unwrap(), entities.get(&2).unwrap());
        assert!(do_collide);
        drop(entities);

        let collisions = compute_collisions(&world);
        assert_eq!(collisions.len(), 2);
    }    

    #[test]
    fn can_not_detect_collisions_of_entities_outside_camera_viewport() {
        let mut world = World::test();
        world.camera_viewport = RECT_ORIGIN_SQUARE_100;

        let tower = world.entity_factory.build_tower();
        let mut towerdart = tower.create_bullet(&world.entity_factory);
        towerdart.body_mut().id = 1;
        towerdart.body_mut().direction = Vector2::new(0.0, 0.0);
        towerdart.place_at(2000.0, 0.0);
        world.add_entity(towerdart);

        let mut hero = world.entity_factory.build_hero();
        hero.body_mut().id = 2;
        hero.body_mut().direction = Vector2::new(0.0, 0.0);
        hero.place_at(2000.0, 0.0);
        world.add_entity(Box::new(hero));

        world.visible_entities = compute_visible_entities(&world);

        let entities = world.entities.borrow();
        let do_collide = is_valid_collision(entities.get(&1).unwrap(), entities.get(&2).unwrap());
        assert!(do_collide);
        drop(entities);

        let collisions = compute_collisions(&world);
        assert_eq!(collisions.len(), 0);
    }    
}