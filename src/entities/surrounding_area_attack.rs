
use crate::{features::{animated_sprite::update_sprite, autoremove::remove_automatically, check_bullet_collisions::handle_collisions_for_bullet}, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, entity_factory::EntityFactory, world::World, world_state_update::WorldStateUpdate}, impl_animated_entity, impl_embodied_entity};

#[derive(Debug)]
pub struct SurroundingAreaAttack {
    body: EntityBody
}

impl SurroundingAreaAttack {
    pub fn new(parent: &dyn Entity, entity_factory: &EntityFactory) -> Self {
        let mut body = entity_factory.build("baseattack");
        body.resize(50.0, 30.0);
        body.is_bullet = true;
        body.requires_collision_detection = true;
        body.dp = 20.0;
        body.hp = 1000.0;
        body.is_ally = parent.body().is_ally;
        body.parent_id = parent.id();
        body.base_speed = 0.0;
        body.lifespan = 2.0;
        body.reset_speed();
        body.center_in(&parent.body().frame);
        
        Self {
            body
        }
    }
}

impl_embodied_entity!(SurroundingAreaAttack);
impl_animated_entity!(SurroundingAreaAttack);

impl Entity for SurroundingAreaAttack {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        self.center_in(&world.cached_hero_frame);
        update_sprite(self, time_since_last_update);
        world_updates.append(&mut handle_collisions_for_bullet(self, world));
        world_updates.append(&mut remove_automatically(self, world));
        world_updates
    }
}