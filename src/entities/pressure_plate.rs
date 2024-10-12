use crate::game_engine::{entity::Entity, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World};

impl Entity {
    pub fn setup_pressure_plate(&mut self) {
        if !self.is_related_pressure_plate_down() {
            self.sprite.frame.x += 1;
        }
    }
  
    pub fn update_pressure_plate(&mut self, world: &World, _: f32) -> Vec<WorldStateUpdate> {  
        if world.creative_mode && world.is_hero_around_and_on_collision_with(&self.frame) {
            return vec![
                WorldStateUpdate::EngineUpdate(
                    EngineStateUpdate::ShowEntityOptions(
                        Box::new(self.clone())
                    )
                )
            ];   
        }

        let hero_on_it = world.is_hero_at(self.frame.x, self.frame.y);
        let weight_on_it = world.weights_map[self.frame.y as usize][self.frame.x as usize] > 0;
        let is_pressed = hero_on_it || weight_on_it;
        let is_up = self.is_related_pressure_plate_down();

        if is_up && is_pressed {
            self.sprite.frame.x = self.original_sprite_frame.x + 1;
            vec![WorldStateUpdate::SetPressurePlateState(self.lock_type, true)]
        } else if !is_up && !is_pressed{
            self.sprite.frame.x = self.original_sprite_frame.x;
            vec![WorldStateUpdate::SetPressurePlateState(self.lock_type, false)]
        } else {
            vec![]
        }
    }
}