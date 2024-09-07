use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::lang::localizable::LocalizableText;

pub type DialogueId = u32;
pub type DialogueAnswerId = u32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dialogue {
    pub id: DialogueId,
    pub options: Vec<DialogueAnswerId>,
    pub stops: bool,
}

impl Dialogue {
    pub const fn empty() -> Self {
        Self { id: 0, options: vec![], stops: true }
    }

    pub fn localized_text(&self) -> String {
        format!("dialogue.{}", self.id).localized()
    }

    pub fn localized_options(&self) -> Vec<String> {
        self.options.iter().map(|o| format!("dialogue.{}", o).localized()).collect()        
    }
}

macro_rules! dialogue_with_options {
    ($id:expr, $( $option:expr ),*) => {
        Dialogue {
            id: $id,
            options: vec![$($option),*],
            stops: false
        }
    };
}

macro_rules! dialogue_answer {
    ($id:expr, $option:expr) => {
        Dialogue {
            id: $id,
            options: vec![$option],
            stops: false
        }
    };
}

macro_rules! dialogue_stopper {
    ($id:expr, $option:expr) => {
        Dialogue {
            id: $id,
            options: vec![$option],
            stops: true
        }
    };
}

lazy_static! {
    pub static ref DIALOGUES: HashMap<u32, Dialogue> = vec!(
        // Old man in main village
        dialogue_with_options!(1, 2, 3),
            dialogue_answer!(2, 4),
                dialogue_stopper!(4, 1),
        dialogue_answer!(3, 5),
            dialogue_stopper!(5, 1)
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