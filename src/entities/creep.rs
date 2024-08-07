use raylib::math::Vector2;

use crate::{features::{animated_sprite::update_sprite, autoremove::remove_automatically, linear_movement_within_game_bounds::move_linearly_within_bounds, shooter::{Shooter}}, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, entity_factory::EntityFactory, game::Game, game_state_update::GameStateUpdate}, impl_animated_entity, impl_embodied_entity};

#[derive(Debug)]
pub struct Creep {
    body: EntityBody
}

impl Creep {
    pub fn new(body: EntityBody) -> Self {
        Self { 
            body
        }
    }
}

impl_embodied_entity!(Creep);
impl_animated_entity!(Creep);

impl Entity for Creep {
    fn update(&mut self, game: &Game, time_since_last_update: f32) -> Vec<GameStateUpdate> {
        let mut game_updates: Vec<GameStateUpdate> = vec![];
        move_linearly_within_bounds(self, &game.bounds, time_since_last_update);
        update_sprite(self, time_since_last_update);
        game_updates.append(&mut remove_automatically(self, game));
        game_updates
    }
}

impl EntityFactory {
    pub fn build_creep(&self) -> Creep {
        let mut creep = Creep::new(self.build("white"));
        creep.set_direction(Vector2::new(1.0, 0.0));    
        creep
    }
}

impl Game {
    pub fn add_creep(&mut self) -> u32 {
        self.add_entity(Box::new(self.entity_factory.build_creep()))
    }
}