use std::collections::HashMap;

use common_macros::hash_map;

lazy_static::lazy_static! {
    pub static ref LOCALIZED_STRINGS: HashMap<String, HashMap<String, String>> = {
        hash_map!{
            "en".to_string() => hash_map!{
                "game.menu.title".to_string() => "Game Menu".to_string(),
                "game.menu.save".to_string() => "Save Game".to_string(),
                "game.menu.map_editor".to_string() => "Map Editor".to_string(),
                "game.menu.save_and_exit".to_string() => "Save & Exit".to_string(),
                
                "entity.menu.title".to_string() => "Entity Options".to_string(),
                "entity.menu.remove".to_string() => "Remove".to_string(),
            },
        }
    };
}