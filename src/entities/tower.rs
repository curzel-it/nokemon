use raylib::math::Vector2;

use crate::{features::{animated_sprite::update_sprite, autoremove::remove_automatically, shooter::{shoot_stuff, Shooter}}, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, entity_factory::EntityFactory, game::Game, game_state_update::GameStateUpdate}, impl_animated_entity, impl_embodied_entity};

use super::tower_dart::TowerDart;

#[derive(Debug)]
pub struct Tower {
    body: EntityBody,
    time_to_next_shot: f32
}

impl Tower {
    pub fn new(body: EntityBody) -> Self {
        Self { 
            body,
            time_to_next_shot: 3.0
        }
    }
}

impl_embodied_entity!(Tower);
impl_animated_entity!(Tower);

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
    fn update(&mut self, game: &Game, time_since_last_update: f32) -> Vec<GameStateUpdate> {
        let mut game_updates: Vec<GameStateUpdate> = vec![];
        update_sprite(self, time_since_last_update);
        game_updates.append(&mut shoot_stuff(self, game, time_since_last_update));
        game_updates.append(&mut remove_automatically(self, game));
        game_updates
    }
}

impl EntityFactory {
    pub fn build_tower(&self) -> Tower {
        let mut body = self.build("tower");
        body.time_between_shots = 3.0;
        body.time_to_next_shot = 3.0;
        body.resize(50.0, 50.0);
        body.base_speed = 0.0;
        body.reset_speed();
        body.direction = Vector2::new(1.0, 0.0);    
        Tower::new(body)
    }
}