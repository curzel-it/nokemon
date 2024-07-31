use std::{collections::{HashMap, HashSet}, fmt::{self, Debug}};

use raylib::math::{Rectangle, Vector2};

use super::{entity::Entity, entity_factory::EntityFactory};

pub struct Game {
    pub entity_factory: EntityFactory,
    pub bounds: Rectangle,
    pub entities: HashMap<u32, Entity>,
    pub bullets: HashSet<u32>,
}

impl Game {
    pub fn new(entity_factory: EntityFactory, bounds: Rectangle) -> Self {
        Self {
            entity_factory,
            bounds,
            entities: HashMap::new(),
            bullets: HashSet::new(),
        }
    }

    pub fn add_entity_by_species(&mut self, species_id: &str) {
        let entity = self.entity_factory.build(species_id);
        return self.add_entity(entity);
    }

    pub fn add_entity(&mut self, entity: Entity) {
        let id = entity.id;
        let is_bullet = entity.is_bullet;

        self.entities.insert(id, entity);

        if is_bullet {
            self.bullets.insert(id);
        }
    }

    pub fn remove_entity(&mut self, id: &u32) {
        self.entities.remove(&id);
        self.bullets.remove(&id);
    }

    pub fn move_entity_by(&mut self, id: u32, offset: Vector2) {
        let entity = self.entities.get_mut(&id);
        if let Some(entity) = entity {
            entity.frame.x += offset.x;
            entity.frame.y += offset.y;
        }
    }

    pub fn move_entity_to(&mut self, id: u32, offset: Vector2) {
        let entity = self.entities.get_mut(&id);
        if let Some(entity) = entity {
            entity.frame.x = offset.x;
            entity.frame.y = offset.y;
        }
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Game")
            .field("bounds", &self.bounds)
            .field("entities", &self.entities)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use raylib::math::Rectangle;

    use crate::{constants::RECT_ORIGIN_FULL_HD, game_engine::{entity::Entity, entity_factory::EntityFactory}};

    use super::Game;

    impl Game {
        pub fn test() -> Game {
            return Game::new(
                EntityFactory::test(), 
                RECT_ORIGIN_FULL_HD
            );
        }       
    }

    impl Game {
        pub fn frame_of_first_entity(&self) -> Rectangle {
            let entities: Vec<&Entity> = self.entities.values().collect();
            let entity = entities.first().unwrap();
            return entity.frame;
        }
    }
}