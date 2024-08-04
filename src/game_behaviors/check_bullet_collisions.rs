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
        if let Some((bullet_id, damage)) = self.check_hit(entity_id, game) {
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

    fn check_hit(&self, entity_id: &u32, game: &Game) -> Option<(u32, f32)> {
        let entity = game.entities.get(entity_id).unwrap();
        if entity.species.is_bullet { 
            return None; 
        }        

        for bullet_id in &game.bullets {
            let bullet = game.entities.get(bullet_id).unwrap();

            if bullet.parent_id == entity.id {
                return None;
            }
            if bullet.frame.check_collision_recs(&entity.frame) {
                return Some((bullet_id.clone(), bullet.dp));
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::RECT_ORIGIN_SQUARE_100, game_engine::{game::Game, game_engine::GameEngine}};

    #[test]
    fn can_decrease_hp_of_both_bullet_and_targets_on_hit() {
        let engine = GameEngine::new();
        let mut game = Game::test();
        
        let mut bullet = game.entity_factory.build("towerdart");
        bullet.id = 1;
        bullet.frame = RECT_ORIGIN_SQUARE_100;
        bullet.dp = 50.0;
        bullet.hp = 60.0;
        bullet.change_direction(Vector2::zero());
        game.add_entity(bullet);      
        
        let mut ape = game.entity_factory.build("ape");
        ape.id = 2;
        ape.frame = RECT_ORIGIN_SQUARE_100;
        ape.hp = 60.0;
        ape.change_direction(Vector2::zero());
        game.add_entity(ape);      

        engine.update(&mut game, 0.1);
        
        assert_eq!(game.entities.get(&1).unwrap().hp, 10.0);
        assert_eq!(game.entities.get(&2).unwrap().hp, 10.0);
    }
    #[test]
    fn can_decrease_hp_of_both_bullet_and_targets_on_hit_2() {
        let engine = GameEngine::new();
        let mut game = Game::test();
        
        let mut bullet = game.entity_factory.build("towerdart");
        bullet.id = 1;
        bullet.frame = RECT_ORIGIN_SQUARE_100;
        bullet.dp = 60.0;
        bullet.hp = 50.0;
        bullet.change_direction(Vector2::zero());
        game.add_entity(bullet);      
        
        let mut ape = game.entity_factory.build("ape");
        ape.id = 2;
        ape.frame = RECT_ORIGIN_SQUARE_100;
        ape.hp = 100.0;
        ape.change_direction(Vector2::zero());
        game.add_entity(ape);      

        engine.update(&mut game, 0.1);
        
        assert!(game.entities.get(&1).is_none());
        assert_eq!(game.entities.get(&2).unwrap().hp, 40.0);
    }
}