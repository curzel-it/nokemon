use std::{cell::RefCell, collections::{HashMap, HashSet}, fmt::{self, Debug}};

use common_macros::hash_set;
use raylib::math::Rectangle;

use crate::{constants::{HERO_ENTITY_ID, RECT_ORIGIN_SQUARE_100, TILE_SIZE}, levels::utils::setup_level, maps::{biome_tiles::{Biome, BiomeTile}, constructions_tiles::ConstructionTile, tiles::TileSet}};

use super::{collision_detection::{compute_collisions, Collision}, entity::{Entity, EntityProps}, keyboard_events_provider::KeyboardState, state_updates::{EngineStateUpdate, WorldStateUpdate}, visible_entities::compute_visible_entities};

pub struct World {
    pub level_id: u32,
    pub total_elapsed_time: f32,
    pub bounds: Rectangle,
    pub biome_tiles: TileSet<BiomeTile>,
    pub constructions_tiles: TileSet<ConstructionTile>,
    pub entities: RefCell<HashMap<u32, Box<dyn Entity>>>,    
    pub visible_entities: HashSet<u32>,
    pub selected_entity_id: Option<u32>,
    pub keyboard_state: KeyboardState,
    pub cached_hero_props: EntityProps,
    pub collisions: HashMap<u32, Vec<Collision>>
}

impl World {
    pub fn new(level_id: u32) -> Self {
        Self {
            level_id,
            total_elapsed_time: 0.0,
            bounds: RECT_ORIGIN_SQUARE_100,
            biome_tiles: TileSet::empty(),
            constructions_tiles: TileSet::empty(),
            entities: RefCell::new(HashMap::new()),
            visible_entities: hash_set![],
            selected_entity_id: None,
            keyboard_state: KeyboardState::default(),
            cached_hero_props: EntityProps::default(),
            collisions: HashMap::new()
        }
    }

    pub fn setup(&mut self) {
        self.load_biome_tiles();
        self.load_construction_tiles();
        setup_level(self);
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
        keyboard_state: KeyboardState
    ) -> Vec<EngineStateUpdate> {
        self.total_elapsed_time += time_since_last_update;
        self.keyboard_state = keyboard_state;
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
        self.apply_state_updates(state_updates)
    } 

    pub fn apply_state_updates(&mut self, updates: Vec<WorldStateUpdate>) -> Vec<EngineStateUpdate> {
        updates.into_iter().filter_map(|u| self.apply_state_update(u)).collect()
    }

    fn apply_state_update(&mut self, update: WorldStateUpdate) -> Option<EngineStateUpdate> {
        match update {
            WorldStateUpdate::AddEntity(entity) => { self.add_entity(entity); },
            WorldStateUpdate::RemoveEntity(id) => self.remove_entity(&id),
            WorldStateUpdate::IncreaseHp(id, value) => self.increase_entity_hp(id, value),
            WorldStateUpdate::CacheHeroProps(props) => { self.cached_hero_props = props; },
            WorldStateUpdate::BiomeTileChange(row, col, new_biome) => self.update_biome_tile(row, col, new_biome),
            WorldStateUpdate::EngineUpdate(update) => return Some(update)
        };
        None
    }

    fn update_biome_tile(&mut self, row: usize, col: usize, new_biome: Biome) {
        self.biome_tiles.update_tile(row, col, new_biome)
    }

    fn increase_entity_hp(&mut self, id: u32, value: f32) {
        let mut entities = self.entities.borrow_mut();
        if let Some(entity) = entities.get_mut(&id) {
            entity.body_mut().hp += value;
        }
    }

    pub fn move_hero_one_tile_down(&mut self) {
        let mut entities = self.entities.borrow_mut();
        if let Some(hero) = entities.get_mut(&HERO_ENTITY_ID) {
            hero.body_mut().frame.y += TILE_SIZE;
            self.cached_hero_props = hero.props();
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

impl World {        
    pub fn update(&mut self, time_since_last_update: f32) -> Vec<EngineStateUpdate> {
        let keyboard_state = KeyboardState::nothing();
        let viewport = self.bounds;
        self.update_rl(time_since_last_update, &viewport, keyboard_state)
    }
}