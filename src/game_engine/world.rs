use std::{cell::RefCell, collections::{HashMap, HashSet}, fmt::{self, Debug}};

use common_macros::hash_set;
use crate::{constants::{WORLD_SIZE_COLUMNS, WORLD_SIZE_ROWS}, entities::teleporter::Teleporter, features::hitmap::Hitmap, maps::{biome_tiles::{Biome, BiomeTile}, constructions_tiles::{Construction, ConstructionTile}, tiles::TileSet}, utils::{rect::Rect, vector::Vector2d}};

use super::{entity::{Entity, EntityProps}, entity_body::EmbodiedEntity, keyboard_events_provider::KeyboardState, state_updates::{EngineStateUpdate, WorldStateUpdate}};

pub struct World {
    pub id: u32,
    pub total_elapsed_time: f32,
    pub bounds: Rect,
    pub biome_tiles: TileSet<BiomeTile>,
    pub constructions_tiles: TileSet<ConstructionTile>,
    pub entities: RefCell<HashMap<u32, Box<dyn Entity>>>,    
    pub visible_entities: HashSet<u32>,
    pub keyboard_state: KeyboardState,
    pub cached_hero_props: EntityProps,
    pub hitmap: Hitmap,
    pub creative_mode: bool,
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
            keyboard_state: KeyboardState::default(),
            cached_hero_props: EntityProps::default(),
            hitmap: vec![vec![false; WORLD_SIZE_COLUMNS]; WORLD_SIZE_ROWS],
            creative_mode: false,
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
        keyboard_state: KeyboardState
    ) -> Vec<EngineStateUpdate> {
        self.total_elapsed_time += time_since_last_update;
        self.keyboard_state = keyboard_state;
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
        let hero_direction: Vector2d = self.cached_hero_props.direction;        
        if !self.keyboard_state.has_confirmation_been_pressed { return false }
        hero.is_around_and_pointed_at(target, &hero_direction)
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
        let keyboard_state = KeyboardState::nothing();
        let viewport = self.bounds;
        self.update_rl(time_since_last_update, &viewport, keyboard_state)
    }
}