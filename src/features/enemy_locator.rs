use raylib::math::Vector2;

use crate::{entities::entity::Entity, game::game::Game};

use super::sorting::sort_by_distance;

pub struct EnemyLocator {}

impl EnemyLocator {
    pub fn new() -> Self {
        EnemyLocator {}
    }

    pub fn list_enemies<'a>(&self, game: &'a Game) -> Vec<&'a Entity> {
        game.entities.iter()
            .filter(|entity| entity.is_enemy)
            .collect()
    }

    pub fn list_sorted_enemies<'a>(&self, origin: Vector2, game: &'a Game) -> Vec<&'a Entity> {
        let mut enemies: Vec<&Entity> = game.entities.iter()
            .filter(|entity| entity.is_enemy)
            .collect();

        sort_by_distance(origin, &mut enemies);
        return enemies;
    }
}

#[cfg(test)]
mod tests {
    use raylib::math::Rectangle;

    use super::*;

    #[test]
    fn can_return_empty_list_for_empty_game() {
        let game = Game::test();
        let locator = EnemyLocator::new();
        let results = locator.list_enemies(&game);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn can_return_filtered_list() {
        let mut game = Game::test();

        let mut enemy1 = game.entity_factory.build("ape");
        enemy1.is_enemy = true;
        game.add_entity(enemy1);

        let mut enemy2 = game.entity_factory.build("ape");
        enemy2.is_enemy = true;
        game.add_entity(enemy2);

        let mut friend = game.entity_factory.build("ape");
        friend.is_enemy = false;
        game.add_entity(friend);

        let locator = EnemyLocator::new();
        let results = locator.list_enemies(&game);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn can_return_sorted_list() {
        let mut game = Game::test();

        let mut enemy1 = game.entity_factory.build("ape");
        enemy1.is_enemy = true;
        enemy1.frame = Rectangle::new(100.0, 0.0, 1.0, 1.0);
        game.add_entity(enemy1);

        let mut enemy2 = game.entity_factory.build("tower");
        enemy2.is_enemy = true;
        enemy2.frame = Rectangle::new(50.0, 0.0, 1.0, 1.0);
        game.add_entity(enemy2);

        let mut enemy3 = game.entity_factory.build("cybertruck");
        enemy3.is_enemy = true;
        enemy3.frame = Rectangle::new(10.0, 0.0, 1.0, 1.0);
        game.add_entity(enemy3);

        let locator = EnemyLocator::new();
        let origin = Vector2::new(0.0, 0.0);
        let results = locator.list_sorted_enemies(origin, &game);
        let result_species: Vec<String> = results.iter().map(|e| e.species.clone()).collect();
        assert_eq!(results.len(), 3);
        assert_eq!(result_species, vec!["cybertruck".to_owned(), "tower".to_owned(), "ape".to_owned()]);
    }
}