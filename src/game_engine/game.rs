use std::{collections::{HashMap, HashSet}, fmt::{self, Debug}};

use raylib::math::{Rectangle, Vector2};

use crate::constants::{FRAME_TIME, HERO_ENTITY_ID};

use super::{entity::Entity, entity_factory::EntityFactory, keyboard_events_provider::KeyboardState};

pub struct Game {
    pub total_elapsed_time: f32,
    pub entity_factory: EntityFactory,
    pub bounds: Rectangle,
    pub entities: HashMap<u32, Entity>,
    pub selected_entity_id: Option<u32>,
    pub bullets: HashSet<u32>,
    pub keyboard_state: KeyboardState
}

impl Game {
    pub fn new(entity_factory: EntityFactory, bounds: Rectangle) -> Self {
        Self {
            total_elapsed_time: 0.0,
            entity_factory,
            bounds,
            entities: HashMap::new(),
            selected_entity_id: None,
            bullets: HashSet::new(),
            keyboard_state: KeyboardState::default()
        }
    }

    pub fn is_every_n_seconds(&self, seconds: u32) -> bool {
        let full_second = self.total_elapsed_time.floor();
        let i_full_second = full_second as u32;
        let diff = self.total_elapsed_time - full_second;
        return diff < FRAME_TIME && (i_full_second % seconds) == 0;
    }
    
    pub fn entity_ids(&self) -> Vec<u32> {
        return self.entities.values().map(|e| e.id).collect();
    }

    pub fn add_entity_by_species(&mut self, species_id: &str) -> u32 {
        let entity = self.entity_factory.build(species_id);
        return self.add_entity(entity);
    }

    pub fn add_entity(&mut self, entity: Entity) -> u32 {
        let id = entity.id;
        let is_bullet = entity.species.is_bullet;

        self.entities.insert(id, entity);

        if is_bullet {
            self.bullets.insert(id);
        }
        if let Some(new_entity) = self.entities.get_mut(&id) {
            new_entity.creation_time = self.total_elapsed_time;
        }
        return id;
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

    /* 
    pub fn selected_entity(&self) -> Option<&Entity> {
        if let Some(id) = self.selected_entity_id {
            return Some(&self.entities[&id]);
        } else {
            return None;
        }
    }*/

    pub fn selected_entity_mut(&mut self) -> Option<&mut Entity> {
        if let Some(id) = self.selected_entity_id {
            if let Some(entity_mut) = self.entities.get_mut(&id) {
                return Some(entity_mut);
            }
        }
        return None;
    }

    pub fn hero(&self) -> Option<&Entity> {
        return self.entities.get(&HERO_ENTITY_ID);
    }

    pub fn hero_frame(&mut self) -> Rectangle {
        if let Some(entity) = self.entities.get(&HERO_ENTITY_ID) {
            return entity.frame;
        }
        return Rectangle::new(0.0, 0.0, 0.0, 0.0);
    }

    pub fn hero_position(&mut self) -> Vector2 {
        let frame = self.hero_frame();
        return Vector2::new(frame.x, frame.y);
    }

    /* 
    pub fn total_elapsed_time_s(&self) -> u32 {
        return self.total_elapsed_time.floor() as u32;
    }

    pub fn total_elapsed_time_ms(&self) -> u32 {
        return (self.total_elapsed_time * 1000.0).floor() as u32;
    }*/
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

    use crate::{constants::RECT_ORIGIN_FULL_HD, game_engine::entity_factory::EntityFactory};

    use super::Game;

    impl Game {
        pub fn test() -> Game {
            return Game::new(
                EntityFactory::test(), 
                RECT_ORIGIN_FULL_HD
            );
        }
        
        pub fn frame_of_entity(&self, id: &u32) -> Rectangle {
            return self.entities.get(id).unwrap().frame;
        }
        
        pub fn animation_name_of_entity(&self, id: &u32) -> String {
            return self.entities.get(id).unwrap().current_sprite.animation_name.clone();
        }
    }
}