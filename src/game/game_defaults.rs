use super::{game::Game, game_capability::{GameCapability, GameStateUpdate}};

pub struct GameDefaultsLoader;

impl GameDefaultsLoader {
    pub fn new() -> Self {
        Self {}
    }
}

impl GameCapability for GameDefaultsLoader {
    fn update(&self, game: &Game, time_since_last_update: f32) -> GameStateUpdate {
        if !game.is_first_update { 
            return GameStateUpdate::nothing();
        }
        
        return GameStateUpdate { 
            entities_to_remove: vec![],
            new_entities: vec![
                game.entity_factory.build("ape"),
                game.entity_factory.build("tower")
            ]
        };
    }
}