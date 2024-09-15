use serde::{Deserialize, Serialize};

use crate::constants::WORLD_ID_DEMO;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Destination {
    pub world: u32,
    pub x: i32,
    pub y: i32
}

impl Destination {
    pub fn new(world: u32, x: i32, y: i32) -> Self {
        Self { world, x, y }
    }

    pub fn nearest(world: u32) -> Self {
        Self::new(world, 0, 0)
    }
}

impl Default for Destination {
    fn default() -> Self {
        Self::nearest(WORLD_ID_DEMO)
    }
}