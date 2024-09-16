use crate::{constants::{HERO_ENTITY_ID, SPRITE_SHEET_LARGE_HUMANOIDS}, entities::{known_species::SPECIES_PILL_RED, species::SpeciesId}, game_engine::world::World, utils::rect::Rect};

use super::animated_sprite::AnimatedSprite;

impl World {
    pub fn use_item(&mut self, species_id: SpeciesId) {
        match species_id {
            SPECIES_PILL_RED => self.use_red_pill(),
            _ => println!("Don't know how to use {}", species_id)
        }
    }

    fn use_red_pill(&mut self) {
        let mut entities = self.entities.borrow_mut();
        if let Some(hero) = entities.iter_mut().find(|e| e.id == HERO_ENTITY_ID) {    
            hero.sprite = AnimatedSprite::new(
                SPRITE_SHEET_LARGE_HUMANOIDS, 
                Rect::new(0, 0, 3, 3), 
                4
            );
            hero.update_sprite_for_current_direction();
            hero.speed_multiplier = 2.0;
            hero.frame.x -= 1;
            hero.frame.y -= 1;
            hero.frame.w = 3;
            hero.frame.h = 3;
        }
    }
}