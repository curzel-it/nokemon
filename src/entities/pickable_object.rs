use crate::game_engine::{entity::Entity, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World};

impl Entity {
    pub fn update_pickable_object(&mut self, world: &World, _: f32) -> Vec<WorldStateUpdate> {              
        if world.is_hero_around_and_on_collision_with(&self.frame) || world.is_hero_at(self.frame.x, self.frame.y) {            
            vec![
                WorldStateUpdate::EngineUpdate(
                    EngineStateUpdate::ShowEntityOptions(
                        Box::new(self.clone())
                    )
                )
            ]
        } else {
            vec![]
        }
    }
}