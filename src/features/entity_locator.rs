use raylib::math::Vector2;

use crate::game_engine::game::Game;

#[derive(Debug)]
pub struct EntityLocator;

impl EntityLocator {
    pub fn new() -> Self {
        EntityLocator {}
    }

    pub fn find_by_position(&self, game: &Game, position: &Vector2) -> Option<u32> {
        for entity in game.entities.values() {
            if entity.frame().check_collision_point_rec(position) {
                return Some(entity.id());
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use raylib::math::Rectangle;

    use crate::game_engine::simple_entity::SimpleEntity;

    use super::*;

    #[test]
    fn can_return_sorted_list() {
        let locator = EntityLocator::new();
        let mut game = Game::test();

        for index in 0..10 {
            let mut body = game.entity_factory.build("red");
            body.id = index;
            body.frame = Rectangle::new((index as f32) * 100.0, 0.0, 10.0, 10.0);
            let entity = SimpleEntity::new(body);
            game.add_entity(Box::new(entity));
        }

        for index in 0..10 {
            let position = Vector2::new(5.0 + (index as f32) * 100.0, 5.0);
            let id = locator.find_by_position(&game, &position);
            assert!(id.is_some());
            assert_eq!(id.unwrap(), index as u32);
        }
    }
}