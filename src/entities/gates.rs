use crate::game_engine::{entity::Entity, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World};

impl Entity {
    pub fn setup_gate(&mut self, creative_mode: bool) {
        if self.is_related_lock_closed() {
            self.sprite.frame.x += 1;
        }
        if creative_mode {
            self.is_rigid = false;
        }
    }  

    pub fn update_gate(&mut self, world: &World, _: f32) -> Vec<WorldStateUpdate> {  
        if world.creative_mode && world.is_hero_around_and_on_collision_with(&self.frame) {
            return vec![
                WorldStateUpdate::EngineUpdate(
                    EngineStateUpdate::ShowEntityOptions(
                        Box::new(self.clone())
                    )
                )
            ];   
        }

        if self.is_related_lock_closed() {
            self.is_rigid = !world.creative_mode;
            self.sprite.frame.x = self.original_sprite_frame.x;
        } else {
            self.is_rigid = false;
            self.sprite.frame.x = self.original_sprite_frame.x + 1;
        }

        vec![]
    }
}

impl Entity {
    pub fn setup_inverse_gate(&mut self) {
        if !self.is_related_lock_closed() {
            self.sprite.frame.x += 1;
        }
    }  

    pub fn update_inverse_gate(&mut self, world: &World, _: f32) -> Vec<WorldStateUpdate> {  
        if world.creative_mode && world.is_hero_around_and_on_collision_with(&self.frame) {
            return vec![
                WorldStateUpdate::EngineUpdate(
                    EngineStateUpdate::ShowEntityOptions(
                        Box::new(self.clone())
                    )
                )
            ];   
        }

        if !self.is_related_lock_closed() {
            self.is_rigid = true;
            self.sprite.frame.x = self.original_sprite_frame.x;
        } else {
            self.is_rigid = false;
            self.sprite.frame.x = self.original_sprite_frame.x + 1;
        }

        vec![]
    }
}