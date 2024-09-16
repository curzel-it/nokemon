use crate::game_engine::{entity::Entity, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World};

impl Entity {
    pub fn handle_melee_attack(&mut self, world: &World) -> Vec<WorldStateUpdate> {
        if self.is_dying {
            return vec![]
        }

        let hero = world.cached_hero_props.hittable_frame;
        let x = self.frame.x;
        let y = self.frame.y + if self.frame.h > 1 { 1 } else { 0 };
        
        if x == hero.x && y == hero.y {
            return vec![WorldStateUpdate::EngineUpdate(EngineStateUpdate::DeathScreen)]
        }
        vec![]
    }
}