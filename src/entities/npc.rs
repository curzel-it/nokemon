use std::any::Any;

use uuid::Uuid;

use crate::{constants::INFINITE_LIFESPAN, features::{animated_sprite::AnimatedSprite, linear_movement::move_linearly}, game_engine::{entity::Entity, entity_body::EntityBody, state_updates::WorldStateUpdate, world::World}, impl_embodied_entity, impl_humanoid_sprite_update, utils::{rect::Rect, vector::Vector2d}};

#[derive(Debug)]
pub struct Npc {
    body: EntityBody,
    sprite: AnimatedSprite,
}

impl Npc {
    pub fn new() -> Self {
        Self {             
            body: EntityBody {
                id: Uuid::new_v4(),
                frame: Rect::new(0, 0, 1, 2),
                offset: Vector2d::zero(),
                direction: Vector2d::zero(),
                current_speed: 1.5,
                base_speed: 1.5,
                creation_time: 0.0,
                is_rigid: true,
                z_index: 0,
                lifespan: INFINITE_LIFESPAN,
            },
            sprite: AnimatedSprite::new_humanoid(0),
        }
    }
}

impl_embodied_entity!(Npc);
impl_humanoid_sprite_update!(Npc);

impl Entity for Npc {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        move_linearly(self, world, time_since_last_update);
        self.update_sprite(time_since_last_update);
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
