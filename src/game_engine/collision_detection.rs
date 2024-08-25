use std::collections::HashMap;

use uuid::Uuid;

use crate::utils::rect::Rect;

use super::{entity::Entity, world::World};

#[derive(Debug, Copy, Clone)]
pub struct Collision {
    pub other_id: Uuid,
    pub other_was_rigid: bool,
    pub are_same_faction: bool,
    pub overlapping_area: Rect,
    pub center_x: u32,
    pub center_y: u32,
}

pub fn compute_collisions(world: &World) -> HashMap<Uuid, Vec<Collision>> {
    let mut collisions: HashMap<Uuid, Vec<Collision>> = HashMap::new();
    let visible_entities = &world.visible_entities;
    let entities = world.entities.borrow();
/*
    for &id1 in visible_entities {
        if let Some(entity1) = entities.get(&id1) {
            for &id2 in visible_entities {
                if let Some(entity2) = entities.get(&id2) {
                    if let Some(overlapping_area) = collision_area(entity1, entity2) {
                        let (first, second) = collisions_pair(entity1, entity2, overlapping_area);
                        collisions.entry(id1).or_default().push(first);
                        collisions.entry(id2).or_default().push(second);
                    }
                }
            }
        }
    } */

    collisions
}
/*
fn collision_area(entity1: &Box<dyn Entity>, entity2: &Box<dyn Entity>) -> Option<Rect> {
    if !entity1.body().requires_collision_detection {
        return None;
    }
    if !entity2.body().is_rigid {
        return None;
    }
    if entity1.id() == entity2.id() {
        return None;
    }
    if entity1.parent_id() == entity2.id() {
        return None;
    }
    if entity2.parent_id() == entity1.id() {
        return None;
    }
    entity1.body().frame.collision_area_with_rect(&entity2.body().frame)
}

fn collisions_pair(first: &Box<dyn Entity>, second: &Box<dyn Entity>, overlapping_area: Rect) -> (Collision, Collision) {
    let center_x = overlapping_area.x + overlapping_area.w / 2;
    let center_y = overlapping_area.y + overlapping_area.h / 2;
    let are_same_faction = first.body().is_ally == second.body().is_ally;

    let first_collision = Collision { 
        other_id: second.id(), 
        other_was_rigid: second.body().is_rigid, 
        are_same_faction,
        overlapping_area,
        center_x, 
        center_y
    };
    let second_collision = Collision { 
        other_id: first.id(), 
        other_was_rigid: first.body().is_rigid, 
        are_same_faction,
        overlapping_area ,
        center_x, 
        center_y
    };
    
    (first_collision, second_collision)
}
 */
#[cfg(test)]
mod tests {
    use crate::{entities::{building::{Building, BuildingType}, hero::Hero}, game_engine::{entity::Entity, entity_body::EmbodiedEntity, visible_entities::compute_visible_entities, world::World}, utils::{rect::Rect, vector::Vector2d}, worlds::constants::WORLD_ID_DEMO};
/*
    use super::{collision_area, compute_collisions};

    fn is_valid_collision(entity1: &Box<dyn Entity>, entity2: &Box<dyn Entity>) -> bool {
        collision_area(entity1, entity2).is_some()
    }

    #[test]
    fn can_detect_collisions_of_entities_inside_camera_viewport() {
        let mut world = World::new(WORLD_ID_DEMO);

        let mut tower = Building::new(BuildingType::House);
        let tower_id = tower.id();
        tower.body_mut().requires_collision_detection = true;
        tower.body_mut().direction = Vector2d::zero();
        tower.place_at(0, 0);
        world.add_entity(Box::new(tower));

        let mut hero = Hero::new();
        let hero_id = hero.id();
        hero.body_mut().direction = Vector2d::zero();
        hero.place_at(0, 0);
        world.add_entity(Box::new(hero));

        world.visible_entities = compute_visible_entities(&world, &Rect::square_from_origin(100));

        let entities = world.entities.borrow();
        let do_collide = is_valid_collision(
            entities.get(&tower_id).unwrap(), 
            entities.get(&hero_id).unwrap()
        );
        assert!(do_collide);
        drop(entities);

        let collisions = compute_collisions(&world);
        assert_eq!(collisions.len(), 2);
    }    

    #[test]
    fn can_not_detect_collisions_of_entities_outside_camera_viewport() {
        let mut world = World::new(WORLD_ID_DEMO);

        let mut tower = Building::new(BuildingType::House);
        tower.body_mut().requires_collision_detection = true;
        let tower_id = tower.id();
        tower.body_mut().direction = Vector2d::zero();
        tower.place_at(2000, 0);
        world.add_entity(Box::new(tower));

        let mut hero = Hero::new();
        let hero_id = hero.id();
        hero.body_mut().direction = Vector2d::zero();
        hero.place_at(2000, 0);
        world.add_entity(Box::new(hero));

        world.visible_entities = compute_visible_entities(&world, &Rect::square_from_origin(100));

        let entities = world.entities.borrow();
        let do_collide = is_valid_collision(
            entities.get(&tower_id).unwrap(), 
            entities.get(&hero_id).unwrap()
        );
        assert!(do_collide);
        drop(entities);

        let collisions = compute_collisions(&world);
        assert_eq!(collisions.len(), 0);
    }    */
}