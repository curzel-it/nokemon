use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::game_engine::world::World;

#[derive(Debug, Copy, Clone)]
pub struct Dialogue {
    pub id: u32
}

impl Dialogue {
    fn new(id: u32) -> Self {
        Self { id }
    }
}

pub fn next_dialogue(npc_id: u32, world: &World) -> Option<Dialogue> {
    if let Some(dialogue) = DIALOGUE_TREE.get(&npc_id) {
        Some(dialogue.clone())
    } else {
        None
    }
}

lazy_static! {
    static ref DIALOGUE_TREE: HashMap<u32, Dialogue> = {
        let mut m = HashMap::new();
        m.insert(1001, Dialogue::new(1001));
        m
    };
}
