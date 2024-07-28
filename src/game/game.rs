use std::fmt::{self, Debug};

use raylib::math::{Rectangle, Vector2};

use crate::entities::{entity::Entity, factory::EntityFactory};

use super::rendered_item::RenderedItem;

pub struct Game {
    pub entity_factory: EntityFactory,
    bounds: Rectangle,
    pub entities: Vec<Entity>,
    did_add_defaults: bool
}

impl Game {
    pub fn new(entity_factory: EntityFactory, bounds: Rectangle) -> Self {
        Self {
            entity_factory,
            bounds,
            entities: Vec::new(),
            did_add_defaults: false
        }
    }

    pub fn update(&mut self, time_since_last_update: f32) {
        for entity in &mut self.entities {
            entity.update(time_since_last_update);
        }
        self.spawn_defaults();
        self.spawn_creeps();
    }

    fn spawn_defaults(&mut self) {
        if self.did_add_defaults { return; }
        self.did_add_defaults = true;
        self.add_entity_by_species("ape");
        self.add_entity_by_species("tower");
    }

    fn spawn_creeps(&mut self) {
        
    }

    pub fn add_entity_by_species(&mut self, species_id: &str) -> &Entity {
        let entity = self.entity_factory.build(species_id);
        return self.add_entity(entity);
    }

    pub fn add_entity(&mut self, entity: Entity) -> &Entity {
        self.entities.push(entity);
        return self.entities.last().unwrap();
    }

    pub fn move_entity_by(&mut self, id: u32, offset: Vector2) {
        for entity in &mut self.entities {
            if entity.id == id {
                entity.frame.x += offset.x;
                entity.frame.y += offset.y;
            }
        }
    }

    pub fn move_entity_to(&mut self, id: u32, offset: Vector2) {
        for entity in &mut self.entities {
            if entity.id == id {
                entity.frame.x = offset.x;
                entity.frame.y = offset.y;
            }
        }
    }

    pub fn render(&self) -> Vec<RenderedItem> {
        return self.entities.iter().map(|e| e.render()).collect();
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Game")
            .field("x", &self.bounds.x)
            .field("y", &self.bounds.y)
            .field("width", &self.bounds.width)
            .field("height", &self.bounds.height)
            .field("entities", &self.entities)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::{constants::RECT_ORIGIN_FULL_HD, entities::factory::EntityFactory};

    use super::Game;

    impl Game {
        pub fn test() -> Self {
            Self {
                entity_factory: EntityFactory::test(),
                bounds: RECT_ORIGIN_FULL_HD,
                entities: Vec::new(),
                did_add_defaults: false
            }
        }       
    }
}