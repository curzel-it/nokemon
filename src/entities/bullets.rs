use crate::game_engine::{entity::Entity, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World};

impl Entity {
    pub fn setup_bullet(&mut self) {
        self.setup_patrol()
    }  

    pub fn update_bullet(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {  
        self.update_sprite_for_current_direction();
        self.handle_patrol();
        self.move_linearly(world, time_since_last_update);

        if world.creative_mode && world.is_hero_around_and_on_collision_with(&self.frame) {
            return vec![
                WorldStateUpdate::EngineUpdate(
                    EngineStateUpdate::ShowEntityOptions(
                        Box::new(self.clone())
                    )
                )
            ];   
        }
        vec![]
    }
}