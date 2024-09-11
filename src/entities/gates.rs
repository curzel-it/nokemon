use crate::game_engine::{entity::Entity, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World};

impl Entity {
    pub fn setup_gate(&mut self) {
        if !self.is_on {
            self.sprite.frame.x += 1;
        }
    }
  
    pub fn gate_flip_on_off(&mut self) {
        if self.is_on {
            self.sprite.frame.x += 1;
            self.is_on = false;
        } else {
            self.sprite.frame.x -= 1;
            self.is_on = true;
        }
    }

    pub fn update_gate(&mut self, world: &World, _: f32) -> Vec<WorldStateUpdate> {  
        if world.creative_mode && world.is_hero_around_and_on_collision_with(&self.frame) {
            return vec![
                WorldStateUpdate::EngineUpdate(
                    EngineStateUpdate::ShowEntityOptions(
                        self.name.clone(), self.id, self.species_id, self.entity_type
                    )
                )
            ];   
        }

        self.is_rigid = self.is_on;

        vec![]
    }
}