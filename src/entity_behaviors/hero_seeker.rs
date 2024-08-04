use crate::{game_engine::{behaviors::EntityBehavior, game::Game}, utils::vector_utils::dumb_direction_vector};

#[derive(Debug)]
pub struct HeroSeeker;

impl HeroSeeker {
    pub fn new() -> Self {
        Self {}
    }
}

impl EntityBehavior for HeroSeeker {
    fn update(&self, entity_id: &u32, game: &mut Game, _: f32) {
        let hero = game.hero_position();
        let entity = game.entities.get_mut(entity_id).unwrap();
        
        if entity.species.is_enemy {
            entity.direction = dumb_direction_vector(entity.frame.x, entity.frame.y, hero.x, hero.y);
        }
    }
}