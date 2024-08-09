use std::collections::HashMap;

use super::{entity::Entity, game::Game};

pub fn compute_collisions(game: &Game) -> HashMap<u32, Vec<u32>> {
    let mut collisions: HashMap<u32, Vec<u32>> = HashMap::new();
    let entities = game.entities.borrow();
    let entity_ids: Vec<u32> = entities.keys().cloned().collect();

    for (i, &id1) in entity_ids.iter().enumerate() {
        if let Some(entity1) = entities.get(&id1) {
            for &id2 in entity_ids.iter().skip(i + 1) {
                if let Some(entity2) = entities.get(&id2) {
                    if is_valid_collision(entity1, entity2) {
                        if !entity2.species().is_bullet {
                            collisions.entry(id1).or_insert_with(Vec::new).push(id2);
                        }
                        collisions.entry(id2).or_insert_with(Vec::new).push(id1);
                    }
                }
            }
        }
    }

    collisions
}

fn is_valid_collision(entity1: &Box<dyn Entity>, entity2: &Box<dyn Entity>) -> bool {
    if !entity1.frame().check_collision_recs(&entity2.frame()) {
        return false;
    }
    if entity1.parent_id() == entity2.id() || entity2.parent_id() == entity1.id() {
        return false;
    }
    if entity1.species().is_enemy == entity2.species().is_enemy {
        return false;
    }             
    true
}