use crate::{entities::species::EntityType, game_engine::{entity::Entity, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}};

pub type NpcId = u32;

impl Entity {
    pub fn update_npc(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {  
        self.update_sprite_for_current_direction();
        self.move_linearly(world, time_since_last_update);
        
        if world.is_hero_around_and_on_collision_with(&self.frame) {
            if world.creative_mode {
                let vec = vec![
                    WorldStateUpdate::EngineUpdate(
                        EngineStateUpdate::ShowEntityOptions(
                            self.name.clone(), self.id, self.entity_type
                        )
                    )
                ];
                return vec;  
            } else if let Some(dialogue) = self.next_dialogue() {
                return vec![
                    WorldStateUpdate::EngineUpdate(
                        EngineStateUpdate::ShowDialogue(
                            self.id, self.name.clone(), dialogue,
                        )
                    )
                ];
            }             
        }  
        vec![]
    }
}