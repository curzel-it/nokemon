use crate::game_engine::{entity::Entity, state_updates::{EngineStateUpdate, WorldStateUpdate}, storage::set_value_for_key, world::World};

impl Entity {
    pub fn setup_pressure_plate(&mut self) {
        if !self.is_related_lock_closed() {
            self.sprite.frame.x += 1;
        }
    }
  
    pub fn update_pressure_plate(&mut self, world: &World, _: f32) -> Vec<WorldStateUpdate> {  
        if world.creative_mode && world.is_hero_around_and_on_collision_with(&self.frame) {
            return vec![
                WorldStateUpdate::EngineUpdate(
                    EngineStateUpdate::ShowEntityOptions(
                        self.name.clone(), self.id, self.species_id, self.entity_type
                    )
                )
            ];   
        }

        let hero = world.cached_hero_props.hittable_frame;
        let hero_on_it = hero.x == self.frame.x && hero.y == self.frame.y;
        let weight_on_it = world.weight_map[self.frame.y as usize][self.frame.x as usize] > 0;
        let is_pressed = hero_on_it || weight_on_it;
        let is_up = self.is_related_lock_closed();

        if is_up && is_pressed {
            set_value_for_key(self.lock_type.pressure_plate(), 1);
            self.sprite.frame.x = self.original_sprite_frame.x + 1;
        } else if !is_up && !is_pressed{
            set_value_for_key(self.lock_type.pressure_plate(), 0);
            self.sprite.frame.x = self.original_sprite_frame.x;
        }
        vec![]
    }
}