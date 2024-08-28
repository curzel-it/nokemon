use crate::{constants::*, utils::file_utils::list_files};

pub fn world_path(id: u32) -> String {
    format!("{}/{}.json", LEVELS_PATH, id)
}

pub fn list_worlds_with_none() -> Vec<u32> {
    let mut ids: Vec<u32> = list_worlds().into_iter().filter(|uuid| *uuid != WORLD_ID_NONE).collect();
    ids.push(WORLD_ID_NONE);
    ids
}

pub fn list_worlds() -> Vec<u32> {
    list_files(LEVELS_PATH, "json")
        .into_iter()
        .filter_map(|path| {
            if let Some(filename) = std::path::Path::new(&path).file_stem() {
                filename.to_str().and_then(|s| s.parse::<u32>().ok())
            } else {
                None
            }
        })
        .collect()
}

pub fn world_name(id: &u32) -> String {
    match *id {
        WORLD_ID_NONE => "New World".to_string(),
        WORLD_ID_DEMO => "Demo World".to_string(),
        WORLD_ID_DEMO_HOUSE_INTERIOR => "Demo House Interior".to_string(),
        WORLD_ID_DEMO_HOUSE_TWO_FLOORS_INTERIOR => "Demo Two-Floors House | 1st Floor".to_string(),
        WORLD_ID_DEMO_HOUSE_TWO_FLOORS_SECOND_FLOOR => "Demo Two-Floors House | 2nd Floor".to_string(),
        _ => format!("World {}", id),
    }
}