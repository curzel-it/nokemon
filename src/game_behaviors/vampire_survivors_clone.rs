use raylib::math::Vector2;

use crate::{constants::HERO_ENTITY_ID, game_engine::{behaviors::GameBehavior, game::Game}};

pub struct VampireSurvivorsClone;

impl GameBehavior for VampireSurvivorsClone {
    fn update(&self, game: &mut Game, _: f32) {        
        let tower_id = game.add_entity_by_species("tower");
        let tower = game.entities.get_mut(&tower_id).unwrap();
        tower.change_direction(Vector2::new(1.0, 0.0));
        
        let mut hero = game.entity_factory.build("red");
        hero.id = HERO_ENTITY_ID;
        hero.change_direction(Vector2::new(1.0, 0.0));  
        hero.place_center_of(game.bounds);
        game.add_entity(hero);

        game.selected_entity_id = Some(HERO_ENTITY_ID);
    }
}

impl VampireSurvivorsClone {
    pub fn new() -> Self {
        Self {}
    }
}