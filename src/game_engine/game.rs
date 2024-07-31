use std::{collections::HashMap, fmt::{self, Debug}};

use raylib::math::{Rectangle, Vector2};

use super::{entity::Entity, entity_factory::EntityFactory};

#[derive(Debug)]
pub struct Game {
    pub entity_factory: EntityFactory,
    pub bounds: Rectangle,
    pub entities: HashMap<u32, Entity>,
}

impl Game {
    pub fn new(entity_factory: EntityFactory, bounds: Rectangle) -> Self {
        Self {
            entity_factory,
            bounds,
            entities: HashMap::new(),
        }
    }

    pub fn setup(&mut self) {
        self.add_entity_by_species("ape");
        self.add_entity_by_species("tower");
    }

    pub fn add_entity_by_species(&mut self, species_id: &str) -> &Entity {
        let entity = self.entity_factory.build(species_id);
        return self.add_entity(entity);
    }

    pub fn add_entity(&mut self, entity: Entity) -> &Entity {
        let id = entity.id;
        self.entities.insert(id, entity);
        return self.entities.get(&id).unwrap();
    }

    pub fn add_entities(&mut self, entities: Vec<Entity>) {
        for entity in entities {
            self.add_entity(entity);
        }
    }

    pub fn remove_entity(&mut self, id: &u32) {
        self.entities.remove(&id);
    }

    pub fn remove_entities(&mut self, ids: &Vec<u32>) {
        for id in ids {
            self.remove_entity(id);
        }
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