use crate::game_engine::{entity::Entity, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World};

impl Entity {
    pub fn update_pickable_object(&mut self, world: &World, _: f32) -> Vec<WorldStateUpdate> {        
        self.update_sprite_for_current_direction();
        
        if world.is_hero_around_and_on_collision_with(&self.frame) {
            if world.creative_mode {
                let vec = vec![
                    WorldStateUpdate::EngineUpdate(
                        EngineStateUpdate::ShowEntityOptions(
                            self.id
                        )
                    )
                ];
                return vec;  
            } else {
                if world.has_confirmation_key_been_pressed {
                    return vec![
                        WorldStateUpdate::EngineUpdate(EngineStateUpdate::AddToInventory(self.species_id)),
                        WorldStateUpdate::RemoveEntity(self.id),
                        WorldStateUpdate::EngineUpdate(EngineStateUpdate::SaveGame),
                    ];
                }
            }             
        }  
        vec![]
    }
}