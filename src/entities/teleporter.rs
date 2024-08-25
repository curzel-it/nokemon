use std::any::Any;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{constants::{INFINITE_LIFESPAN, SPRITE_SHEET_TELEPORTER}, features::animated_sprite::AnimatedSprite, game_engine::{entity::Entity, entity_body::EntityBody, state_updates::{EngineStateUpdate, WorldStateUpdate}, world::World}, impl_embodied_entity, impl_single_animation_sprite_update, utils::{rect::Rect, vector::Vector2d}};

#[derive(Debug, Serialize, Deserialize)]
pub struct Teleporter {
    body: EntityBody,
    pub destination: Uuid,
    sprite: AnimatedSprite,
}

impl Teleporter {
    pub fn new(destination: Uuid) -> Self {
        Self { 
            body: EntityBody {
                id: Uuid::new_v4(),
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
            sprite: AnimatedSprite::new(
                SPRITE_SHEET_TELEPORTER, 
                3, 
                1, 
                1
            ),
        }
    }
}

impl_embodied_entity!(Teleporter);
impl_single_animation_sprite_update!(Teleporter);

impl Entity for Teleporter {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        self.update_sprite(time_since_last_update);

        self.sprite.row = if world.creative_mode { 0 } else { 1 };

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

        if hero_frame.w == 0 || hero_frame.h == 0 {
            return false
        }

        let base_y = hero_frame.y + hero_frame.h - 1;
        
        if base_y == self.body.frame.y {
            if hero_frame.x == self.body.frame.x + 1 {
                return hero_direction.x < 0.0;
            }
            if hero_frame.x == self.body.frame.x - 1 {
                return hero_direction.x > 0.0;
            }
        }
        if hero_frame.x == self.body.frame.x {
            if base_y == self.body.frame.y + 1 {
                return hero_direction.y < 0.0;
            }
            if base_y == self.body.frame.y - 1 {
                return hero_direction.y > 0.0;
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
