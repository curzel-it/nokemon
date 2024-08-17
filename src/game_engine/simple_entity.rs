use raylib::math::Rectangle;

use crate::{features::{autoremove::remove_automatically, linear_movement::move_linearly}, impl_embodied_entity};

use super::{entity::Entity, entity_body::EntityBody, world::World, state_updates::WorldStateUpdate};

#[derive(Debug)]
pub struct SimpleEntity {
    body: EntityBody,
    sprite_sheet_path: String,
}

impl_embodied_entity!(SimpleEntity);

impl Entity for SimpleEntity {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        move_linearly(self, world, time_since_last_update);
        world_updates.append(&mut remove_automatically(self, world));
        world_updates
    }

    fn texture_source_rect(&self) -> Rectangle {
        Rectangle::new(
            0.0,
            0.0,
            self.body.frame.width,
            self.body.frame.height
        )
    }

    fn sprite_sheet_path(&self) -> &str {
        &self.sprite_sheet_path 
    }
}

#[cfg(test)]
mod tests {
    use crate::{constants::ASSETS_PATH, game_engine::entity_body::EntityBody};

    use super::SimpleEntity;

    impl SimpleEntity {
        pub fn new(body: EntityBody) -> Self {
            Self { 
                body,
                sprite_sheet_path: format!("{}/entity.png", ASSETS_PATH)
            }
        }
    }
}