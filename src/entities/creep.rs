use raylib::math::Vector2;

use crate::{features::{animated_sprite::update_sprite, autoremove::remove_automatically, linear_movement::move_linearly, position_seeker::set_direction_towards}, game_engine::{entity::Entity, entity_body::EntityBody, entity_factory::EntityFactory, world::World, world_state_update::WorldStateUpdate}, impl_animated_entity, impl_embodied_entity, utils::geometry_utils::Insets};

#[derive(Debug)]
pub struct Creep {
    body: EntityBody
}

impl Creep {
    pub fn new(body: EntityBody) -> Self {
        Self { 
            body
        }
    }
}

impl_embodied_entity!(Creep);
impl_animated_entity!(Creep);

impl Entity for Creep {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        set_direction_towards(self, &world.cached_hero_position);
        move_linearly(self, world, time_since_last_update);
        update_sprite(self, time_since_last_update);
        world_updates.append(&mut remove_automatically(self, world));
        world_updates
    }
}

impl EntityFactory {
    pub fn build_creep(&self) -> Creep {
        let mut body = self.build("white");
        body.resize(19.0, 22.0);
        body.collision_insets = Insets::new(12.0, 4.0, 0.0, 4.0);
        body.base_speed = 1.5;
        body.reset_speed();
        body.is_ally = false;
        body.direction = Vector2::new(1.0, 0.0);    
        Creep::new(body)
    }
}