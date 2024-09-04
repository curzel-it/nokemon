use std::{collections::HashMap, sync::RwLock};

use lazy_static::lazy_static;

use crate::lang::localizable::LocalizableText;

#[derive(Debug, Clone)]
pub struct Dialogue {
    pub id: String,
    pub options: Vec<Dialogue>
}

impl Dialogue {
    fn new(id: &str, options: Vec<Dialogue>) -> Self {
        Self { id: id.to_string(), options }
    }

    pub fn localized_text(&self) -> String {
        format!("dialogue.{}", self.id).localized()
    }
}

type NpcId = u32;
type DialogueSelection = usize;

const NO_SELECTIONS: Vec<usize> = vec![];

lazy_static! {
    pub static ref DIALOGUE_SELECTIONS: RwLock<HashMap<NpcId, Vec<DialogueSelection>>> = RwLock::new(HashMap::new());    

    pub static ref DIALOGUES: HashMap<NpcId, Dialogue> = vec!(
        (1001, Dialogue::new("1001", vec![
            Dialogue::new("1001.0", vec![
                Dialogue::new("1001.0.0", vec![]),
                Dialogue::new("1001.0.1", vec![]),
            ]),
            Dialogue::new("1001.1", vec![
                Dialogue::new("1001.1.0", vec![]),
                Dialogue::new("1001.1.1", vec![]),
            ])
        ]))
    ).into_iter().collect();
}

pub fn current_dialogue(npc_id: u32) -> Option<Dialogue> {
    if let Some(mut dialogue) = DIALOGUES.get(&npc_id).cloned() {
        let selections = DIALOGUE_SELECTIONS.read().unwrap().get(&npc_id).unwrap_or(&NO_SELECTIONS).clone();

        for choice in selections {
            dialogue = dialogue.options[choice].clone();
        }
        return Some(dialogue);
    }
    None
}

pub fn next_dialogue(npc_id: u32, new_choice: usize) -> Option<Dialogue> {
    let mut selections = DIALOGUE_SELECTIONS.write().unwrap();
    let mut chain = selections.get(&npc_id).unwrap_or(&NO_SELECTIONS).clone();
    chain.push(new_choice);
    selections.insert(npc_id, chain.clone());
    current_dialogue(npc_id)
}