use std::any::Any;

use crate::{features::{autoremove::remove_automatically, linear_movement::move_linearly}, impl_embodied_entity, utils::rect::Rect};

use super::{entity::Entity, entity_body::EntityBody, world::World, state_updates::WorldStateUpdate};

#[derive(Debug)]
pub struct SimpleEntity {
    body: EntityBody,
    sprite_sheet: u32,
}

impl_embodied_entity!(SimpleEntity);

impl Entity for SimpleEntity {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        move_linearly(self, world, time_since_last_update);
        world_updates.append(&mut remove_automatically(self, world));
        world_updates
    }

    fn texture_source_rect(&self) -> Rect {
        Rect::new(
            0.0,
            0.0,
            self.body.frame.w,
            self.body.frame.h
        )
    }

    fn sprite_sheet(&self) -> u32 {
        self.sprite_sheet 
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{constants::SPRITE_SHEET_BLANK, game_engine::entity_body::EntityBody};

    use super::SimpleEntity;

    impl SimpleEntity {
        pub fn new(body: EntityBody) -> Self {
            Self { 
                body,
                sprite_sheet: SPRITE_SHEET_BLANK
            }
        }
    }
}