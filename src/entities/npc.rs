use std::any::Any;

use uuid::Uuid;

use crate::{constants::{INFINITE_LIFESPAN, NO_PARENT, SPRITE_SHEET_HUMANOIDS, SPRITE_SHEET_HUMANOIDS_COUNT}, features::{animated_sprite::AnimatedSprite, autoremove::remove_automatically, linear_movement::move_linearly}, game_engine::{entity::Entity, entity_body::EntityBody, state_updates::WorldStateUpdate, world::World}, impl_embodied_entity, impl_humanoid_sprite_update, utils::{rect::Rect, vector::Vector2d}};

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
                parent_id: NO_PARENT,
                frame: Rect::new(0, 0, 1, 1),
                offset: Vector2d::zero(),
                direction: Vector2d::zero(),
                current_speed: 1.5,
                base_speed: 1.5,
                hp: 100.0,
                dp: 0.0,
                creation_time: 0.0,
                requires_collision_detection: true,
                is_rigid: true,
                z_index: 0,
                is_ally: true,
                lifespan: INFINITE_LIFESPAN,
            },
            sprite: AnimatedSprite::new_stepped(
                SPRITE_SHEET_HUMANOIDS, 
                3, 
                1,
                SPRITE_SHEET_HUMANOIDS_COUNT,
                1, 
                1
            ),
        }
    }
}

impl_embodied_entity!(Npc);
impl_humanoid_sprite_update!(Npc);

impl Entity for Npc {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        move_linearly(self, world, time_since_last_update);
        self.update_sprite(time_since_last_update);
        world_updates.append(&mut remove_automatically(self, world));
        world_updates
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
