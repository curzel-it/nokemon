use raylib::math::{Rectangle, Vector2};

use crate::{constants::{ASSETS_PATH, INFINITE_LIFESPAN, NO_PARENT, TILE_SIZE}, game_engine::{entity::Entity, entity_body::EntityBody, entity_factory::get_next_entity_id, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, impl_embodied_entity, utils::geometry_utils::{is_collision_trajectory, Insets, Scalable}};

const BUILDINGS_SPRITE_SHEET: &str = "buildings";

#[derive(Debug, Clone, Copy, Hash)]
pub enum BuildingType {
    House
}

impl BuildingType {
    fn texture_source_rect(&self) -> Rectangle {
        let (row, w, h) = match self {
            BuildingType::House => (0, 5, 5)
        };
        Rectangle::new(
            0 as f32, 
            row as f32, 
            w as f32, 
            h as f32
        ).scaled(TILE_SIZE)
    }

    fn door_frame(&self, x: f32, y: f32) -> Rectangle {
        let (row, col, w, h) = match self {
            BuildingType::House => (4, 3, 1, 1)
        };
        Rectangle::new(
            x + col as f32 * TILE_SIZE, 
            y + row as f32 * TILE_SIZE, 
            w as f32 * TILE_SIZE, 
            h as f32 * TILE_SIZE - TILE_SIZE / 4.0
        )
    }
}

#[derive(Debug)]
pub struct Building {
    body: EntityBody,
    building_type: BuildingType,
    interior_level_id: u32,
    sprite_sheet_path: String
}

impl Building {
    pub fn new(building_type: BuildingType, interior_level_id: u32) -> Self {
        let frame = building_type.texture_source_rect();

        Self { 
            body: EntityBody {
                id: get_next_entity_id(),
                parent_id: NO_PARENT,
                frame: Rectangle::new(0.0, 0.0, frame.width, frame.height),
                collision_insets: Insets::zero(),
                direction: Vector2::zero(),
                current_speed: 0.0,
                base_speed: 0.0,
                hp: 1000.0,
                dp: 0.0,
                creation_time: 0.0,
                requires_collision_detection: false,
                is_rigid: true,
                z_index: 0,
                is_ally: true,
                is_bullet: false,
                lifespan: INFINITE_LIFESPAN,
            },      
            building_type,
            interior_level_id,
            sprite_sheet_path: format!("{}/{}.png", ASSETS_PATH, BUILDINGS_SPRITE_SHEET),
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

    fn texture_source_rect(&self) -> Rectangle {
        self.building_type.texture_source_rect()
    }

    fn sprite_sheet_path(&self) -> &str {
        &self.sprite_sheet_path
    }
}

impl Building {
    fn should_teleport(&self, world: &World) -> bool {
        if !self.hero_is_on_door(world) {
            return false 
        }
        let is_colliding = is_collision_trajectory(
            &world.cached_hero_props.direction, 
            &world.cached_hero_props.frame, 
            &self.body.frame
        );
        is_colliding && world.cached_hero_props.speed > 0.1
    }

    fn hero_is_on_door(&self, world: &World) -> bool {
        let door = self.building_type.door_frame(self.body.frame.x, self.body.frame.y);
        door.check_collision_recs(&world.cached_hero_props.frame)
    }

    fn engine_update_push_world(&self) -> WorldStateUpdate {
        WorldStateUpdate::EngineUpdate(
            EngineStateUpdate::ToggleWorld(self.interior_level_id)
        )
    }
}