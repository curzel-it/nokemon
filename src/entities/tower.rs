use crate::{constants::{INFINITE_LIFESPAN, NO_PARENT}, features::{animated_sprite::AnimatedSprite, autoremove::remove_automatically, shooter::{shoot_stuff, Shooter}}, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, entity_factory::get_next_entity_id, state_updates::WorldStateUpdate, world::World}, impl_embodied_entity, impl_shooter, impl_single_animation_sprite_update, utils::{geometry_utils::Insets, rect::Rect, vector::Vector2d}};

use super::tower_dart::TowerDart;

#[derive(Debug)]
pub struct Tower {
    body: EntityBody,
    time_to_next_shot: f32,
    time_between_shots: f32,
    sprite: AnimatedSprite,
}

impl Tower {
    pub fn new() -> Self {
        Self { 
            body: EntityBody {
                id: get_next_entity_id(),
                parent_id: NO_PARENT,
                frame: Rect::new(0.0, 0.0, 26.0, 42.0),
                collision_insets: Insets::new(8.0, 0.0, 0.0, 0.0),
                direction: Vector2d::new(1.0, 0.0),
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
            time_to_next_shot: 2.0,
            time_between_shots: 2.0,
            sprite: AnimatedSprite::new("tower", 8, 26, 42)
        }
    }
}

impl_embodied_entity!(Tower);
impl_shooter!(Tower, TowerDart);
impl_single_animation_sprite_update!(Tower);

impl Entity for Tower {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        self.update_sprite(time_since_last_update);
        world_updates.append(&mut shoot_stuff(self, time_since_last_update));
        world_updates.append(&mut remove_automatically(self, world));
        world_updates
    }

    fn texture_source_rect(&self) -> Rect {
        self.sprite.texture_source_rect()
    }

    fn sprite_sheet_path(&self) -> &str {
        &self.sprite.sheet_path
    }
}