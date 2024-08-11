use super::entity::Entity;

pub enum WorldStateUpdate {
    AddEntity(Box<dyn Entity>),
    RemoveEntity(u32),
    IncreaseHp(u32, f32)
}