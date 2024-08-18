use crate::maps::biome_tiles::Biome;

use super::entity::{Entity, EntityProps};

pub enum WorldStateUpdate {
    AddEntity(Box<dyn Entity>),
    RemoveEntity(u32),
    IncreaseHp(u32, f32),
    CacheHeroProps(EntityProps),
    BiomeTileChange(usize, usize, Biome),
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
    use crate::{entities::{hero::Hero, tower::Tower}, game_engine::{entity_body::EmbodiedEntity, game_engine::GameEngine}};

    #[test]
    fn entity_can_relay_world_state_updates() {
        let mut engine = GameEngine::new();
        let mut world = engine.start_headless();
        let tower = Tower::new();
        let tower_id = tower.id();
        world.add_entity(Box::new(tower));

        let mut entities = world.entities.borrow_mut();
        let actual_tower = entities.get_mut(&tower_id).unwrap();
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