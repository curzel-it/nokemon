use raylib::math::Vector2;

use crate::{features::{animated_sprite::update_sprite, autoremove::remove_automatically, shooter::{shoot_stuff, Shooter}}, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, entity_factory::EntityFactory, game::Game, game_state_update::GameStateUpdate}, impl_animated_entity, impl_embodied_entity};

#[derive(Debug)]
pub struct Tower {
    body: EntityBody,
    time_to_next_shot: f32
}

impl Tower {
    pub fn new(body: EntityBody) -> Self {
        let time_to_next_shot = body.species.time_between_shots;

        return Self { 
            body,
            time_to_next_shot
        };
    }
}

impl_embodied_entity!(Tower);
impl_animated_entity!(Tower);

impl Shooter for Tower {
    fn time_to_next_shot(&self) -> f32 {
        return self.time_to_next_shot;
    }

    fn inc_time_to_next_shot(&mut self, delta: f32) {
        self.time_to_next_shot += delta;
    }

    fn reset_time_to_next_shot(&mut self) {
        self.time_to_next_shot = self.species().time_between_shots;
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
        let mut tower = Tower::new(self.build("tower"));
        tower.set_direction(Vector2::new(1.0, 0.0));    
        return tower;
    }
}

impl Game {
    pub fn add_tower(&mut self) -> u32 {
        self.add_entity(Box::new(self.entity_factory.build_tower()))
    }
}