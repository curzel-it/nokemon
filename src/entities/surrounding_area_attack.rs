use raylib::math::Vector2;

use crate::{features::{animated_sprite::update_sprite, autoremove::remove_automatically, linear_movement_within_game_bounds::move_linearly_within_bounds, position_seeker::set_direction_towards, shooter::Shooter}, game_engine::{entity::Entity, entity_body::{EmbodiedEntity, EntityBody}, entity_factory::{self, EntityFactory}, game::Game, game_state_update::GameStateUpdate}, impl_animated_entity, impl_embodied_entity};

#[derive(Debug)]
pub struct SurroundingAreaAttack {
    body: EntityBody
}

impl SurroundingAreaAttack {
    pub fn new(parent: &dyn Entity, entity_factory: &EntityFactory) -> Self {
        let mut body = entity_factory.build("baseattack");
        body.parent_id = parent.id();
        body.speed = 0.0;
        body.center_in(&parent.frame());
        
        Self {
            body
        }
    }
}

impl_embodied_entity!(SurroundingAreaAttack);
impl_animated_entity!(SurroundingAreaAttack);

impl Entity for SurroundingAreaAttack {
    fn update(&mut self, game: &Game, time_since_last_update: f32) -> Vec<GameStateUpdate> {
        self.center_in(&game.cached_hero_frame);
        update_sprite(self, time_since_last_update);
        vec![]
    }
}