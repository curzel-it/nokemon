use crate::game_engine::world::World;

use super::{constants::WORLD_ID_DEMO, world_setup_demo::world_setup_demo};

impl World {
    pub fn setup(&mut self) {
        match self.id {
            WORLD_ID_DEMO => world_setup_demo(self),
            _ => {}
        }
    }
}