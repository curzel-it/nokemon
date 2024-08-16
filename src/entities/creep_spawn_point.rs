use raylib::math::{Rectangle, Vector2};

use crate::{constants::{INFINITE_LIFESPAN, NO_PARENT}, features::{animated_sprite::AnimatedSprite, autoremove::remove_automatically, linear_movement::move_linearly}, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, entity_factory::get_next_entity_id, world::World, world_state_update::WorldStateUpdate}, impl_embodied_entity, utils::geometry_utils::{Insets, Scalable}};

use super::creep::Creep;

#[derive(Debug)]
pub struct CreepSpawnPoint {
    body: EntityBody,
    last_spawn_time: f32,
    time_to_spawn: f32,
    sprite: AnimatedSprite,
}

impl CreepSpawnPoint {
    pub fn new() -> Self {
        Self { 
            body: EntityBody {
                id: get_next_entity_id(),
                parent_id: NO_PARENT,
                frame: Rectangle::new(0.0, 0.0, 50.0, 30.0).to_scale(),
                collision_insets: Insets::zero().to_scale(),
                direction: Vector2::zero(),
                current_speed: 0.0,
                base_speed: 0.0,
                hp: 100.0,
                dp: 0.0,
                time_to_next_shot: 1000.0,
                time_between_shots: 1000.0,
                creation_time: 0.0,
                requires_collision_detection: false,
                is_rigid: false,
                z_index: 0,
                is_ally: false,
                is_bullet: false,
                lifespan: INFINITE_LIFESPAN,
            },
            last_spawn_time: 0.0,
            time_to_spawn: 2.0,
            sprite: AnimatedSprite::new("baseattack", 3, 50, 30)
        }
    }
}

impl_embodied_entity!(CreepSpawnPoint);

impl Entity for CreepSpawnPoint {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        move_linearly(self, world, time_since_last_update);
        self.update_sprite(time_since_last_update);

        if world.total_elapsed_time - self.last_spawn_time > self.time_to_spawn {
            self.last_spawn_time = world.total_elapsed_time;
            world_updates.push(WorldStateUpdate::AddEntity(self.build_creep()))
        }

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

impl CreepSpawnPoint {
    fn update_sprite(&mut self, time_since_last_update: f32) {
        self.sprite.update(time_since_last_update);
    }
}

impl CreepSpawnPoint {
    fn build_creep(&self) -> Box<dyn Entity> {
        let mut creep = Creep::new();
        creep.center_in(&self.body().frame);
        creep.body_mut().direction = Vector2::new(1.0, 0.0);
        Box::new(creep)
    }
}