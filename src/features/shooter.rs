use crate::game_engine::{entity::Entity, state_updates::WorldStateUpdate};

pub trait Shooter: Entity {
    fn time_to_next_shot(&self) -> f32;
    fn inc_time_to_next_shot(&mut self, delta: f32);
    fn reset_time_to_next_shot(&mut self);
    fn create_bullet(&self) -> Box<dyn Entity>;
}

pub fn shoot_stuff(entity: &mut dyn Shooter, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
    entity.inc_time_to_next_shot(-time_since_last_update);
    
    if entity.time_to_next_shot() <= 0.0 {
        entity.reset_time_to_next_shot();
        let bullet = entity.create_bullet();
        return vec![WorldStateUpdate::AddEntity(bullet)];
    }
    vec![]
}

#[macro_export]
macro_rules! impl_shooter {
    ($shooter_struct:ident, $bullet_struct:ident) => {
        impl $crate::features::shooter::Shooter for $shooter_struct {
            fn time_to_next_shot(&self) -> f32 {
                self.time_to_next_shot
            }
            
            fn inc_time_to_next_shot(&mut self, delta: f32) {
                self.time_to_next_shot += delta;
            }
            
            fn reset_time_to_next_shot(&mut self) {
                self.time_to_next_shot = self.time_between_shots;
            }
            
            fn create_bullet(&self) -> Box<dyn $crate::game_engine::entity::Entity> {
                Box::new($bullet_struct::new(self))
            }
        }
    };
}
