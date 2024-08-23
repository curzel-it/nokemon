
use std::any::Any;

use crate::{constants::SPRITE_SHEET_BASE_ATTACK, features::{animated_sprite::AnimatedSprite, autoremove::remove_automatically, check_bullet_collisions::handle_collisions_for_bullet}, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, entity_factory::get_next_entity_id, state_updates::WorldStateUpdate, world::World}, impl_embodied_entity, impl_single_animation_sprite_update, utils::{geometry_utils::Insets, rect::Rect, vector::Vector2d}};

#[derive(Debug)]
pub struct SurroundingAreaAttack {
    body: EntityBody,
    sprite: AnimatedSprite,
}

impl SurroundingAreaAttack {
    pub fn new(parent: &dyn Entity) -> Self {        
        let mut entity = Self {
            body: EntityBody {
                id: get_next_entity_id(),
                parent_id: parent.id(),
                frame: Rect::new(0.0, 0.0, 50.0, 30.0),
                collision_insets: Insets::zero(),
                direction: Vector2d::zero(),
                current_speed: 0.0,
                base_speed: 0.0,
                hp: 1000.0,
                dp: 20.0,
                creation_time: 0.0,
                requires_collision_detection: true,
                is_rigid: false,
                z_index: 0,
                is_ally: parent.body().is_ally,
                lifespan: 2.5,
            },
            sprite: AnimatedSprite::new(
                SPRITE_SHEET_BASE_ATTACK, 
                3, 
                50, 
                30
            )
        };
        entity.center_in(&parent.body().frame);
        entity
    }
}

impl_embodied_entity!(SurroundingAreaAttack);
impl_single_animation_sprite_update!(SurroundingAreaAttack);

impl Entity for SurroundingAreaAttack {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        self.center_in(&world.cached_hero_props.frame);
        self.update_sprite(time_since_last_update);
        world_updates.append(&mut handle_collisions_for_bullet(self, world));
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