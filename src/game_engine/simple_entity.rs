use crate::{features::{animated_sprite::update_sprite, autoremove::remove_automatically, linear_movement::move_linearly}, impl_animated_entity, impl_embodied_entity};

use super::{entity::Entity, entity_body::EntityBody, world::World, world_state_update::WorldStateUpdate};

#[derive(Debug)]
pub struct SimpleEntity {
    body: EntityBody
}

impl_embodied_entity!(SimpleEntity);
impl_animated_entity!(SimpleEntity);

impl Entity for SimpleEntity {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        move_linearly(self, world, time_since_last_update);
        update_sprite(self, time_since_last_update);
        world_updates.append(&mut remove_automatically(self, world));
        world_updates
    }
}

#[cfg(test)]
mod tests {
    use crate::game_engine::entity_body::EntityBody;

    use super::SimpleEntity;

    impl SimpleEntity {
        pub fn new(body: EntityBody) -> Self {
            Self { 
                body
            }
        }
    }
}