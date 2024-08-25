use std::any::Any;

use serde::{Deserialize, Serialize};

use crate::{constants::{INFINITE_LIFESPAN, NO_PARENT, SPRITE_SHEET_BUILDINGS, TILE_SIZE}, game_engine::{entity::Entity, entity_body::EntityBody, entity_factory::get_next_entity_id, state_updates::WorldStateUpdate, world::World}, impl_embodied_entity, utils::{geometry_utils::Insets, rect::Rect, vector::Vector2d}};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildingType {
    House
}

impl BuildingType {
    fn texture_source_rect(&self) -> Rect {
        let (row, w, h) = match self {
            BuildingType::House => (0, 5, 5)
        };
        Rect::new(
            0 as f32, 
            row as f32, 
            w as f32, 
            h as f32
        ).scaled(TILE_SIZE)
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
        let id = get_next_entity_id();
        let frame = building_type.texture_source_rect();

        Self { 
            body: EntityBody {
                id,
                parent_id: NO_PARENT,
                frame: Rect::new(0.0, 0.0, frame.w, frame.h),
                collision_insets: Insets::zero(),
                direction: Vector2d::zero(),
                current_speed: 0.0,
                base_speed: 0.0,
                hp: 1000.0,
                dp: 0.0,
                creation_time: 0.0,
                requires_collision_detection: false,
                is_rigid: true,
                z_index: 0,
                is_ally: true,
                lifespan: INFINITE_LIFESPAN,
            },      
            building_type,
            sprite_sheet: SPRITE_SHEET_BUILDINGS,
        }
    }
}

impl_embodied_entity!(Building);

impl Entity for Building {
    fn update(&mut self, _: &World, _: f32) -> Vec<WorldStateUpdate> {
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