use crate::{features::linear_movement::{would_collide, would_over_weight}, game_engine::{entity::Entity, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, utils::directions::Direction};

impl Entity {
    pub fn update_pushable(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {  
        if world.creative_mode && world.is_hero_around_and_on_collision_with(&self.frame) {
            return vec![
                WorldStateUpdate::EngineUpdate(
                    EngineStateUpdate::ShowEntityOptions(
                        Box::new(self.clone())
                    )
                )
            ];   
        }

        let hero = world.cached_hero_props.hittable_frame;
        let hero_direction = world.cached_hero_props.direction;       
        let hero_offset = world.cached_hero_props.offset;        
        let non_zero_offset = hero_offset.x != 0.0 || hero_offset.y != 0.0;
        
        if non_zero_offset {
            let is_around = match hero_direction {
                Direction::Up => hero.y == self.frame.y + self.frame.h && hero.x >= self.frame.x && hero.x < self.frame.x + self.frame.w,
                Direction::Right => hero.x == self.frame.x.saturating_sub(1) && hero.y >= self.frame.y && hero.y < self.frame.y + self.frame.h,
                Direction::Down => hero.y == self.frame.y && hero.x >= self.frame.x && hero.x < self.frame.x + self.frame.w,
                Direction::Left => hero.x == self.frame.x + self.frame.w && hero.y >= self.frame.y && hero.y < self.frame.y + self.frame.h,
                Direction::Unknown => false,
            };
            if is_around {
                let hits = would_collide(&self.frame, &hero_direction, &world.hitmap);
                let weights = would_over_weight(&self.frame, &hero_direction, &world.weights_map);
                
                if hits || weights {
                    return vec![WorldStateUpdate::StopHeroMovement]
                } else {
                    self.direction = hero_direction;
                    self.current_speed = 1.2 * world.cached_hero_props.speed;
                    self.move_linearly(world, time_since_last_update);
                }
            }
        }

        vec![]
    }
}

