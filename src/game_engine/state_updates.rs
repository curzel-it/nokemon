use crate::{dialogues::models::Dialogue, entities::npcs::NpcId, maps::{biome_tiles::Biome, constructions_tiles::Construction}, utils::vector::Vector2d};

use super::entity::{Entity, EntityProps};

pub enum WorldStateUpdate {
    AddEntity(Entity),
    RemoveEntity(u32),
    RemoveEntityAtCoordinates(usize, usize),
    CacheHeroProps(EntityProps),
    BiomeTileChange(usize, usize, Biome),
    ConstructionTileChange(usize, usize, Construction),
    EngineUpdate(EngineStateUpdate),
}

#[derive(Debug, Clone)]
pub enum EngineStateUpdate {
    CenterCamera(i32, i32, Vector2d),
    SwitchWorld(u32),
    SaveGame,
    Exit,
    ShowEntityOptions(u32),
    ShowDialogue(NpcId, String, Dialogue), 
    ShowNpcOptions(NpcId, String, Option<Dialogue>)
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

        let updates = world.update(60.0);

        assert!(!updates.is_empty());
    }
}