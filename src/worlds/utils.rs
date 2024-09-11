use crate::constants::*;

pub fn world_path(id: u32) -> String {
    format!("{}/{}.json", LEVELS_PATH, id)
}