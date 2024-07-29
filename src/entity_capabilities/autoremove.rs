use crate::entities::{entity::EntityStateSnapshot, entity_capability::{EntityCapability, EntityStateUpdate, GameStateSnapshot}};

#[derive(Debug)]
pub struct Autoremove;

impl Autoremove {
    pub fn new() -> Self {
        Self {}
    }
}

impl EntityCapability for Autoremove {
    fn update(&mut self, entity: &EntityStateSnapshot, game: &GameStateSnapshot, _: f32) -> EntityStateUpdate {
        if game.bounds.get_collision_rec(&entity.frame) == None {
            EntityStateUpdate::remove_entity(entity.id)
        } else {
            EntityStateUpdate::nothing()
        }
    }
}

#[cfg(test)]
mod tests {
    use raylib::math::Vector2;

    use crate::{constants::RECT_ORIGIN_SQUARE_100, game::game::Game};

    #[test]
    fn can_remove_automatically_when_leaving_screen() {
        let mut game = Game::test();
        
        let mut entity = game.entity_factory.build("towerdart");
        entity.frame = RECT_ORIGIN_SQUARE_100;
        entity.speed = 100.0;
        entity.direction = Vector2::new(-1.0, 0.0);
        game.add_entity(entity);
                
        game.update(0.6);
        assert_eq!(game.entities.len(), 1);
                
        game.update(0.6);
        assert_eq!(game.entities.len(), 1);
                
        game.update(0.6);
        assert_eq!(game.entities.len(), 0);
    }
}