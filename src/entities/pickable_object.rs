use crate::game_engine::{entity::Entity, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World};

impl Entity {
    pub fn update_pickable_object(&mut self, world: &World, _: f32) -> Vec<WorldStateUpdate> {        
        self.update_sprite_for_current_direction();
        
        if world.is_hero_around_and_on_collision_with(&self.frame) {
            let vec = vec![
                WorldStateUpdate::EngineUpdate(
                    EngineStateUpdate::ShowEntityOptions(
                        self.name.clone(), self.id, self.entity_type
                    )
                )
            ];
            return vec;  
        }  
        vec![]
    }
}