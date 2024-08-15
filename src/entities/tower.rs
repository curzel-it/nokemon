use raylib::math::{Rectangle, Vector2};

use crate::{constants::ASSETS_PATH, features::{animated_sprite::update_sprite, autoremove::remove_automatically, shooter::{shoot_stuff, Shooter}}, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, entity_factory::EntityFactory, world::World, world_state_update::WorldStateUpdate}, impl_embodied_entity};

use super::tower_dart::TowerDart;

#[derive(Debug)]
pub struct Tower {
    body: EntityBody,
    time_to_next_shot: f32,
    sprite_sheet_path: String,
}

impl Tower {
    pub fn new(body: EntityBody) -> Self {
        Self { 
            body,
            time_to_next_shot: 3.0,
            sprite_sheet_path: format!("{}/tower.png", ASSETS_PATH)
        }
    }
}

impl_embodied_entity!(Tower);

impl Shooter for Tower {
    fn time_to_next_shot(&self) -> f32 {
        self.time_to_next_shot
    }
    
    fn inc_time_to_next_shot(&mut self, delta: f32) {
        self.time_to_next_shot += delta;
    }
    
    fn reset_time_to_next_shot(&mut self) {
        self.time_to_next_shot = self.body().time_between_shots;
    }
    
    fn create_bullet(&self, entity_factory: &EntityFactory) -> Box<dyn Entity> {
        Box::new(TowerDart::new(self, entity_factory))
    }
}

impl Entity for Tower {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        let mut world_updates: Vec<WorldStateUpdate> = vec![];
        update_sprite(self, time_since_last_update);
        world_updates.append(&mut shoot_stuff(self, world, time_since_last_update));
        world_updates.append(&mut remove_automatically(self, world));
        world_updates
    }

    fn texture_source_rect(&self) -> Rectangle {
        Rectangle::new(
            0.0,
            0.0,
            self.body.frame.width,
            self.body.frame.height
        )
    }

    fn sprite_sheet_path(&self) -> &str {
        &self.sprite_sheet_path 
    }
}

impl EntityFactory {
    pub fn build_tower(&self) -> Tower {
        let mut body = self.build("tower");
        body.time_between_shots = 3.0;
        body.time_to_next_shot = 3.0;
        body.resize(26.0, 42.0);
        body.base_speed = 0.0;
        body.reset_speed();
        body.direction = Vector2::new(1.0, 0.0);    
        Tower::new(body)
    }
}