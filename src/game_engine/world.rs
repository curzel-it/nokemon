use std::{cell::RefCell, collections::{HashMap, HashSet}, fmt::{self, Debug}};

use common_macros::hash_set;
use raylib::math::{Rectangle, Vector2};

use crate::{constants::{HERO_ENTITY_ID, INITIAL_CAMERA_VIEWPORT, RECT_ORIGIN_SQUARE_100}, entities::background_tile::BackgroundTile};

use super::{collision_detection::compute_collisions, entity::Entity, entity_factory::EntityFactory, game_state_update::GameStateUpdate, keyboard_events_provider::{KeyboardEventsProvider, KeyboardState}, tile_set::TileSet, visible_entities::compute_visible_entities};

pub struct World {
    pub total_elapsed_time: f32,
    pub entity_factory: EntityFactory,
    pub bounds: Rectangle,
    pub camera_viewport: Rectangle,
    pub tiles: TileSet,
    pub entities: RefCell<HashMap<u32, Box<dyn Entity>>>,
    pub visible_entities: HashSet<u32>,
    pub selected_entity_id: Option<u32>,
    pub keyboard_state: KeyboardState,
    pub cached_hero_frame: Rectangle,
    pub cached_hero_position: Vector2,
    pub collisions: HashMap<u32, Vec<u32>>
}

impl World {
    pub fn new(entity_factory: EntityFactory) -> Self {
        Self {
            total_elapsed_time: 0.0,
            entity_factory,
            bounds: RECT_ORIGIN_SQUARE_100,
            camera_viewport: INITIAL_CAMERA_VIEWPORT,
            tiles: TileSet::empty(),
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
        keyboard_events: &dyn KeyboardEventsProvider
    ) {
        self.total_elapsed_time += time_since_last_update;
        self.keyboard_state = keyboard_events.keyboard_state();
        self.visible_entities = compute_visible_entities(self);
        self.collisions = compute_collisions(self);

        let mut state_updates: Vec<GameStateUpdate> = vec![];
        let mut entities = self.entities.borrow_mut();

        for id in &self.visible_entities {
            if let Some(entity) = entities.get_mut(id) {
                let mut updates = entity.update(self, time_since_last_update);
                state_updates.append(&mut updates);
            }
        }

        drop(entities);
        self.store_updated_hero_state();
        self.apply_state_updates(state_updates);

        self.camera_viewport = Rectangle::new(
            self.cached_hero_position.x - self.camera_viewport.width / 2.0,
            self.cached_hero_position.y - self.camera_viewport.height / 2.0,
            self.camera_viewport.width,
            self.camera_viewport.height
        );
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

    pub fn visible_tiles(&self) -> Vec<&BackgroundTile> {
        self.tiles.visible_tiles(&self.camera_viewport)
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
    use crate::game_engine::{entity_factory::EntityFactory, keyboard_events_provider::NoKeyboard};

    use super::World;

    impl World {
        pub fn test() -> World {
            World::new(EntityFactory::test())
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