use crate::game_engine::{game::Game, behaviors::EntityBehavior};

#[derive(Debug)]
pub struct CheckBulletCollisons;

impl CheckBulletCollisons {
    pub fn new() -> Self {
        Self {}
    }
}

impl EntityBehavior for CheckBulletCollisons {
    fn update(&self, entity_id: &u32, game: &mut Game, _: f32) {
        for (bullet_id, damage) in self.check_hits(entity_id, game) {
            self.decrease_hp(entity_id, game, damage);
            self.decrease_hp(&bullet_id, game, damage);
        }
    }
}

impl CheckBulletCollisons {
    fn decrease_hp(&self, entity_id: &u32, game: &mut Game, damage: f32) {
        let entity = game.entities.get_mut(entity_id).unwrap();
        entity.hp -= damage;
    }

    fn check_hits(&self, entity_id: &u32, game: &Game) -> Vec<(u32, f32)> {
        let entity = game.entities.get(entity_id).unwrap();
        if entity.species.is_bullet() { 
            return vec![]; 
        }        

        let mut collisions: Vec<(u32, f32)> = vec![];

        for bullet_id in &game.entity_ids() {
            let bullet = game.entities.get(bullet_id).unwrap();
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

    use crate::{constants::RECT_ORIGIN_SQUARE_100, game_engine::{game::Game, game_engine::GameEngine, keyboard_events_provider::NoKeyboard}};

    #[test]
    fn can_decrease_hp_of_both_bullet_and_targets_on_hit() {
        let engine = GameEngine::new();
        let mut game = Game::test();
        let nokb = NoKeyboard {};
        
        let mut bullet = game.entity_factory.build("towerdart");
        bullet.id = 1;
        bullet.frame = RECT_ORIGIN_SQUARE_100;
        bullet.dp = 50.0;
        bullet.hp = 60.0;
        bullet.change_direction(Vector2::zero());
        game.add_entity(bullet);      
        
        let mut red = game.entity_factory.build("red");
        red.id = 2;
        red.frame = RECT_ORIGIN_SQUARE_100;
        red.hp = 60.0;
        red.change_direction(Vector2::zero());
        game.add_entity(red);      

        engine.update(&mut game, 0.1, &nokb);
        
        assert_eq!(game.entities.get(&1).unwrap().hp, 10.0);
        assert_eq!(game.entities.get(&2).unwrap().hp, 10.0);
    }

    #[test]
    fn can_decrease_hp_of_both_bullet_and_targets_on_hit_2() {
        let engine = GameEngine::new();
        let mut game = Game::test();
        let nokb = NoKeyboard {};
        
        let mut bullet = game.entity_factory.build("towerdart");
        bullet.id = 1;
        bullet.frame = RECT_ORIGIN_SQUARE_100;
        bullet.dp = 60.0;
        bullet.hp = 50.0;
        bullet.change_direction(Vector2::zero());
        game.add_entity(bullet);      
        
        let mut red = game.entity_factory.build("red");
        red.id = 2;
        red.frame = RECT_ORIGIN_SQUARE_100;
        red.hp = 100.0;
        red.change_direction(Vector2::zero());
        game.add_entity(red);      

        engine.update(&mut game, 0.1, &nokb);
        engine.update(&mut game, 0.1, &nokb);
        
        assert!(game.entities.get(&1).is_none());
        assert_eq!(game.entities.get(&2).unwrap().hp, 40.0);
    }
}