use crate::{constants::HERO_ENTITY_ID, game_engine::{entity_body::EmbodiedEntity, world::World}};

impl World {    
    pub fn setup(&mut self) {
        self.load_biome_tiles();
        self.load_constructions_tiles();
        // self.add_creep_spawn_point();
        // self.add_tower();
        self.add_hero();
        self.selected_entity_id = Some(HERO_ENTITY_ID);
    }

    fn add_creep_spawn_point(&mut self) -> u32 {
        let mut entity = self.entity_factory.build_creep_spawn_point();
        entity.center_in(&self.bounds);
        entity.body_mut().frame.x -= 100.0;
        entity.body_mut().frame.y -= 100.0;
        self.add_entity(Box::new(entity))
    }
    
    fn add_tower(&mut self) -> u32 {
        let mut entity = self.entity_factory.build_tower();
        entity.center_in(&self.bounds);
        entity.body_mut().frame.x -= 100.0;
        entity.body_mut().frame.y -= 100.0;
        self.add_entity(Box::new(entity))
    }

    fn add_hero(&mut self) -> u32 {
        let mut entity = self.entity_factory.build_hero();
        entity.center_in(&self.bounds);
        self.add_entity(Box::new(entity))
    }
}