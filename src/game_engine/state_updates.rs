use crate::{dialogues::models::Dialogue, entities::{npcs::NpcId, species::{EntityType, SpeciesId}}, features::destination::Destination, maps::{biome_tiles::Biome, constructions_tiles::Construction}, utils::vector::Vector2d};

use super::{entity::{Entity, EntityId, EntityProps}, locks::LockType};

#[derive(Debug, Clone)]
pub enum WorldStateUpdate {
    AddEntity(Box<Entity>),
    RemoveEntity(EntityId),
    RemoveEntityAtCoordinates(usize, usize),
    RenameEntity(EntityId, String),
    UpdateDestinationWorld(EntityId, u32),
    UpdateDestinationX(EntityId, i32),
    UpdateDestinationY(EntityId, i32),
    CacheHeroProps(Box<EntityProps>),
    ChangeLock(EntityId, LockType),
    BiomeTileChange(usize, usize, Biome),
    StopHeroMovement,
    ConstructionTileChange(usize, usize, Construction),
    EngineUpdate(EngineStateUpdate),
}

#[derive(Debug, Clone)]
pub enum EngineStateUpdate {
    CenterCamera(i32, i32, Vector2d),
    Teleport(Destination),
    SaveGame,
    Exit,
    ShowEntityOptions(String, EntityId, SpeciesId, EntityType),
    ShowInventoryOptions(SpeciesId),
    ShowDialogue(NpcId, String, Dialogue), 
    ShowShop,
    AddToInventory(SpeciesId),
    RemoveFromInventory(SpeciesId),
    Toast(String),
    Confirmation(String, String, Vec<WorldStateUpdate>)
}

#[cfg(test)]
mod tests {
    use crate::{entities::{known_species::SPECIES_HERO, species::make_entity_by_species}, game_engine::engine::GameEngine};

    #[test]
    fn entity_can_relay_world_state_updates() {
        let mut engine = GameEngine::new();
        let mut world = engine.start_headless();
        let hero = make_entity_by_species(SPECIES_HERO);
        let (hero_index, _) = world.add_entity(hero);

        let mut entities = world.entities.borrow_mut();
        let actual_tower = &mut entities[hero_index];
        let updates = actual_tower.update(&world, 60.0);
        
        assert!(!updates.is_empty());
    }

    #[test]
    fn entity_can_relay_engine_state_updates() {
        let mut engine = GameEngine::new();
        let mut world = engine.start_headless();
        let hero = make_entity_by_species(SPECIES_HERO);
        world.add_entity(hero);

        world.update(1.0);
        let updates = world.update(60.0);

        assert!(!updates.is_empty());
    }
}