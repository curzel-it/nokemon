use serde::{Deserialize, Serialize};

use crate::game_engine::{entity::Entity, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World};

use super::{known_species::{SPECIES_NPC_OLD_MAN, SPECIES_NPC_OLD_WOMAN}, species::SpeciesConvertible};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum NpcType {
    OldMan,
    OldWoman,
}

pub type NpcId = u32;

impl SpeciesConvertible for NpcType {
    fn get_species_id(&self) -> u32 {
        match self {
            NpcType::OldMan => SPECIES_NPC_OLD_MAN,
            NpcType::OldWoman => SPECIES_NPC_OLD_WOMAN
        }
    }
}

impl Entity {
    pub fn update_npc(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {  
        self.update_sprite_for_current_direction();
        self.move_linearly(world, time_since_last_update);
        
        if world.is_hero_around_and_on_collision_with(&self.frame) {
            if world.creative_mode {
                let vec = vec![
                    WorldStateUpdate::EngineUpdate(
                        EngineStateUpdate::ShowNpcOptions(
                            self.id, self.name.clone(), self.next_dialogue()
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