use crate::{game_engine::world::World, lang::localizable::LocalizableText};

#[derive(Debug, Clone)]
pub struct Dialogue {
    pub npc_id: u32,
    pub chain: Vec<u32>,
    pub options: Vec<u32>
}

impl Dialogue {
    pub fn localized_text(&self) -> String {
        self.localization_key(None).localized()
    }

    pub fn localized_options(&self) -> Vec<(u32, String)> {
        self.options
            .iter()
            .map(|id| {
                let key = self.localization_key(Some(id.to_string()));
                (*id, key.localized())
            })
            .collect()
    }

    fn localization_key(&self, extra: Option<String>) -> String {
        let mut chain_str: Vec<String> = self.chain.iter().map(|n| n.to_string()).collect();
        chain_str.insert(0, "dialogue".to_string());
        chain_str.insert(1, self.npc_id.to_string());

        if let Some(extra) = extra {
            chain_str.push(extra);
        }
        chain_str.join(".")
    }
}

pub fn next_dialogue(npc_id: u32, world: &World) -> Option<Dialogue> {
    Some(Dialogue { npc_id, chain: vec![0], options: vec![0, 1] })
}