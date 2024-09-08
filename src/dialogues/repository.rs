use std::{collections::HashMap, fs::File, io::BufReader};

use lazy_static::lazy_static;

use crate::constants::DIALOGUES_PATH;

use super::models::Dialogue;

fn load_dialogues_from_json(file_path: &str) -> HashMap<u32, Dialogue> {
    let file = File::open(file_path).expect("Failed to open dialogues.json file");    
    let reader = BufReader::new(file);
    let data: Vec<Dialogue> = serde_json::from_reader(reader).expect("Failed to deserialize dialogues from JSON");
    return data.into_iter().map(|d| (d.id, d)).collect()
}

lazy_static! {
    static ref DIALOGUES: HashMap<u32, Dialogue> = load_dialogues_from_json(DIALOGUES_PATH);
}

pub fn dialogue_by_id(id: u32) -> Option<Dialogue> {
    DIALOGUES.get(&id).cloned()
}