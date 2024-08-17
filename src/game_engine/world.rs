use std::{cell::RefCell, collections::{HashMap, HashSet}, fmt::{self, Debug}};

use common_macros::hash_set;
use raylib::math::{Rectangle, Vector2};

use crate::{constants::{HERO_ENTITY_ID, RECT_ORIGIN_SQUARE_100}, maps::{biome_tiles::BiomeTile, constructions_tiles::ConstructionTile, tiles::TileSet}};

use super::{collision_detection::{compute_collisions, Collision}, entity::Entity, keyboard_events_provider::{KeyboardEventsProvider, KeyboardState}, state_updates::{EngineStateUpdate, WorldStateUpdate}, visible_entities::compute_visible_entities};

pub struct World {
    pub total_elapsed_time: f32,
    pub bounds: Rectangle,
    pub biome_tiles: TileSet<BiomeTile>,
    pub constructions_tiles: TileSet<ConstructionTile>,
    pub entities: RefCell<HashMap<u32, Box<dyn Entity>>>,    
    pub visible_entities: HashSet<u32>,
    pub selected_entity_id: Option<u32>,
    pub keyboard_state: KeyboardState,
    pub cached_hero_frame: Rectangle,
    pub cached_hero_position: Vector2,
    pub collisions: HashMap<u32, Vec<Collision>>
}

impl World {
    pub fn new() -> Self {
        Self {
            total_elapsed_time: 0.0,
            bounds: RECT_ORIGIN_SQUARE_100,
            biome_tiles: TileSet::empty(),
            constructions_tiles: TileSet::empty(),
            entities: RefCell::new(HashMap::new()),
            visible_entities: hash_set![],
            selected_entity_id: None,
            keyboard_state: KeyboardState::default(),
            cached_hero_frame: Rectangle::new(0.0, 0.0, 1.0, 1.0),
            cached_hero_position: Vector2::zero(),
            collisions: HashMap::new()
        }
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
        viewport: &Rectangle,
        keyboard_events: &dyn KeyboardEventsProvider
    ) -> Vec<EngineStateUpdate> {
        self.total_elapsed_time += time_since_last_update;
        self.keyboard_state = keyboard_events.keyboard_state();
        self.visible_entities = compute_visible_entities(self, viewport);
        self.collisions = compute_collisions(self);

        let mut state_updates: Vec<WorldStateUpdate> = vec![];
        let mut entities = self.entities.borrow_mut();

        for id in &self.visible_entities {
            if let Some(entity) = entities.get_mut(id) {
                let mut updates = entity.update(self, time_since_last_update);
                state_updates.append(&mut updates);
            }
        }

        self.biome_tiles.update(time_since_last_update);
        // self.constructions_tiles.update(time_since_last_update);

        drop(entities);
        self.store_updated_hero_state();
        self.apply_state_updates(state_updates)
    } 

    fn apply_state_updates(&mut self, updates: Vec<WorldStateUpdate>) -> Vec<EngineStateUpdate> {
        updates.into_iter().filter_map(|u| self.apply_state_update(u)).collect()
    }

    fn apply_state_update(&mut self, update: WorldStateUpdate) -> Option<EngineStateUpdate> {
        match update {
            WorldStateUpdate::AddEntity(entity) => { 
                self.add_entity(entity); 
            },
            WorldStateUpdate::RemoveEntity(id) => { 
                self.remove_entity(&id); 
            },
            WorldStateUpdate::IncreaseHp(id, value) => { 
                if let Some(entity) = self.entities.borrow_mut().get_mut(&id) {
                    entity.body_mut().hp += value;
                }
            },
            WorldStateUpdate::EngineUpdate(update) => return Some(update)
        };
        None
    }
    
    fn store_updated_hero_state(&mut self) {
        if let Some(entity) = self.entities.borrow().get(&HERO_ENTITY_ID) {
            self.cached_hero_frame = entity.body().frame;
            self.cached_hero_position = Vector2::new(self.cached_hero_frame.x, self.cached_hero_frame.y);
        }
    }

    pub fn visible_biome_tiles(&self, viewport: &Rectangle) -> Vec<&BiomeTile> {
        self.biome_tiles.visible_tiles(viewport)
    }

    pub fn visible_construction_tiles(&self, viewport: &Rectangle) -> Vec<&ConstructionTile> {
        self.constructions_tiles.visible_tiles(viewport)
    }
}

impl Debug for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Game")
            .field("bounds", &self.bounds)
            .field("entities", &self.entities)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::game_engine::{keyboard_events_provider::NoKeyboard, state_updates::EngineStateUpdate};

    use super::World;

    impl World {        
        pub fn update(&mut self, time_since_last_update: f32) -> Vec<EngineStateUpdate> {
            let nokb = NoKeyboard {};
            let viewport = self.bounds;
            self.update_rl(time_since_last_update, &viewport, &nokb)
        }
    }
}