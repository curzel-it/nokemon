use crate::game_engine::{entity::Entity, world_state_update::WorldStateUpdate};

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
