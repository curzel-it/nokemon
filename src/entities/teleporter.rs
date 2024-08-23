use std::any::Any;

use crate::{constants::{INFINITE_LIFESPAN, NO_PARENT, SPRITE_SHEET_TELEPORTER, TILE_SIZE, TILE_SIZE_HALF}, features::animated_sprite::AnimatedSprite, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, entity_factory::get_next_entity_id, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, impl_embodied_entity, impl_single_animation_sprite_update, levels::constants::LEVEL_ID_HOUSE_INTERIOR, utils::{geometry_utils::Insets, rect::Rect, vector::Vector2d}};

#[derive(Debug)]
pub struct Teleporter {
    body: EntityBody,
    destination: u32,
    sprite: AnimatedSprite,
}

impl Teleporter {
    pub fn new() -> Self {
        Self { 
            body: EntityBody {
                id: get_next_entity_id(),
                parent_id: NO_PARENT,
                frame: Rect::new(0.0, 0.0, TILE_SIZE, TILE_SIZE),
                collision_insets: Insets::zero(),
                direction: Vector2d::zero(),
                current_speed: 0.0,
                base_speed: 0.0,
                hp: 100.0,
                dp: 0.0,
                creation_time: 0.0,
                requires_collision_detection: true,
                is_rigid: false,
                z_index: 0,
                is_ally: false,
                lifespan: INFINITE_LIFESPAN,
            },
            destination: LEVEL_ID_HOUSE_INTERIOR,
            sprite: AnimatedSprite::new(
                SPRITE_SHEET_TELEPORTER, 
                3, 
                TILE_SIZE as u32, 
                TILE_SIZE as u32
            ),
        }
    }
}

impl_embodied_entity!(Teleporter);
impl_single_animation_sprite_update!(Teleporter);

impl Entity for Teleporter {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        self.update_sprite(time_since_last_update);

        if self.should_teleport(world) {
            return vec![self.engine_update_push_world()];
        }
        vec![]
    }

    fn texture_source_rect(&self) -> Rect {
        self.sprite.texture_source_rect()
    }

    fn sprite_sheet(&self) -> u32 {
        self.sprite.sheet_id
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Teleporter {
    fn should_teleport(&self, world: &World) -> bool {
        let hero_frame = world.cached_hero_props.frame;
        let hero_direction = world.cached_hero_props.direction;
        
        if let Some(collision) = self.body.frame.collision_area_with_rect(&hero_frame) {
            if collision.w.floor() <= TILE_SIZE_HALF { return false }
            if collision.h.floor() < TILE_SIZE_HALF { return false }
            return hero_direction.y != 0.0;
        }
        false
    }

    fn engine_update_push_world(&self) -> WorldStateUpdate {
        WorldStateUpdate::EngineUpdate(
            EngineStateUpdate::ToggleWorld(self.destination)
        )
    }
}
