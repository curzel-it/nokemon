use std::{collections::HashMap, fs::File, io::BufReader};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::{constants::DIALOGUES_PATH, lang::localizable::LocalizableText};

pub type DialogueId = u32;
pub type DialogueAnswerId = u32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dialogue {
    pub id: DialogueId,
    pub options: Vec<(DialogueAnswerId, DialogueId)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDialogues {
    pub options: Vec<EntityDialogue>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDialogue {
    pub key: String,
    pub expected_value: u32,
    pub dialogue: Dialogue,
}

impl EntityDialogues {
    pub fn empty() -> Self {
        Self { options: vec![] }
    }
}

impl Default for EntityDialogues {
    fn default() -> Self {
        Self::empty()
    }
}

impl Dialogue {
    pub const fn empty() -> Self {
        Self { id: 0, options: vec![] }
    }

    pub fn localized_text(&self) -> String {
        format!("dialogue.{}", self.id).localized()
    }

    pub fn localized_options(&self) -> Vec<String> {
        self.options.iter().map(|o| format!("dialogue.{}", o.0).localized()).collect()        
    }
}

fn load_dialogues_from_json(file_path: &str) -> HashMap<u32, Dialogue> {
    let file = File::open(file_path).expect("Failed to open dialogues.json file");    
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to deserialize dialogues from JSON")
}

lazy_static! {
    static ref DIALOGUES: HashMap<u32, Dialogue> = load_dialogues_from_json(DIALOGUES_PATH);
}

pub fn dialogue_by_id(id: u32) -> Option<Dialogue> {
    DIALOGUES.get(&id).cloned()
}