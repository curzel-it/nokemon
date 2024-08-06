use super::entity::Entity;

pub enum GameStateUpdate {
    AddEntity(Box<dyn Entity>),
    RemoveEntity(u32),
    SelectEntity(u32)
}