
use raylib::math::{Rectangle, Vector2};

use crate::{features::{animated_sprite::AnimatedSprite, autoremove::remove_automatically, check_bullet_collisions::handle_collisions_for_bullet}, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, entity_factory::get_next_entity_id, world::World, world_state_update::WorldStateUpdate}, impl_embodied_entity, sprites::{sprite::Sprite, sprite_set::SpriteSet}, utils::geometry_utils::{Insets, Scalable}};

#[derive(Debug)]
pub struct SurroundingAreaAttack {
    body: EntityBody,
    sprite: AnimatedSprite,
}

impl SurroundingAreaAttack {
    pub fn new(parent: &dyn Entity) -> Self {        
        Self {
            body: EntityBody {
                id: get_next_entity_id(),
                parent_id: parent.id(),
                frame: Rectangle::new(0.0, 0.0, 50.0, 30.0).to_scale(),
                collision_insets: Insets::zero(),
                direction: Vector2::new(0.0, 0.0),
                current_speed: 0.0,
                base_speed: 0.0,
                hp: 1000.0,
                dp: 20.0,
                sprite_set: SpriteSet::default(),
                current_sprite: Sprite::empty(),
                sprite_invalidated: true,
                time_to_next_shot: 1000.0,
                time_between_shots: 1000.0,
                creation_time: 0.0,
                requires_collision_detection: true,
                is_rigid: false,
                z_index: 0,
                is_ally: parent.body().is_ally,
                is_bullet: true,
                lifespan: 2.5,
            },
            sprite: AnimatedSprite::new("baseattack.png", 3, 50, 30)
        }
    }
}

impl_embodied_entity!(SurroundingAreaAttack);

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

impl SurroundingAreaAttack {
    fn update_sprite(&mut self, time_since_last_update: f32) {
        self.sprite.update(time_since_last_update);
    }
}