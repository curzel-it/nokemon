use std::{collections::HashMap, fmt::{self, Debug}};

use raylib::math::{Rectangle, Vector2};

use crate::{entities::{entity::Entity, entity_capability::GameStateSnapshot, factory::EntityFactory}, features::entity_locator::EntityLocator, game_capabilities::game_defaults::GameDefaultsLoader};

use super::{game_capability::{GameCapability, GameStateUpdate}, rendered_item::RenderedItem};

pub struct Game {
    pub entity_factory: EntityFactory,
    bounds: Rectangle,
    pub entities: HashMap<u32, Entity>,
    pub capabilities: Vec<Box<dyn GameCapability>>,
    entity_locator: EntityLocator
}

impl Game {
    pub fn new(entity_factory: EntityFactory, bounds: Rectangle, capabilities: Vec<Box<dyn GameCapability>>) -> Self {
        Self {
            entity_factory,
            bounds,
            entities: HashMap::new(),
            capabilities: capabilities,
            entity_locator: EntityLocator::new()
        }
    }

    pub fn update(&mut self, time_since_last_update: f32) {
        let state = self.state_snapshot();
        let mut updates: Vec<GameStateUpdate> = vec![];

        for (_, entity) in &mut self.entities {
            let update = entity.update(&state, time_since_last_update);
            updates.push(update);
        }
        for capabilty in &mut self.capabilities {
            let update = capabilty.update(&state, time_since_last_update);
            updates.push(update);
        }
        for update in updates {
            self.apply(update);
        }
    }

    fn state_snapshot(&self) -> GameStateSnapshot {
        GameStateSnapshot {
            enemies: self.entities.values()
                .filter(|entity| entity.is_enemy)
                .map(|e| e.state_snapshot())
                .collect()
        }
    }

    fn apply(&mut self, update: GameStateUpdate) {
        self.remove_entities(update.entities_to_remove);

        for descriptor in update.new_entities {
            let entity = self.entity_factory.build_ex(&descriptor);
            self.add_entity(entity);
        }        
    }

    fn add_entities(&mut self, entities: Vec<Entity>) {
        for entity in entities {
            self.add_entity(entity);
        }
    }

    fn remove_entities(&mut self, ids: Vec<u32>) {
        for id in ids {
            self.entities.remove(&id);
        }
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

    pub fn move_entity_by(&mut self, id: u32, offset: Vector2) {
        let entity = self.entities.get_mut(&id).unwrap();
        entity.frame.x += offset.x;
        entity.frame.y += offset.y;
    }

    pub fn move_entity_to(&mut self, id: u32, offset: Vector2) {
        let entity = self.entities.get_mut(&id).unwrap();
        entity.frame.x = offset.x;
        entity.frame.y = offset.y;
    }

    pub fn render(&self) -> Vec<RenderedItem> {
        return self.entities.values().map(|e| e.render()).collect();
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
    use raylib::math::Rectangle;

    use crate::{constants::RECT_ORIGIN_FULL_HD, entities::{entity::Entity, factory::EntityFactory}};

    use super::Game;

    impl Game {
        pub fn test() -> Game {
            return Game::new(
                EntityFactory::test(), 
                RECT_ORIGIN_FULL_HD,
                vec![]
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