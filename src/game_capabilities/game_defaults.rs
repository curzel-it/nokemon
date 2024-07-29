use crate::entities::entity_capability::GameStateSnapshot;
use crate::entities::factory::EntityDescriptor;
use crate::game::game_capability::{GameCapability, GameStateUpdate};

pub struct GameDefaultsLoader {
    is_first_update: bool
}

impl GameDefaultsLoader {
    pub fn new() -> Self {
        Self {
            is_first_update: true
        }
    }
}

impl GameCapability for GameDefaultsLoader {
    fn update(&mut self, game: &GameStateSnapshot, _: f32) -> GameStateUpdate {
        if !self.is_first_update { 
            return GameStateUpdate::nothing();
        }
        self.is_first_update = false;
        
        return GameStateUpdate { 
            entities_to_remove: vec![],
            new_entities: vec![
                EntityDescriptor::for_species("ape"),
                EntityDescriptor::for_species("tower"),
            ]
        };
    }
}