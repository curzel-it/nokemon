use crate::maps::{biome_tiles::Biome, constructions_tiles::Construction};

use super::entity::{Entity, EntityProps};

pub enum WorldStateUpdate {
    AddEntity(Box<dyn Entity>),
    RemoveEntity(u32),
    IncreaseHp(u32, f32),
    CacheHeroProps(EntityProps),
    BiomeTileChange(usize, usize, Biome),
    ConstructionTileChange(usize, usize, Construction),
    EngineUpdate(EngineStateUpdate),
}

#[derive(Debug, Clone, Copy)]
pub enum EngineStateUpdate {
    CenterCamera(f32, f32),
    PushWorld(u32),
    PopWorld,
    ToggleWorld(u32),
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