use std::any::Any;

use serde::{Deserialize, Serialize};
use crate::{constants::INFINITE_LIFESPAN, game_engine::{entity::Entity, entity_body::EntityBody, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, impl_embodied_entity, utils::{ids::get_next_id, rect::Rect, vector::Vector2d}};

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleEntity {
    body: EntityBody,
    sprite_sheet: u32,
    texture_source_rect: Rect,
}

impl SimpleEntity {
    pub fn new(is_rigid: bool, width: u32, height: u32, sprite_sheet: u32, texture_source_rect: Rect) -> Self {
        Self { 
            body: EntityBody {
                id: get_next_id(),
                frame: Rect::new(0, 0, width, height),
                offset: Vector2d::zero(),
                direction: Vector2d::zero(),
                current_speed: 0.0,
                base_speed: 0.0,
                creation_time: 0.0,
                is_rigid,
                z_index: 0,
                lifespan: INFINITE_LIFESPAN,
            },
            sprite_sheet,
            texture_source_rect,
        }
    }
}

impl_embodied_entity!(SimpleEntity);

impl Entity for SimpleEntity {
    fn update(&mut self, world: &World, _: f32) -> Vec<WorldStateUpdate> {
        let hero = world.cached_hero_props.hittable_frame;
        let hero_direction = world.cached_hero_props.direction;
        
        if !world.creative_mode { return vec![] }
        if !world.keyboard_state.has_confirmation_been_pressed { return vec![] }

        if hero.is_around_and_pointed_at(&self.body.frame, &hero_direction) {      
            return vec![
                WorldStateUpdate::EngineUpdate(
                    EngineStateUpdate::EntityInteraction(
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