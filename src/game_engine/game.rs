use std::{cell::RefCell, collections::HashMap, fmt::{self, Debug}};

use raylib::math::{Rectangle, Vector2};

use crate::constants::{FRAME_TIME, HERO_ENTITY_ID};

use super::{collision_detection::compute_collisions, entity::Entity, entity_factory::EntityFactory, game_state_update::GameStateUpdate, keyboard_events_provider::{KeyboardEventsProvider, KeyboardState}, simple_entity::SimpleEntity};

pub struct Game {
    pub total_elapsed_time: f32,
    pub entity_factory: EntityFactory,
    pub bounds: Rectangle,
    pub outer_bounds: Rectangle,
    pub entities: RefCell<HashMap<u32, Box<dyn Entity>>>,
    pub selected_entity_id: Option<u32>,
    pub keyboard_state: KeyboardState,
    pub cached_hero_frame: Rectangle,
    pub cached_hero_position: Vector2,
    pub collisions: HashMap<u32, Vec<u32>>
}

impl Game {
    pub fn new(entity_factory: EntityFactory, bounds: Rectangle) -> Self {
        let outer_bounds = Rectangle::new(bounds.x - 100.0, bounds.y - 100.0, bounds.width + 200.0, bounds.height + 200.0);

        Self {
            total_elapsed_time: 0.0,
            entity_factory,
            bounds,
            outer_bounds,
            entities: RefCell::new(HashMap::new()),
            selected_entity_id: None,
            keyboard_state: KeyboardState::default(),
            cached_hero_frame: Rectangle::new(0.0, 0.0, 1.0, 1.0),
            cached_hero_position: Vector2::zero(),
            collisions: HashMap::new()
        }
    }
    
    pub fn setup(&mut self) {
        self.add_creep_spawn_point();
        self.add_tower();
        self.add_hero();
        self.selected_entity_id = Some(HERO_ENTITY_ID);
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
            new_entity.body_mut().creation_time = self.total_elapsed_time;
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

        self.collisions = compute_collisions(self);

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
        self.store_updated_hero_state();
        self.apply_state_updates(state_updates);
    } 

    fn apply_state_updates(&mut self, updates: Vec<GameStateUpdate>) {
        for update in updates {
            self.apply_state_update(update)
        }
    }

    fn apply_state_update(&mut self, update: GameStateUpdate) {
        match update {
            GameStateUpdate::AddEntity(entity) => { 
                self.add_entity(entity); 
            },
            GameStateUpdate::RemoveEntity(id) => { 
                self.remove_entity(&id); 
            },
            GameStateUpdate::IncreaseHp(id, value) => { 
                if let Some(entity) = self.entities.borrow_mut().get_mut(&id) {
                    entity.body_mut().hp += value;
                }
            }
        };
    }
    
    fn store_updated_hero_state(&mut self) {
        if let Some(entity) = self.entities.borrow().get(&HERO_ENTITY_ID) {
            self.cached_hero_frame = entity.body().frame;
            self.cached_hero_position = Vector2::new(self.cached_hero_frame.x, self.cached_hero_frame.y);
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
            entity.body().frame
        }
        
        pub fn update(&mut self, time_since_last_update: f32) {
            let nokb = NoKeyboard {};
            self.update_rl(time_since_last_update, &nokb)
        }
        
        pub fn animation_name_of_entity(&self, id: &u32) -> String {
            let entities = self.entities.borrow();
            let entity = entities.get(id).unwrap();
            return entity.body().current_sprite.animation_name.to_owned();
        }
    }
}