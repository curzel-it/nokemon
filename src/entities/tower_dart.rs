
use crate::{features::{animated_sprite::update_sprite, autoremove::remove_automatically, check_bullet_collisions::handle_collisions_for_bullet, linear_movement::move_linearly}, game_engine::{entity::Entity, entity_body::EntityBody, entity_factory::EntityFactory, world::World, world_state_update::WorldStateUpdate}, impl_animated_entity, impl_embodied_entity};

#[derive(Debug)]
pub struct TowerDart {
    body: EntityBody
}

impl TowerDart {
    pub fn new(parent: &dyn Entity, entity_factory: &EntityFactory) -> Self {
        let mut body = entity_factory.build("towerdart");
        body.resize(10.0, 10.0);
        body.is_bullet = true;
        body.dp = 60.0;
        body.hp = 100.0;
        body.is_ally = parent.body().is_ally;
        body.parent_id = parent.id();
        body.direction = parent.body().direction;
        body.base_speed = 5.0;
        body.lifespan = 10.0;
        body.is_rigid = false;
        body.reset_speed();
        body.center_in(&parent.body().frame);
        
        Self {
            body
        }
    }
}

impl_embodied_entity!(TowerDart);
impl_animated_entity!(TowerDart);

impl Entity for TowerDart {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        move_linearly(self, world, time_since_last_update);
        update_sprite(self, time_since_last_update);
        world_updates.append(&mut handle_collisions_for_bullet(self, world));
        world_updates.append(&mut remove_automatically(self, world));
        world_updates
    }
}