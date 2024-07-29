use crate::entities::{entity_capability::GameStateSnapshot, factory::EntityDescriptor};

pub trait GameCapability {
    fn update(&mut self, game: &GameStateSnapshot, time_since_last_update: f32) -> GameStateUpdate;
}

#[derive(Clone)]
pub struct GameStateUpdate {
    pub new_entities: Vec<EntityDescriptor>,
    pub entities_to_remove: Vec<u32>
}

impl GameStateUpdate {
    pub fn nothing() -> Self {
        Self {
            new_entities: vec![],
            entities_to_remove: vec![],
        }
    }

    pub fn new_entity(entity: EntityDescriptor) -> Self {
        Self {
            new_entities: vec![entity],
            entities_to_remove: vec![],
        }
    }
}