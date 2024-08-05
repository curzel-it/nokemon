use raylib::math::Vector2;

use crate::{constants::HERO_ENTITY_ID, game_engine::{behaviors::GameBehavior, game::Game}};

pub struct HeroBaseAttack;

impl GameBehavior for HeroBaseAttack {
    fn update(&self, game: &mut Game, _: f32) {        
        if game.keyboard_state.is_base_attack_pressed {
            let hero_frame = game.hero_frame();
            let mut new_entity = game.entity_factory.build("baseattack");
            new_entity.parent_id = HERO_ENTITY_ID;
            new_entity.direction = Vector2::zero();
            new_entity.change_animation("front");
            new_entity.place_center_of(hero_frame);
            game.add_entity(new_entity);
        }
    }
}

impl HeroBaseAttack {
    pub fn new() -> Self {
        Self {}
    }
}