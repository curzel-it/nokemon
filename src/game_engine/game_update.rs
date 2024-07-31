use crate::game_behaviors::{linear_movement::LinearMovement, remove_entities_outside_of_bounds::RemoveEntitiesOutsideOfBounds, shooter::Shooter, update_sprites::UpdateSprites};

use super::{game::Game, game_behavior::GameBehavior};

pub struct GameEngine {
    behaviors: Vec<Box<dyn GameBehavior>>
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            behaviors: vec![
                Box::new(LinearMovement::new()),
                Box::new(UpdateSprites::new()),
                Box::new(Shooter::new()),
                Box::new(RemoveEntitiesOutsideOfBounds::new()),
            ]
        }
    }

    pub fn update(
        &self,
        game: &mut Game, 
        time_since_last_update: f32
    ) {
        let entity_ids: Vec<u32> = game.entities.values().map(|e| e.id).collect();
    
        for behavior in &self.behaviors {
            for id in &entity_ids {
                behavior.update(id, game, time_since_last_update);
            }        
        }
    } 
}

