use uuid::Uuid;

use crate::{constants::LEVELS_PATH, utils::file_utils::list_files};

use super::constants::{WORLD_ID_DEMO, WORLD_ID_NONE};

pub fn world_path(id: Uuid) -> String {
    format!("{}/{}.json", LEVELS_PATH, id)
}

pub fn list_worlds_with_none() -> Vec<Uuid> {
    let mut ids: Vec<Uuid> = list_worlds().into_iter().filter(|uuid| *uuid != WORLD_ID_NONE).collect();
    ids.push(WORLD_ID_NONE);
    ids
}

pub fn list_worlds() -> Vec<Uuid> {
    list_files(LEVELS_PATH, "json")
        .into_iter()
        .filter_map(|path| {
            if let Some(filename) = std::path::Path::new(&path).file_stem() {
                filename.to_str().and_then(|s| s.parse::<Uuid>().ok())
            } else {
                None
            }
        })
        .collect()
}

pub fn world_name(id: &Uuid) -> String {
    match *id {
        WORLD_ID_NONE => "New World".to_string(),
        WORLD_ID_DEMO => "Demo World".to_string(),
        _ => format!("World {}", id),
    }
}