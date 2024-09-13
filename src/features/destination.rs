use serde::{Deserialize, Serialize};

use crate::constants::{WORLD_ID_DEMO, WORLD_SIZE_COLUMNS, WORLD_SIZE_ROWS};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Destination {
    pub world: u32,
    pub x: i32,
    pub y: i32
}

impl Destination {
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
        Self::center(WORLD_ID_DEMO)
    }
}