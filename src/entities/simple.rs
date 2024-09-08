use std::any::Any;

use serde::{Deserialize, Serialize};
use crate::{dialogues::dialogues::EntityDialogues, game_engine::{entity::Entity, entity_body::EntityBody, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, impl_embodied_entity, utils::{directions::Direction, ids::get_next_id, rect::Rect, vector::Vector2d}};

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleEntity {
    body: EntityBody,
    sprite_sheet: u32,
    texture_source_rect: Rect,
}

impl SimpleEntity {
    pub fn new(is_rigid: bool, width: i32, height: i32, sprite_sheet: u32, texture_source_rect: Rect) -> Self {
        Self { 
            body: EntityBody {
                id: get_next_id(),
                frame: Rect::new(0, 0, width, height),
                offset: Vector2d::zero(),
                direction: Direction::Unknown,
                current_speed: 0.0,
                base_speed: 0.0,
                is_rigid,
                z_index: 0,
                dialogues: EntityDialogues::empty(),
            },
            sprite_sheet,
            texture_source_rect,
        }
    }
}

impl_embodied_entity!(SimpleEntity);

impl Entity for SimpleEntity {
    fn update(&mut self, world: &World, _: f32) -> Vec<WorldStateUpdate> {
        if world.creative_mode && world.is_hero_around_and_on_collision_with(&self.body.frame) {
            return vec![
                WorldStateUpdate::EngineUpdate(
                    EngineStateUpdate::ShowEntityOptions(
                        self.body.id
                    )
                )
            ];   
        }
        vec![]
    }

    fn texture_source_rect(&self) -> Rect {
        self.texture_source_rect
    }

    fn sprite_sheet(&self) -> u32 {
        self.sprite_sheet
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}