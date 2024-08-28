use uuid::Uuid;

use crate::{maps::{biome_tiles::Biome, constructions_tiles::Construction}, utils::vector::Vector2d};

use super::entity::{Entity, EntityProps};

pub enum WorldStateUpdate {
    AddEntity(Box<dyn Entity>),
    RemoveEntity(Uuid),
    CacheHeroProps(EntityProps),
    BiomeTileChange(usize, usize, Biome),
    ConstructionTileChange(usize, usize, Construction),
    EngineUpdate(EngineStateUpdate),
}

#[derive(Debug, Clone, Copy)]
pub enum EngineStateUpdate {
    CenterCamera(u32, u32, Vector2d),
    SwitchWorld(Uuid),
    SaveGame,
    Exit,
    BuildingInteraction(Uuid),
    NpcInteraction(Uuid),
    EntityInteraction(Uuid)
}

#[cfg(test)]
mod tests {
    use crate::{constants::HERO_ENTITY_ID, entities::hero::Hero, game_engine::game_engine::GameEngine};

    #[test]
    fn entity_can_relay_world_state_updates() {
        let mut engine = GameEngine::new();
        let mut world = engine.start_headless();
        let hero = Hero::new();
        world.add_entity(Box::new(hero));

        let mut entities = world.entities.borrow_mut();
        let actual_tower = entities.get_mut(&HERO_ENTITY_ID).unwrap();
        let updates = actual_tower.update(&world, 60.0);
        
        assert!(!updates.is_empty());
    }

    #[test]
    fn entity_can_relay_engine_state_updates() {
        let mut engine = GameEngine::new();
        let mut world = engine.start_headless();
        let hero = Hero::new();
        world.add_entity(Box::new(hero));

        let updates = world.update(60.0);

        assert!(!updates.is_empty());
    }
}