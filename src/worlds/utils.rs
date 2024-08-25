use crate::{constants::LEVELS_PATH, utils::file_utils::list_files};

use super::constants::{WORLD_ID_DEMO, WORLD_ID_NONE};

pub fn world_path(id: u32) -> String {
    format!("{}/{}.json", LEVELS_PATH, id)
}

pub fn get_next_world_id() -> u32 {
    if let Some(last_id) = list_worlds().iter().max() {
        last_id + 1
    } else {
        10_000
    }
}

pub fn list_worlds() -> Vec<u32> {
    list_files(&LEVELS_PATH, "json")
        .into_iter()
        .filter_map(|path| {
            if let Some(filename) = std::path::Path::new(&path).file_stem() {
                filename.to_str().and_then(|s| s.parse::<u32>().ok())
            } else {
                None
            }
        })
        .filter(|id| *id > 0) 
        .collect()
}

pub fn world_name(id: &u32) -> String {
    match *id {
        WORLD_ID_NONE => "New World".to_string(),
        WORLD_ID_DEMO => "Demo World".to_string(),
        _ => format!("World #{}", id),
    }
}