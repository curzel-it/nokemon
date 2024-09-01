use std::{cell::RefCell, collections::{HashMap, HashSet}, fmt::{self, Debug}};

use common_macros::hash_set;
use crate::{constants::{WORLD_SIZE_COLUMNS, WORLD_SIZE_ROWS}, entities::teleporter::Teleporter, features::hitmap::Hitmap, maps::{biome_tiles::{Biome, BiomeTile}, constructions_tiles::{Construction, ConstructionTile}, tiles::TileSet}, utils::{directions::Direction, rect::Rect}};

use super::{entity::{Entity, EntityProps}, entity_body::EmbodiedEntity, keyboard_events_provider::{KeyboardEventsProvider, NO_KEYBOARD_EVENTS}, state_updates::{EngineStateUpdate, WorldStateUpdate}};

pub struct World {
    pub id: u32,
    pub total_elapsed_time: f32,
    pub bounds: Rect,
    pub biome_tiles: TileSet<BiomeTile>,
    pub constructions_tiles: TileSet<ConstructionTile>,
    pub entities: RefCell<HashMap<u32, Box<dyn Entity>>>,    
    pub visible_entities: HashSet<u32>,
    pub cached_hero_props: EntityProps,
    pub hitmap: Hitmap,
    pub creative_mode: bool,
    pub direction_based_on_current_keys: Direction,
    pub is_any_arrow_key_down: bool,
    pub has_confirmation_key_been_pressed: bool,
}

impl World {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            total_elapsed_time: 0.0,
            bounds: Rect::square_from_origin(100),
            biome_tiles: TileSet::empty(),
            constructions_tiles: TileSet::empty(),
            entities: RefCell::new(HashMap::new()),
            visible_entities: hash_set![],
            cached_hero_props: EntityProps::default(),
            hitmap: vec![vec![false; WORLD_SIZE_COLUMNS]; WORLD_SIZE_ROWS],
            creative_mode: false,
            direction_based_on_current_keys: Direction::Unknown,
            is_any_arrow_key_down: false,
            has_confirmation_key_been_pressed: false,
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
        viewport: &Rect,
        keyboard: &KeyboardEventsProvider
    ) -> Vec<EngineStateUpdate> {
        self.total_elapsed_time += time_since_last_update;
        self.direction_based_on_current_keys = keyboard.direction_based_on_current_keys(self.cached_hero_props.direction);
        self.is_any_arrow_key_down = keyboard.is_any_arrow_key_down();
        self.has_confirmation_key_been_pressed = keyboard.has_confirmation_been_pressed;
        self.visible_entities = self.compute_visible_entities(viewport);
        self.hitmap = self.compute_hitmap();

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
        self.apply_state_updates(state_updates)
    } 

    pub fn apply_state_updates(&mut self, updates: Vec<WorldStateUpdate>) -> Vec<EngineStateUpdate> {
        updates.into_iter().filter_map(|u| self.apply_state_update(u)).collect()
    }

    fn apply_state_update(&mut self, update: WorldStateUpdate) -> Option<EngineStateUpdate> {
        match update {
            WorldStateUpdate::AddEntity(entity) => { self.add_entity(entity); },
            WorldStateUpdate::RemoveEntity(id) => self.remove_entity(&id),
            WorldStateUpdate::RemoveEntityAtCoordinates(row, col) => self.remove_entity_coords(row, col),
            WorldStateUpdate::CacheHeroProps(props) => { self.cached_hero_props = props; },
            WorldStateUpdate::BiomeTileChange(row, col, new_biome) => self.update_biome_tile(row, col, new_biome),
            WorldStateUpdate::ConstructionTileChange(row, col, new_construction) => self.update_construction_tile(row, col, new_construction),
            WorldStateUpdate::EngineUpdate(update) => return Some(update)
        };
        None
    }

    fn update_biome_tile(&mut self, row: usize, col: usize, new_biome: Biome) {
        self.biome_tiles.update_tile(row, col, new_biome)
    }

    fn update_construction_tile(&mut self, row: usize, col: usize, new_construction: Construction) {
        self.constructions_tiles.update_tile(row, col, new_construction)
    }

    pub fn visible_biome_tiles(&self, viewport: &Rect) -> Vec<&BiomeTile> {
        self.biome_tiles.visible_tiles(viewport)
    }

    pub fn visible_construction_tiles(&self, viewport: &Rect) -> Vec<&ConstructionTile> {
        self.constructions_tiles.visible_tiles(viewport)
    }

    pub fn find_teleporter_for_destination(&self, destination: &u32) -> Option<Rect> {
        self.entities.borrow().values()
            .filter_map(|e| e.as_ref().as_any().downcast_ref::<Teleporter>())
            .find(|t| t.destination == *destination)
            .map(|t| t.body().frame)
    }

    pub fn is_hero_around_and_on_collision_with(&self, target: &Rect) -> bool {
        let hero = self.cached_hero_props.hittable_frame;
        let hero_direction: Direction = self.cached_hero_props.direction;        
        if !self.has_confirmation_key_been_pressed { return false }  
        hero.is_around_and_pointed_at(target, &hero_direction)
    }

    pub fn find_entity_at_coords(&self, row: usize, col: usize) -> Option<u32> {
        for entity in self.entities.borrow().values() {
            if entity.body().frame.contains_or_touches_point(col as u32, row as u32) {
                return Some(entity.id())
            }
        }
        None
    }

    fn remove_entity_coords(&mut self, row: usize, col: usize) {
        while let Some(id) = self.find_entity_at_coords(row, col) {
            self.remove_entity(&id)
        }      
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

impl World {        
    pub fn update(&mut self, time_since_last_update: f32) -> Vec<EngineStateUpdate> {
        let keyboard = &NO_KEYBOARD_EVENTS;
        let viewport = self.bounds;
        self.update_rl(time_since_last_update, &viewport, keyboard)
    }
}