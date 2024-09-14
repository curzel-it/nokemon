use serde::{Deserialize, Serialize};

use crate::constants::{WORLD_ID_DEMO, WORLD_SIZE_COLUMNS, WORLD_SIZE_ROWS};

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

    pub fn center(world: u32) -> Self {
        Self {
            world, 
            x: WORLD_SIZE_COLUMNS as i32 / 2, 
            y: WORLD_SIZE_ROWS as i32 / 2
        }
    }
}

impl Default for Destination {
    fn default() -> Self {
        Self::nearest(WORLD_ID_DEMO)
    }
}