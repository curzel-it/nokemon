use crate::game_engine::{world::Game, behaviors::EntityBehavior};

#[derive(Debug)]
pub struct CheckBulletCollisons;

impl CheckBulletCollisons {
    pub fn new() -> Self {
        Self {}
    }
}

impl EntityBehavior for CheckBulletCollisons {
    fn update(&self, entity_id: &u32, world: &mut Game, _: f32) {
        for (bullet_id, damage) in self.check_hits(entity_id, world) {
            self.decrease_hp(entity_id, world, damage);
            self.decrease_hp(&bullet_id, world, damage);
        }
    }
}

impl CheckBulletCollisons {
    fn decrease_hp(&self, entity_id: &u32, world: &mut Game, damage: f32) {
        let entity = world.entities.get_mut(entity_id).unwrap();
        entity.hp -= damage;
    }

    fn check_hits(&self, entity_id: &u32, world: &Game) -> Vec<(u32, f32)> {
        let entity = world.entities.get(entity_id).unwrap();
        if entity.species.is_bullet() { 
            return vec![]; 
        }        

        let mut collisions: Vec<(u32, f32)> = vec![];

        for bullet_id in &world.entity_ids() {
            let bullet = world.entities.get(bullet_id).unwrap();
            if !bullet.species.is_bullet() { continue; }
            if bullet.parent_id == entity.id { continue; }
            if bullet.species.is_enemy == entity.species.is_enemy { continue; }

            if bullet.frame.check_collision_recs(&entity.frame) {
                collisions.push((bullet_id.clone(), bullet.dp));
            }
        }
        return collisions;
    }
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::RECT_ORIGIN_SQUARE_100, game_engine::{world::Game, game_engine::GameEngine, keyboard_events_provider::NoKeyboard}};

    #[test]
    fn can_decrease_hp_of_both_bullet_and_targets_on_hit() {
        let engine = GameEngine::new();
        let mut world = Game::test();
        let nokb = NoKeyboard {};
        
        let mut bullet = world.entity_factory.build("towerdart");
        bullet.id = 1;
        bullet.frame = RECT_ORIGIN_SQUARE_100;
        bullet.dp = 50.0;
        bullet.hp = 60.0;
        bullet.change_direction(Vector2::zero());
        world.add_entity(bullet);      
        
        let mut red = world.entity_factory.build("red");
        red.id = 2;
        red.frame = RECT_ORIGIN_SQUARE_100;
        red.hp = 60.0;
        red.change_direction(Vector2::zero());
        world.add_entity(red);      

        engine.update_rl(&mut world, 0.1, &nokb);
        
        assert_eq!(world.entities.get(&1).unwrap().hp, 10.0);
        assert_eq!(world.entities.get(&2).unwrap().hp, 10.0);
    }

    #[test]
    fn can_decrease_hp_of_both_bullet_and_targets_on_hit_2() {
        let engine = GameEngine::new();
        let mut world = Game::test();
        let nokb = NoKeyboard {};
        
        let mut bullet = world.entity_factory.build("towerdart");
        bullet.id = 1;
        bullet.frame = RECT_ORIGIN_SQUARE_100;
        bullet.dp = 60.0;
        bullet.hp = 50.0;
        bullet.change_direction(Vector2::zero());
        world.add_entity(bullet);      
        
        let mut red = world.entity_factory.build("red");
        red.id = 2;
        red.frame = RECT_ORIGIN_SQUARE_100;
        red.hp = 100.0;
        red.change_direction(Vector2::zero());
        world.add_entity(red);      

        engine.update_rl(&mut world, 0.1, &nokb);
        engine.update_rl(&mut world, 0.1, &nokb);
        
        assert!(world.entities.get(&1).is_none());
        assert_eq!(world.entities.get(&2).unwrap().hp, 40.0);
    }
}