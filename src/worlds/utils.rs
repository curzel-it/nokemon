use crate::{constants::*, lang::localizable::LocalizableText, utils::file_utils::list_files};

pub fn world_path(id: u32) -> String {
    format!("{}/{}.json", LEVELS_PATH, id)
}

pub fn list_worlds_with_none() -> Vec<u32> {
    let mut ids: Vec<u32> = list_worlds().into_iter().filter(|id| *id != WORLD_ID_NONE).collect();
    ids.push(WORLD_ID_NONE);
    ids
}

pub fn list_worlds() -> Vec<u32> {
    let mut ids: Vec<u32> = list_files(LEVELS_PATH, "json")
        .into_iter()
        .filter_map(|path| {
            if let Some(filename) = std::path::Path::new(&path).file_stem() {
                filename.to_str().and_then(|s| s.parse::<u32>().ok())
            } else {
                None
            }
        })
        .collect();
    ids.sort();
    ids
}

pub fn world_name(id: &u32) -> String {
    match *id {
        WORLD_ID_NONE => "world.names.new_world".localized(),
        WORLD_ID_DEMO => "world.names.demo".localized(),
        _ => format!("{} #{}", "world.names.with_id".localized(), id),
    }
}