use serde::{Deserialize, Serialize};

use crate::{entities::species::{species_by_id, SpeciesId}, game_engine::storage::StorageKey, lang::localizable::LocalizableText};

pub type DialogueId = u32;
pub type DialogueAnswerId = u32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dialogue {
    pub id: DialogueId,
    pub options: Vec<(DialogueAnswerId, DialogueId)>,

    #[serde(default)]
    pub reward: Option<SpeciesId>,
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
    pub fn single_option(id: DialogueId) -> Self {
        Self { options: vec![
            EntityDialogue {
                key: StorageKey::always(), 
                expected_value: 1, 
                dialogue: id 
            }
        ] }
    }
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
        Self { id: 0, options: vec![], reward: None }
    }

    pub fn localized_text(&self) -> String {
        format!("dialogue.{}", self.id).localized()
    }

    pub fn localized_options(&self) -> Vec<String> {
        self.options.iter().map(|o| format!("dialogue.{}", o.0).localized()).collect()        
    }

    pub fn localized_reward_text(&self) -> String {
        if let Some(reward_species_id) = self.reward {
            let species_name = species_by_id(reward_species_id).localized_name();
            let text = "dialogue.reward_received".localized();
            text.replace("%s", &species_name)
        } else {
            "".to_owned()
        }
    }
}
