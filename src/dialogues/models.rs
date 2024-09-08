use serde::{Deserialize, Serialize};

use crate::lang::localizable::LocalizableText;

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
    pub dialogue: DialogueId,
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
