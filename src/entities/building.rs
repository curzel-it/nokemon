use std::any::Any;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{constants::{INFINITE_LIFESPAN, SPRITE_SHEET_BUILDINGS}, game_engine::{entity::Entity, entity_body::EntityBody, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, impl_embodied_entity, utils::{rect::Rect, vector::Vector2d}};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildingType {
    House
}

impl BuildingType {
    fn texture_source_rect(&self) -> Rect {
        let (row, w, h) = match self {
            BuildingType::House => (0, 5, 5)
        };
        Rect::new(0, row, w, h)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Building {
    body: EntityBody,
    building_type: BuildingType,
    sprite_sheet: u32
}

impl Building {
    pub fn new(building_type: BuildingType) -> Self {
        let id = Uuid::new_v4();
        let frame = building_type.texture_source_rect();

        Self { 
            body: EntityBody {
                id,
                frame: Rect::new(0, 0, frame.w, frame.h),
                offset: Vector2d::zero(),
                direction: Vector2d::zero(),
                current_speed: 0.0,
                base_speed: 0.0,
                creation_time: 0.0,
                is_rigid: true,
                z_index: 0,
                lifespan: INFINITE_LIFESPAN,
            },      
            building_type,
            sprite_sheet: SPRITE_SHEET_BUILDINGS,
        }
    }
}

impl_embodied_entity!(Building);

impl Entity for Building {
    fn update(&mut self, world: &World, _: f32) -> Vec<WorldStateUpdate> {
        let hero = world.cached_hero_props.hittable_frame;
        let hero_direction = world.cached_hero_props.direction;
        
        if !world.creative_mode { return vec![] }
        if !world.keyboard_state.has_confirmation_been_pressed { return vec![] }

        if hero.is_around_and_pointed_at(&hero_direction, &self.body.frame) {        
            return vec![
                WorldStateUpdate::EngineUpdate(
                    EngineStateUpdate::BuildingInteraction(
                        self.body.id
                    )
                )
            ];   
        } 
        vec![]
    }

    fn texture_source_rect(&self) -> Rect {
        self.building_type.texture_source_rect()
    }

    fn sprite_sheet(&self) -> u32 {
        self.sprite_sheet
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}