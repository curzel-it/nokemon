
use raylib::math::{Rectangle, Vector2};

use crate::{features::{animated_sprite::AnimatedSprite, autoremove::remove_automatically, check_bullet_collisions::handle_collisions_for_bullet}, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, entity_factory::get_next_entity_id, world::World, world_state_update::WorldStateUpdate}, impl_embodied_entity, impl_single_animation_sprite_update, utils::geometry_utils::{Insets, Scalable}};

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
                frame: Rectangle::new(0.0, 0.0, 50.0, 30.0).to_scale(),
                collision_insets: Insets::zero().to_scale(),
                direction: Vector2::zero(),
                current_speed: 0.0,
                base_speed: 0.0,
                hp: 1000.0,
                dp: 20.0,
                creation_time: 0.0,
                requires_collision_detection: true,
                is_rigid: false,
                z_index: 0,
                is_ally: parent.body().is_ally,
                is_bullet: true,
                lifespan: 2.5,
            },
            sprite: AnimatedSprite::new("baseattack", 3, 50, 30)
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
        self.center_in(&world.cached_hero_frame);
        self.update_sprite(time_since_last_update);
        world_updates.append(&mut handle_collisions_for_bullet(self, world));
        world_updates.append(&mut remove_automatically(self, world));
        world_updates
    }

    fn texture_source_rect(&self) -> Rectangle {
        self.sprite.texture_source_rect()
    }

    fn sprite_sheet_path(&self) -> &str {
        &self.sprite.sheet_path 
    }
}