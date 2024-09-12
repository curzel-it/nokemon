use crate::{game_engine::{state_updates::WorldStateUpdate, world::World}, utils::rect::Rect};

pub struct CreepSpawner {
    last_hero_frame: Rect
}

impl CreepSpawner {
    pub fn new() -> Self {
        CreepSpawner {
            last_hero_frame: Rect::square_from_origin(1)
        }
    }

    pub fn update(&mut self, world: &World, _: f32) -> Vec<WorldStateUpdate> {
        if self.last_hero_frame != world.cached_hero_props.hittable_frame {
            self.last_hero_frame = world.cached_hero_props.hittable_frame;
        }
        
        vec![]
    }
}