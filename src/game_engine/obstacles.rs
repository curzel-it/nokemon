use raylib::math::{Rectangle, Vector2};

use crate::{constants::{ANIMATION_NAME_FRONT, INFINITE_LIFESPAN}, impl_embodied_entity};

use super::{entity::Entity, entity_body::EntityBody, entity_factory::EntityFactory, world::World, world_state_update::WorldStateUpdate};

#[derive(Debug)]
pub struct StaticObstacle {
    body: EntityBody,
}

impl StaticObstacle {
    pub fn new(body: EntityBody) -> Self {
        Self { 
            body,
        }
    }
}

impl_embodied_entity!(StaticObstacle);

impl Entity for StaticObstacle {
    fn update(&mut self, _: &World, _: f32) -> Vec<WorldStateUpdate> {
        vec![]
    }
}


impl EntityFactory {
    pub fn build_invisible_obstacle(&self, frame: Rectangle) -> StaticObstacle {
        let mut body = self.build("obstacle");
        body.set_animation(ANIMATION_NAME_FRONT);
        body.is_rigid = true;
        body.base_speed = 0.0;
        body.current_speed = 0.0;
        body.lifespan = INFINITE_LIFESPAN;
        body.dp = 0.0;
        body.direction = Vector2::zero();
        body.frame = frame;
        StaticObstacle::new(body)
    }
}