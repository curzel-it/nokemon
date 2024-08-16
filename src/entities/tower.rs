use raylib::math::{Rectangle, Vector2};

use crate::{constants::{INFINITE_LIFESPAN, NO_PARENT}, features::{animated_sprite::AnimatedSprite, autoremove::remove_automatically, shooter::{shoot_stuff, Shooter}}, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, entity_factory::get_next_entity_id, world::World, world_state_update::WorldStateUpdate}, impl_embodied_entity, utils::geometry_utils::{Insets, Scalable}};

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
                frame: Rectangle::new(0.0, 0.0, 26.0, 42.0).to_scale(),
                collision_insets: Insets::new(8.0, 0.0, 0.0, 0.0).to_scale(),
                direction: Vector2::new(1.0, 0.0),
                current_speed: 0.0,
                base_speed: 0.0,
                hp: 100.0,
                dp: 0.0,
                creation_time: 0.0,
                requires_collision_detection: false,
                is_rigid: true,
                z_index: 0,
                is_ally: false,
                is_bullet: false,
                lifespan: INFINITE_LIFESPAN,
            },
            time_to_next_shot: 2.0,
            time_between_shots: 2.0,
            sprite: AnimatedSprite::new("tower", 8, 26, 42)
        }
    }
}

impl_embodied_entity!(Tower);

impl Shooter for Tower {
    fn time_to_next_shot(&self) -> f32 {
        self.time_to_next_shot
    }
    
    fn inc_time_to_next_shot(&mut self, delta: f32) {
        self.time_to_next_shot += delta;
    }
    
    fn reset_time_to_next_shot(&mut self) {
        self.time_to_next_shot = self.time_between_shots;
    }
    
    fn create_bullet(&self) -> Box<dyn Entity> {
        Box::new(TowerDart::new(self))
    }
}

impl Entity for Tower {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        self.update_sprite(time_since_last_update);
        world_updates.append(&mut shoot_stuff(self, time_since_last_update));
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

impl Tower {
    fn update_sprite(&mut self, time_since_last_update: f32) {
        self.sprite.update(time_since_last_update);
    }
}