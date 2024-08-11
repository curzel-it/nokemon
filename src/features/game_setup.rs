use crate::{constants::HERO_ENTITY_ID, game_engine::{entity_body::EmbodiedEntity, game::Game}};

impl Game {    
    pub fn setup(&mut self) {
        self.load_map();
        self.add_creep_spawn_point();
        self.add_tower();
        self.add_hero();
        self.selected_entity_id = Some(HERO_ENTITY_ID);
    }

    fn add_creep_spawn_point(&mut self) -> u32 {
        let mut entity = self.entity_factory.build_creep_spawn_point();
        entity.center_in(&self.bounds);
        entity.body_mut().frame.x -= 50.0;
        entity.body_mut().frame.y -= 50.0;
        self.add_entity(Box::new(entity))
    }
    
    fn add_tower(&mut self) -> u32 {
        let mut entity = self.entity_factory.build_tower();
        entity.center_in(&self.bounds);
        entity.body_mut().frame.x -= 50.0;
        entity.body_mut().frame.y -= 50.0;
        self.add_entity(Box::new(entity))
    }

    fn add_hero(&mut self) -> u32 {
        let mut entity = self.entity_factory.build_hero();
        entity.center_in(&self.bounds);
        self.add_entity(Box::new(entity))
    }
}