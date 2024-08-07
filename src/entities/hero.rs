use raylib::math::Vector2;

use crate::{features::{animated_sprite::update_sprite, autoremove::remove_automatically, linear_movement_within_game_bounds::move_linearly_within_bounds, shooter::{Shooter}}, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, entity_factory::EntityFactory, game::Game, game_state_update::GameStateUpdate}, impl_animated_entity, impl_embodied_entity};

#[derive(Debug)]
pub struct Hero {
    body: EntityBody,
    time_to_next_shot: f32
}

impl Hero {
    pub fn new(body: EntityBody) -> Self {
        let time_to_next_shot = body.species.time_between_shots;

        Self { 
            body,
            time_to_next_shot
        }
    }
}

impl_embodied_entity!(Hero);
impl_animated_entity!(Hero);

impl Entity for Hero {
    fn update(&mut self, game: &Game, time_since_last_update: f32) -> Vec<GameStateUpdate> {
        let mut game_updates: Vec<GameStateUpdate> = vec![];
        move_linearly_within_bounds(self, &game.bounds, time_since_last_update);
        update_sprite(self, time_since_last_update);
        game_updates.append(&mut remove_automatically(self, game));
        game_updates
    }
}

impl EntityFactory {
    pub fn build_hero(&self) -> Hero {
        let mut hero = Hero::new(self.build("hero"));
        hero.set_direction(Vector2::new(1.0, 0.0));    
        hero
    }
}

impl Game {
    pub fn add_hero(&mut self) -> u32 {
        self.add_entity(Box::new(self.entity_factory.build_hero()))
    }
}