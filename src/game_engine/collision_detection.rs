use std::collections::HashMap;

use super::{entity::Entity, game::Game};

pub fn compute_collisions(game: &Game) -> HashMap<u32, Vec<u32>> {
    let mut collisions: HashMap<u32, Vec<u32>> = HashMap::new();
    let entities = game.entities.borrow();
    
    let all_entity_ids: Vec<u32> = entities.keys().cloned().collect();
    
    let mut handlers_entity_ids: Vec<u32> = vec![];
    for entity in entities.values() {
        if entity.body().requires_collision_detection {
            handlers_entity_ids.push(entity.id());
        }
    }


    for id1 in handlers_entity_ids {
        if let Some(entity1) = entities.get(&id1) {
            for &id2 in &all_entity_ids {
                if let Some(entity2) = entities.get(&id2) {
                    if is_valid_collision(entity1, entity2) {
                        collisions.entry(id2).or_default().push(id1);
                        collisions.entry(id1).or_default().push(id2);
                    }
                }
            }
        }
    }

    collisions
}

fn is_valid_collision(entity1: &Box<dyn Entity>, entity2: &Box<dyn Entity>) -> bool {
    if !entity1.body().frame.check_collision_recs(&entity2.body().frame) {
        return false;
    }
    if entity1.parent_id() == entity2.id() || entity2.parent_id() == entity1.id() {
        return false;
    }
    if entity1.body().is_ally == entity2.body().is_ally {
        return false;
    }             
    true
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{features::shooter::Shooter, game_engine::{collision_detection::is_valid_collision, entity_body::EmbodiedEntity, game::Game}};

    use super::compute_collisions;

    #[test]
    fn can_detect_collisions() {
        let mut game = Game::test();

        let tower = game.entity_factory.build_tower();
        let mut towerdart = tower.create_bullet(&game.entity_factory);
        towerdart.body_mut().id = 1;
        towerdart.body_mut().direction = Vector2::new(0.0, 0.0);
        towerdart.place_at(0.0, 0.0);
        game.add_entity(towerdart);

        let mut hero = game.entity_factory.build_hero();
        hero.body_mut().id = 2;
        hero.body_mut().direction = Vector2::new(0.0, 0.0);
        hero.place_at(0.0, 0.0);
        game.add_entity(Box::new(hero));

        let entities = game.entities.borrow();
        let do_collide = is_valid_collision(entities.get(&1).unwrap(), entities.get(&2).unwrap());
        assert!(do_collide);
        drop(entities);

        let collisions = compute_collisions(&game);
        assert_eq!(collisions.len(), 2);
    }    
}