use std::any::Any;

use serde::{Deserialize, Serialize};
use crate::{constants::{SPRITE_SHEET_TELEPORTER}, dialogues::dialogues::EntityDialogues, game_engine::{entity::Entity, entity_body::EntityBody, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, impl_embodied_entity, utils::{directions::Direction, ids::get_next_id, rect::Rect, vector::Vector2d}};

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
                direction: Direction::Unknown,
                current_speed: 0.0,
                base_speed: 0.0,
                is_rigid: false,
                z_index: 0,
                dialogues: EntityDialogues::empty(),
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

        match hero_direction {
            Direction::Up => hero.x == self.body.frame.x && hero.y == self.body.frame.y + 1,
            Direction::Right => hero.y == self.body.frame.y && hero.x == self.body.frame.x - 1,
            Direction::Down => hero.x == self.body.frame.x && hero.y == self.body.frame.y - 1,
            Direction::Left => hero.y == self.body.frame.y && hero.x == self.body.frame.x + 1,
            Direction::Unknown => false
        }
    }

    fn engine_update_push_world(&self) -> WorldStateUpdate {
        WorldStateUpdate::EngineUpdate(
            EngineStateUpdate::SwitchWorld(self.destination)
        )
    }
}
