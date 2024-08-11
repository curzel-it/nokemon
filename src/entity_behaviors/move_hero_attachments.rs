use crate::game_engine::{world::Game, behaviors::EntityBehavior};

#[derive(Debug)]
pub struct MoveHeroAttachments;

impl MoveHeroAttachments {
    pub fn new() -> Self {
        Self {}
    }
}

impl EntityBehavior for MoveHeroAttachments {
    fn update(&self, entity_id: &u32, world: &mut Game, _: f32) {
        let hero_frame = world.hero_frame();
        let entity = world.entities.get_mut(entity_id).unwrap();        
        
        if entity.species.is_hero_attachment() {
            entity.place_center_of(hero_frame)
        }
    }
}