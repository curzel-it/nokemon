use crate::entities::{entity::Entity, entity_capability::{EntityCapability, EntityStateUpdate, GameStateSnapshot}};

pub struct LinearMovement;

impl LinearMovement {
    pub fn new() -> Self {
        Self {}
    }
}

impl EntityCapability for LinearMovement {
    fn update(&self, entity: &Entity, _: &GameStateSnapshot, time_since_last_update: f32) -> EntityStateUpdate {
        let offset = entity.direction * entity.speed * time_since_last_update;
        
        let mut updated_frame = entity.frame;
        updated_frame.x += offset.x;
        updated_frame.y += offset.y;
                
        return EntityStateUpdate::frame(updated_frame);
    }
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::RECT_ORIGIN_SQUARE_100, game::game::Game};

    #[test]
    fn can_move_on_update() {
        let mut game = Game::test();
        
        let mut entity = game.entity_factory.build("ape");
        entity.frame = RECT_ORIGIN_SQUARE_100;
        entity.direction = Vector2::new(1.0, 1.0);
        game.add_entity(entity);
                
        game.update(1.0);
        let result = game.frame_of_first_entity();
        assert_eq!(result.x, 30.0);
        assert_eq!(result.y, 30.0);
    }
}