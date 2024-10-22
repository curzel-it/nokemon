use crate::{dialogues::models::Dialogue, entities::{npcs::NpcId, species::SpeciesId}, features::destination::Destination, maps::{biome_tiles::Biome, constructions_tiles::Construction}, menus::toasts::Toast, utils::vector::Vector2d};

use super::{entity::{Entity, EntityId, EntityProps}, locks::LockType};

#[derive(Debug, Clone)]
pub enum WorldStateUpdate {
    AddEntity(Box<Entity>),
    RemoveEntity(EntityId),
    RemoveEntityAtCoordinates(usize, usize),
    RenameEntity(EntityId, String),
    UseItem(SpeciesId),
    ToggleDemandAttention(EntityId),
    UpdateDestinationWorld(EntityId, u32),
    UpdateDestinationX(EntityId, i32),
    UpdateDestinationY(EntityId, i32),
    CacheHeroProps(Box<EntityProps>),
    ChangeLock(EntityId, LockType),
    BiomeTileChange(usize, usize, Biome),
    StopHeroMovement,
    ConstructionTileChange(usize, usize, Construction),
    EngineUpdate(EngineStateUpdate),
    HandleHit(EntityId, EntityId),
    SetPressurePlateState(LockType, bool)
}

#[derive(Debug, Clone)]
pub enum EngineStateUpdate {
    CenterCamera(i32, i32, Vector2d),
    Teleport(Destination),
    SaveGame,
    Exit,
    ShowEntityOptions(Box<Entity>),
    ShowInventoryOptions(Box<Entity>),
    ShowDialogue(NpcId, String, Dialogue), 
    AddToInventory(Box<Entity>),
    RemoveFromInventory(EntityId),
    Toast(Toast),
    Confirmation(String, String, Vec<WorldStateUpdate>),
    DisplayLongText(String),
    DeathScreen,
    ResumeGame,
}

#[cfg(test)]
mod tests {
    use crate::{entities::{known_species::SPECIES_HERO, species::make_entity_by_species}, game_engine::engine::GameEngine};

    #[test]
    fn entity_can_relay_world_state_updates() {
        let mut engine = GameEngine::new();
        engine.start();
        let hero = make_entity_by_species(SPECIES_HERO);
        let (hero_index, _) = engine.world.add_entity(hero);

        let mut entities = engine.world.entities.borrow_mut();
        let actual_tower = &mut entities[hero_index];
        let updates = actual_tower.update(&engine.world, 60.0);
        
        assert!(!updates.is_empty());
    }

    #[test]
    fn entity_can_relay_engine_state_updates() {
        let mut engine = GameEngine::new();
        engine.start();
        let hero = make_entity_by_species(SPECIES_HERO);
        engine.world.add_entity(hero);

        engine.world.update(1.0);
        let updates = engine.world.update(60.0);

        assert!(!updates.is_empty());
    }
}