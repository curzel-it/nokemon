use std::{cell::RefCell, collections::HashMap, fmt::{self, Debug}};

use raylib::math::{Rectangle, Vector2};

use crate::constants::{FRAME_TIME, HERO_ENTITY_ID};

use super::{entity::Entity, entity_factory::EntityFactory, game_state_update::GameStateUpdate, keyboard_events_provider::{KeyboardEventsProvider, KeyboardState}, simple_entity::SimpleEntity};

pub struct Game {
    pub total_elapsed_time: f32,
    pub entity_factory: EntityFactory,
    pub bounds: Rectangle,
    pub entities: RefCell<HashMap<u32, Box<dyn Entity>>>,
    pub selected_entity_id: Option<u32>,
    pub keyboard_state: KeyboardState,
}

        // self.game_defaults.update(&mut game, 0.0);
        /* 
        for id in &game.entity_ids() {
            for behavior in &self.entity_behaviors {
                behavior.update(id, game, time_since_last_update);
            }        
        }
        for behavior in &self.game_behaviors {
            behavior.update(game, time_since_last_update);
        }*/

impl Game {
    pub fn new(entity_factory: EntityFactory, bounds: Rectangle) -> Self {
        Self {
            total_elapsed_time: 0.0,
            entity_factory,
            bounds,
            entities: RefCell::new(HashMap::new()),
            selected_entity_id: None,
            keyboard_state: KeyboardState::default()
        }
    }
    
    pub fn setup(&mut self) {
        // self.add_tower();
        self.add_creep_spawn_point();
        self.add_hero();
        self.selected_entity_id = Some(HERO_ENTITY_ID);
    }

    pub fn is_every_n_seconds(&self, seconds: u32) -> bool {
        let full_second = self.total_elapsed_time.floor();
        let i_full_second = full_second as u32;
        let diff = self.total_elapsed_time - full_second;
        diff < FRAME_TIME && (i_full_second % seconds) == 0
    }
    
    pub fn entity_ids(&self) -> Vec<u32> {
        return self.entities.borrow().values().map(|e| e.id()).collect();
    }

    pub fn add_entity_by_species(&mut self, species_id: &str) -> u32 {
        let body = self.entity_factory.build(species_id);
        let entity = SimpleEntity::new(body);
        self.add_entity(Box::new(entity))
    }

    pub fn add_entity(&mut self, entity: Box<dyn Entity>) -> u32 {
        let id = entity.id();
        self.entities.borrow_mut().insert(id, entity);

        if let Some(new_entity) = self.entities.borrow_mut().get_mut(&id) {
            new_entity.set_creation_time(self.total_elapsed_time);
        }
        id
    }

    pub fn remove_entity(&mut self, id: &u32) {
        self.entities.borrow_mut().remove(id);
    }

    pub fn update_rl(
        &mut self, 
        time_since_last_update: f32,
        keyboard_events: &dyn KeyboardEventsProvider
    ) {
        self.total_elapsed_time += time_since_last_update;
        self.keyboard_state = keyboard_events.keyboard_state();

        let entity_ids = self.entity_ids();
        let mut entities = self.entities.borrow_mut();

        let mut state_updates: Vec<GameStateUpdate> = vec![];

        for id in &entity_ids {
            if let Some(entity) = entities.get_mut(id) {
                let mut updates = entity.update(self, time_since_last_update);
                state_updates.append(&mut updates);
            }
        }

        drop(entities);
        self.apply_state_updates(state_updates);
    } 

    fn apply_state_updates(&mut self, updates: Vec<GameStateUpdate>) {
        for update in updates {
            self.apply_state_update(update)
        }
    }

    fn apply_state_update(&mut self, update: GameStateUpdate) {
        match update {
            GameStateUpdate::AddEntity(entity) => { self.add_entity(entity); },
            GameStateUpdate::RemoveEntity(id) => { self.remove_entity(&id); },
            GameStateUpdate::SelectEntity(id) => { self.selected_entity_id = Some(id) },
        };
    }

    /* 
    pub fn move_entity_by(&mut self, id: u32, offset: Vector2) {
        let entity = self.entities.get_mut(&id);
        if let Some(entity) = entity {
            entity.frame.x += offset.x;
            entity.frame.y += offset.y;
        }
    }

    pub fn selected_entity(&self) -> Option<&Entity> {
        if let Some(id) = self.selected_entity_id {
            return Some(&self.entities[&id]);
        } else {
            return None;
        }
    }

    pub fn selected_entity_mut(&mut self) -> Option<&mut Box<dyn Entity>> {
        let mut entities = self.entities.borrow_mut();

        if let Some(id) = self.selected_entity_id {
            if let Some(entity_mut) = entities.get_mut(&id) {
                return Some(entity_mut);
            }
        }
        return None;
    }

    pub fn hero(&self) -> Option<&Entity> {
        return self.entities.get(&HERO_ENTITY_ID);
    }
*/
    pub fn hero_frame(&mut self) -> Rectangle {
        if let Some(entity) = self.entities.borrow().get(&HERO_ENTITY_ID) {
            return entity.frame();
        }
        Rectangle::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn hero_position(&mut self) -> Vector2 {
        let frame = self.hero_frame();
        Vector2::new(frame.x, frame.y)
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

    use crate::{constants::RECT_ORIGIN_FULL_HD, game_engine::{entity_factory::EntityFactory, keyboard_events_provider::NoKeyboard}};

    use super::Game;

    impl Game {
        pub fn test() -> Game {
            Game::new(
                EntityFactory::test(), 
                RECT_ORIGIN_FULL_HD
            )
        }
        
        pub fn frame_of_entity(&self, id: &u32) -> Rectangle {
            let entities = self.entities.borrow();
            let entity = entities.get(id).unwrap();
            entity.frame()
        }
        
        pub fn update(&mut self, time_since_last_update: f32) {
            let nokb = NoKeyboard {};
            self.update_rl(time_since_last_update, &nokb)
        }
        
        pub fn animation_name_of_entity(&self, id: &u32) -> String {
            let entities = self.entities.borrow();
            let entity = entities.get(id).unwrap();
            return entity.current_animation().to_string();
        }
    }
}