use raylib::math::Vector2;

use crate::{features::{animated_sprite::update_sprite, autoremove::remove_automatically, linear_movement::move_linearly}, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, entity_factory::EntityFactory, world_state_update::WorldStateUpdate, world::World}, impl_animated_entity, impl_embodied_entity};


#[derive(Debug)]
pub struct CreepSpawnPoint {
    body: EntityBody,
    last_spawn_time: f32,
    time_to_spawn: f32
}

impl CreepSpawnPoint {
    pub fn new(body: EntityBody) -> Self {
        Self { 
            body,
            last_spawn_time: 0.0,
            time_to_spawn: 2.0
        }
    }
}

impl_embodied_entity!(CreepSpawnPoint);
impl_animated_entity!(CreepSpawnPoint);

impl Entity for CreepSpawnPoint {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        move_linearly(self, time_since_last_update);
        update_sprite(self, time_since_last_update);

        if world.total_elapsed_time - self.last_spawn_time > self.time_to_spawn {
            self.last_spawn_time = world.total_elapsed_time;
            world_updates.push(WorldStateUpdate::AddEntity(self.build_creep(world)))
        }

        world_updates.append(&mut remove_automatically(self, world));
        world_updates
    }
}

impl CreepSpawnPoint {
    fn build_creep(&self, world: &World) -> Box<dyn Entity> {
        let mut creep = world.entity_factory.build_creep();
        creep.center_in(&self.body().frame);
        creep.body_mut().direction = Vector2::new(1.0, 0.0);
        Box::new(creep)
    }
}

impl EntityFactory {
    pub fn build_creep_spawn_point(&self) -> CreepSpawnPoint {
        let mut body = self.build("creepspawnpoint");
        body.resize(50.0, 30.0);
        CreepSpawnPoint::new(body)
    }
}