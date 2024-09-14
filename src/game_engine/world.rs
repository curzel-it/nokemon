use std::{cell::RefCell, collections::HashSet, fmt::{self, Debug}};

use common_macros::hash_set;
use crate::{constants::{HERO_ENTITY_ID, WORLD_SIZE_COLUMNS, WORLD_SIZE_ROWS}, entities::{known_species::SPECIES_HERO, species::EntityType}, features::{hitmap::Hitmap, weight_map::WeightMap}, maps::{biome_tiles::{Biome, BiomeTile}, constructions_tiles::{Construction, ConstructionTile}, tiles::TileSet}, utils::{directions::Direction, rect::Rect, vector::Vector2d}};

use super::{entity::{Entity, EntityProps}, keyboard_events_provider::{KeyboardEventsProvider, NO_KEYBOARD_EVENTS}, locks::LockType, state_updates::{EngineStateUpdate, WorldStateUpdate}};

pub struct World {
    pub id: u32,
    pub total_elapsed_time: f32,
    pub bounds: Rect,
    pub biome_tiles: TileSet<BiomeTile>,
    pub constructions_tiles: TileSet<ConstructionTile>,
    pub entities: RefCell<Vec<Entity>>,    
    pub visible_entities: HashSet<(usize, u32)>,
    pub cached_hero_props: EntityProps,
    pub hitmap: Hitmap,
    pub weight_map: WeightMap,
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
            bounds: Rect::square_from_origin(150),
            biome_tiles: TileSet::empty(),
            constructions_tiles: TileSet::empty(),
            entities: RefCell::new(vec![]),
            visible_entities: hash_set![],
            cached_hero_props: EntityProps::default(),
            hitmap: vec![vec![false; WORLD_SIZE_COLUMNS]; WORLD_SIZE_ROWS],
            weight_map: vec![vec![0; WORLD_SIZE_COLUMNS]; WORLD_SIZE_ROWS],
            creative_mode: false,
            direction_based_on_current_keys: Direction::Unknown,
            is_any_arrow_key_down: false,
            has_confirmation_key_been_pressed: false,
        }
    }

    pub fn add_entity(&mut self, entity: Entity) -> (usize, u32) {
        let id = entity.id;
        let mut entities = self.entities.borrow_mut();        
        entities.push(entity);
        let new_index = entities.len() - 1;
        entities[new_index].setup();
        (new_index, id)
    }

    fn remove_entity_by_id(&mut self, id: u32) {
        if id != HERO_ENTITY_ID {
            if let Some(index) = self.index_for_entity(id) {
                self.remove_entity_at_index(index);
            }
        }
    }

    fn remove_entity_at_index(&mut self, index: usize) {
        self.entities.borrow_mut().swap_remove(index);
    }

    fn index_for_entity(&self, id: u32) -> Option<usize> {
        self.entities.borrow().iter()
            .enumerate()
            .find(|(_, entity)|{ entity.id == id })
            .map(|(index, _)| index)
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

        let mut entities = self.entities.borrow_mut();

        let state_updates: Vec<WorldStateUpdate> = self.visible_entities.iter()
            .flat_map(|(index, _)| {
                if let Some(entity) = entities.get_mut(*index) {
                    entity.update(self, time_since_last_update)
                } else {
                    vec![]
                }                
            })
            .collect();

        self.biome_tiles.update(time_since_last_update);

        drop(entities);
        let updates = self.apply_state_updates(state_updates);
        self.visible_entities = self.compute_visible_entities(viewport);
        self.hitmap = self.compute_hitmap();
        self.weight_map = self.compute_weight_map();
        updates
    } 

    pub fn apply_state_updates(&mut self, updates: Vec<WorldStateUpdate>) -> Vec<EngineStateUpdate> {
        updates.into_iter().filter_map(|u| self.apply_state_update(u)).collect()
    }

    fn apply_state_update(&mut self, update: WorldStateUpdate) -> Option<EngineStateUpdate> {
        match update {
            WorldStateUpdate::AddEntity(entity) => { 
                self.add_entity(*entity); 
            },
            WorldStateUpdate::RemoveEntity(id) => {
                self.remove_entity_by_id(id)
            },
            WorldStateUpdate::RemoveEntityAtCoordinates(row, col) => {
                self.remove_entities_by_coords(row, col)
            },
            WorldStateUpdate::RenameEntity(id, new_name) => {
                self.rename_entity(id, new_name)
            },
            WorldStateUpdate::CacheHeroProps(props) => { 
                self.cached_hero_props = *props; 
            },
            WorldStateUpdate::ChangeLock(entity_id, lock_type) => {
                self.change_lock(entity_id, lock_type)
            },
            WorldStateUpdate::BiomeTileChange(row, col, new_biome) => {
                self.update_biome_tile(row, col, new_biome)
            },
            WorldStateUpdate::ConstructionTileChange(row, col, new_construction) => {
                self.update_construction_tile(row, col, new_construction)
            },
            WorldStateUpdate::StopHeroMovement => {
                self.stop_hero_movement()
            },
            WorldStateUpdate::EngineUpdate(update) => {
                return Some(update)
            },
            WorldStateUpdate::UpdateDestinationWorld(entity_id, world) => {
                self.change_destination_world(entity_id, world)
            },
            WorldStateUpdate::UpdateDestinationX(entity_id, x) => {
                self.change_destination_x(entity_id, x)
            },
            WorldStateUpdate::UpdateDestinationY(entity_id, y) => {
                self.change_destination_y(entity_id, y)
            },
        };
        None
    }

    fn stop_hero_movement(&mut self) {
        let mut entities = self.entities.borrow_mut();
        if let Some(entity) = entities.iter_mut().find(|e| e.id == HERO_ENTITY_ID) {            
            entity.offset = Vector2d::zero();
            entity.current_speed = 0.0;
        }
    }

    fn rename_entity(&mut self, id: u32, name: String) {
        let mut entities = self.entities.borrow_mut();
        if let Some(entity) = entities.iter_mut().find(|e| e.id == id) {
            entity.name = name;
        }
    }

    fn change_lock(&mut self, id: u32, lock_type: LockType) {
        let mut entities = self.entities.borrow_mut();
        if let Some(entity) = entities.iter_mut().find(|e| e.id == id) {
            entity.lock_type = lock_type;
        }
    }

    fn change_destination_world(&mut self, id: u32, world: u32) {
        let mut entities = self.entities.borrow_mut();
        if let Some(entity) = entities.iter_mut().find(|e| e.id == id) {
            if let Some(destination) = entity.destination.as_mut() {
                destination.world = world;
            }
        }
    }

    fn change_destination_x(&mut self, id: u32, x: i32) {
        let mut entities = self.entities.borrow_mut();
        if let Some(entity) = entities.iter_mut().find(|e| e.id == id) {
            if let Some(destination) = entity.destination.as_mut() {
                destination.x = x;
            }
        }
    }

    fn change_destination_y(&mut self, id: u32, y: i32) {
        let mut entities = self.entities.borrow_mut();
        if let Some(entity) = entities.iter_mut().find(|e| e.id == id) {
            if let Some(destination) = entity.destination.as_mut() {
                destination.y = y;
            }
        }
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
    
    pub fn find_teleporter_for_destination(&self, destination_world: u32) -> Option<Rect> {
        self.entities.borrow().iter()
            .find(|t| {
                if !matches!(t.entity_type, EntityType::Teleporter) {
                    return false
                } 
                if let Some(destination) = &t.destination {
                    return destination.world == destination_world;
                }
                false
            })
            .map(|t| t.frame)
    }


    pub fn is_hero_on_slippery_surface(&self) -> bool {
        let frame = self.cached_hero_props.hittable_frame;
        
        if self.biome_tiles.tiles.len() > frame.y as usize {
            let tile = self.biome_tiles.tiles[frame.y as usize][frame.x as usize].tile_type;
            matches!(tile, Biome::Ice)
        } else {
            false
        }
    }

    pub fn is_hero_around_and_on_collision_with(&self, target: &Rect) -> bool {
        let hero = self.cached_hero_props.hittable_frame;
        let hero_direction: Direction = self.cached_hero_props.direction;        
        if !self.has_confirmation_key_been_pressed { return false }  
        
        if hero.is_around_and_pointed_at(target, &hero_direction) {
            return true 
        }
        if self.hitmap[(hero.y as usize).saturating_sub(1)][hero.x as usize] && hero.x == target.x && hero.y.saturating_sub(3) == target.y && matches!(hero_direction, Direction::Up) {
            return true
        }
        false
    }

    pub fn find_non_hero_entity_at_coords(&self, row: usize, col: usize) -> Option<(usize, u32)> {
        self.entities.borrow().iter()
            .enumerate()
            .find(|(_, entity)| {
                entity.species_id != SPECIES_HERO && entity.frame.contains_or_touches_tile(col as i32, row as i32)
            })
            .map(|(index, e)| (index, e.id))
    }

    fn remove_entities_by_coords(&mut self, row: usize, col: usize) {
        while let Some((index, _)) = self.find_non_hero_entity_at_coords(row, col) {
            self.remove_entity_at_index(index)
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