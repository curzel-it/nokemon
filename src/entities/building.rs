use std::any::Any;

use serde::{Deserialize, Serialize};

use crate::{constants::{INFINITE_LIFESPAN, NO_PARENT, SPRITE_SHEET_BUILDINGS, TILE_SIZE, TILE_SIZE_HALF}, game_engine::{entity::Entity, entity_body::EntityBody, entity_factory::get_next_building_id, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, impl_embodied_entity, utils::{geometry_utils::Insets, rect::Rect, vector::Vector2d}};

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

    fn door_frame(&self, x: f32, y: f32) -> Rect {
        let (row, col, w, h) = match self {
            BuildingType::House => (4, 3, 1, 1)
        };
        Rect::new(
            x + col as f32 * TILE_SIZE, 
            y + row as f32 * TILE_SIZE, 
            w as f32 * TILE_SIZE, 
            h as f32 * TILE_SIZE
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Building {
    body: EntityBody,
    building_type: BuildingType,
    interior_id: u32,
    sprite_sheet: u32
}

impl Building {
    pub fn new(building_type: BuildingType) -> Self {
        Self::new_with_destination(building_type, None)
    }

    fn new_with_destination(building_type: BuildingType, interior_id: Option<u32>) -> Self {
        let id = get_next_building_id();
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
            interior_id: interior_id.unwrap_or(id),
            sprite_sheet: SPRITE_SHEET_BUILDINGS,
        }
    }
}

impl_embodied_entity!(Building);

impl Entity for Building {
    fn update(&mut self, world: &World, _: f32) -> Vec<WorldStateUpdate> {
        if self.should_teleport(world) {
            return vec![self.engine_update_push_world()];
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

impl Building {
    fn door_frame(&self) -> Rect {
        self.building_type.door_frame(self.body.frame.x, self.body.frame.y)
    }

    fn should_teleport(&self, world: &World) -> bool {
        let door = self.door_frame();
        let hero_frame = world.cached_hero_props.frame;
        let hero_direction = world.cached_hero_props.direction;
        
        if let Some(collision) = door.collision_area_with_rect(&hero_frame) {
            if collision.w.floor() <= TILE_SIZE_HALF { return false }
            if collision.h.floor() < TILE_SIZE_HALF { return false }
            return hero_direction.y != 0.0;
        }
        false
    }

    fn engine_update_push_world(&self) -> WorldStateUpdate {
        WorldStateUpdate::EngineUpdate(
            EngineStateUpdate::ToggleWorld(self.interior_id)
        )
    }
}
