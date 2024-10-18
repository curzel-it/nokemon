use crate::{game_engine::{entity::Entity, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, utils::directions::Direction};

impl Entity {
    pub fn setup_bullet(&mut self) {
        // ...
    }  

    pub fn update_bullet(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {  
        self.update_sprite_for_current_state();
        self.move_linearly(world, time_since_last_update);

        if self.current_speed == 0.0 {
            if world.is_hero_around_and_on_collision_with(&self.frame) {            
                return vec![
                    WorldStateUpdate::EngineUpdate(
                        EngineStateUpdate::ShowEntityOptions(
                            Box::new(self.clone())
                        )
                    )
                ];   
            }
        }

        if self.current_speed == 0.0 || matches!(self.direction, Direction::Unknown) {
            return vec![]
        }

        self.check_hits(world)
    }

    fn check_hits(&self, world: &World) -> Vec<WorldStateUpdate> {
        let hit = world.entities_map[self.frame.y as usize][self.frame.x as usize];

        if hit == 0 || hit == self.id || hit == self.parent_id { 
            vec![] 
        } else {
            vec![WorldStateUpdate::HandleHit(self.id, hit)]
        }
    }
}