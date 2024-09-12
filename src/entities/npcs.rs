use crate::game_engine::{entity::Entity, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World};

pub type NpcId = u32;

const NO_DIALOG_SHOW_SHOP_INSTEAD: u32 = 3;

impl Entity {
    pub fn update_npc(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {  
        self.update_sprite_for_current_direction();
        self.handle_patrol();
        self.move_linearly(world, time_since_last_update);
        
        if world.is_hero_around_and_on_collision_with(&self.frame) {
            if world.creative_mode {
                let vec = vec![
                    WorldStateUpdate::EngineUpdate(
                        EngineStateUpdate::ShowEntityOptions(
                            self.name.clone(), self.id, self.species_id, self.entity_type
                        )
                    )
                ];
                return vec;  
            } else if let Some(dialogue) = self.next_dialogue() {
                if dialogue.id == NO_DIALOG_SHOW_SHOP_INSTEAD {
                    return vec![
                        WorldStateUpdate::EngineUpdate(
                            EngineStateUpdate::ShowShop
                        )
                    ];
                } else {
                    return vec![
                        WorldStateUpdate::EngineUpdate(
                            EngineStateUpdate::ShowDialogue(
                                self.id, self.name.clone(), dialogue,
                            )
                        )
                    ];
                }
            }             
        }  
        vec![]
    }
}