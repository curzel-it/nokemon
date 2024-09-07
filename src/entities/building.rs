use std::any::Any;

use serde::{Deserialize, Serialize};
use crate::{constants::{INFINITE_LIFESPAN, SPRITE_SHEET_BUILDINGS}, game_engine::{entity::Entity, entity_body::EntityBody, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, impl_embodied_entity, utils::{directions::Direction, ids::get_next_id, rect::Rect, vector::Vector2d}};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildingType {
    House(i32),
    HouseTwoFloors(i32),
}

impl BuildingType {
    fn texture_source_rect(&self) -> Rect {
        let (x, y, w, h) = match self {
            BuildingType::House(variant) => (0, 5 * variant + 1, 5, 4),
            BuildingType::HouseTwoFloors(variant) => (5, 5 * variant, 5, 5),
        };
        Rect::new(x, y, w, h)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Building {
    body: EntityBody,
    building_type: BuildingType
}

impl Building {
    pub fn new(building_type: BuildingType) -> Self {
        let id = get_next_id();
        let frame = building_type.texture_source_rect();

        Self { 
            body: EntityBody {
                id,
                frame: Rect::new(0, 0, frame.w, frame.h),
                offset: Vector2d::zero(),
                direction: Direction::Unknown,
                current_speed: 0.0,
                base_speed: 0.0,
                creation_time: 0.0,
                is_rigid: true,
                z_index: 0,
                lifespan: INFINITE_LIFESPAN,
                dialogue: None
            },      
            building_type
        }
    }
}

impl_embodied_entity!(Building);

impl Entity for Building {
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
        self.building_type.texture_source_rect()
    }

    fn sprite_sheet(&self) -> u32 {
        SPRITE_SHEET_BUILDINGS
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}