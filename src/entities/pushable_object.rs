use crate::{features::linear_movement::would_collide, game_engine::{entity::Entity, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}};

impl Entity {
    pub fn update_pushable(&mut self, world: &World, _: f32) -> Vec<WorldStateUpdate> {  
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
        let hero_direction = world.cached_hero_props.direction;        
        
        if hero.x == self.frame.x && hero.y == self.frame.y {
            if !would_collide(&self.frame, &hero_direction, &world.hitmap) {
                let (dx, dy) = hero_direction.as_col_row_offset();
                self.frame.x += dx;
                self.frame.y += dy;
            } else {
                return vec![WorldStateUpdate::StepBackHero]
            }
        }

        vec![]
    }
}

