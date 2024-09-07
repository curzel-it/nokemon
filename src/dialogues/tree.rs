use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::lang::localizable::LocalizableText;

pub type DialogueId = u32;
pub type DialogueAnswerId = u32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dialogue {
    pub id: DialogueId,
    pub options: Vec<(DialogueAnswerId, DialogueId)>,
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

lazy_static! {
    pub static ref DIALOGUES: HashMap<u32, Dialogue> = vec!(
        // Old man in main village
        Dialogue { id: 101, options: vec![(1, 107)] },
        Dialogue { id: 107, options: vec![(1, 108)] },
        Dialogue { id: 108, options: vec![(102, 104), (103, 105)] },
        Dialogue { id: 104, options: vec![(0, 101)] },
        Dialogue { id: 105, options: vec![(0, 101)] },

        // Old lady in main village
        Dialogue { id: 6, options: vec![(0, 6)] },
    )
    .into_iter()
    .map(|d| (d.id, d))
    .collect();
}

pub fn dialogue_by_id(id: u32) -> Option<Dialogue> {
    if let Some(dialogue) = DIALOGUES.get(&id) {
        Some(dialogue.clone())
    } else {
        None
    }
}