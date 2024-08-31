use std::any::Any;

use serde::{Deserialize, Serialize};
use crate::{constants::{INFINITE_LIFESPAN, SPRITE_SHEET_TELEPORTER}, game_engine::{entity::Entity, entity_body::EntityBody, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, impl_embodied_entity, utils::{ids::get_next_id, rect::Rect, vector::Vector2d}};

#[derive(Debug, Serialize, Deserialize)]
pub struct Teleporter {
    body: EntityBody,
    pub destination: u32,
    creative_mode: bool
}

impl Teleporter {
    pub fn new(destination: u32) -> Self {
        Self { 
            body: EntityBody {
                id: get_next_id(),
                frame: Rect::new(0, 0, 1, 1),
                offset: Vector2d::zero(),
                direction: Vector2d::zero(),
                current_speed: 0.0,
                base_speed: 0.0,
                creation_time: 0.0,
                is_rigid: false,
                z_index: 0,
                lifespan: INFINITE_LIFESPAN,
            },
            destination,
            creative_mode: false
        }
    }
}

impl_embodied_entity!(Teleporter);

impl Entity for Teleporter {
    fn update(&mut self, world: &World, _: f32) -> Vec<WorldStateUpdate> {
        self.creative_mode = world.creative_mode;

        if self.should_teleport(world) {
            return vec![self.engine_update_push_world()];
        }
        vec![]
    }

    fn texture_source_rect(&self) -> Rect {
        let row = if self.creative_mode { 0 } else { 1 };
        Rect::new(0, row, 1, 1)
    }

    fn sprite_sheet(&self) -> u32 {
        SPRITE_SHEET_TELEPORTER
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Teleporter {
    fn should_teleport(&self, world: &World) -> bool {
        let hero = world.cached_hero_props.hittable_frame;
        let hero_direction = world.cached_hero_props.direction;
        let hero_speed = world.cached_hero_props.speed;

        if !world.is_any_arrow_key_down { return false }
        if hero_speed <= 0.0 { return false }

        if hero.y == self.body.frame.y {
            if hero.x == self.body.frame.x + 1 {
                return hero_direction.x < 0.0
            }
            if hero.x == self.body.frame.x - 1 {
                return hero_direction.x > 0.0
            }
        }
        if hero.x == self.body.frame.x {
            if hero.y == self.body.frame.y + 1 {
                return hero_direction.y < 0.0
            } 
            if hero.y == self.body.frame.y - 1 {
                return hero_direction.y > 0.0
            }
        }
        false
    }

    fn engine_update_push_world(&self) -> WorldStateUpdate {
        WorldStateUpdate::EngineUpdate(
            EngineStateUpdate::SwitchWorld(self.destination)
        )
    }
}
