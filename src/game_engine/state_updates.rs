use crate::{dialogues::dialogues::Dialogue, entities::npc::NpcId, maps::{biome_tiles::Biome, constructions_tiles::Construction}, utils::vector::Vector2d};

use super::entity::{Entity, EntityProps};

pub enum WorldStateUpdate {
    AddEntity(Box<dyn Entity>),
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
    use crate::{constants::HERO_ENTITY_ID, entities::hero::Hero, game_engine::engine::GameEngine};

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