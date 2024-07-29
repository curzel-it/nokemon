use raylib::math::Vector2;

use crate::{entities::{entity::Entity, entity_capability::{EntityCapability, EntityStateUpdate, GameStateSnapshot}}, features::entity_locator::EntityLocator};

pub struct Shooter {
    entity_locator: EntityLocator
}

impl Shooter {
    pub fn new() -> Self {
        Self {
            entity_locator: EntityLocator::new()
        }
    }
}

impl EntityCapability for Shooter {
    fn update(&self, entity: &Entity, game_state: &GameStateSnapshot, _: f32) -> EntityStateUpdate {
        /*self.entity_locator.list_sorted_entities(
            Vector2::new(entity.frame.x, entity.frame.y), game
        );
        */
        
        return EntityStateUpdate::nothing();
    }
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::RECT_ORIGIN_SQUARE_100, game::game::Game};

    #[test]
    fn can_move_on_update() {
        // ...
    }
}