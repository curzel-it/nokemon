use raylib::math::{Rectangle, Vector2};

use crate::{constants::{INFINITE_LIFESPAN, NO_PARENT}, features::animated_sprite::AnimatedSprite, impl_embodied_entity, utils::geometry_utils::Insets};

use super::{entity::Entity, entity_body::EntityBody, entity_factory::get_next_entity_id, world::World, state_updates::WorldStateUpdate};

#[derive(Debug)]
pub struct StaticObstacle {
    body: EntityBody,
    sprite: AnimatedSprite,
}

impl StaticObstacle {
    pub fn new(sprite: &str, frame: Rectangle) -> Self {
        Self { 
            body: EntityBody {
                id: get_next_entity_id(),
                parent_id: NO_PARENT,
                frame,
                collision_insets: Insets::zero(),
                direction: Vector2::zero(),
                current_speed: 0.0,
                base_speed: 0.0,
                hp: 100.0,
                dp: 0.0,
                creation_time: 0.0,
                requires_collision_detection: false,
                is_rigid: true,
                z_index: 0,
                is_ally: false,
                lifespan: INFINITE_LIFESPAN,
            },
            sprite: AnimatedSprite::new(sprite, 1, frame.width as u32, frame.height as u32)
        }
    }
}

impl_embodied_entity!(StaticObstacle);

impl Entity for StaticObstacle {
    fn update(&mut self, _: &World, _: f32) -> Vec<WorldStateUpdate> {
        vec![]
    }

    fn texture_source_rect(&self) -> Rectangle {
        self.sprite.texture_source_rect()
    }

    fn sprite_sheet_path(&self) -> &str {
        &self.sprite.sheet_path 
    }
}