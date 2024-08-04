use std::fmt::Debug;

use raylib::math::{Rectangle, Vector2};

use crate::constants::ANIMATIONS_FPS;
use crate::species::species_model::Species;
use crate::sprites::sprite::Sprite;
use crate::sprites::sprite_set::SpriteSet;

#[derive(Debug)]
pub struct Entity {
    pub id: u32,
    pub parent_id: u32,
    pub frame: Rectangle,
    pub direction: Vector2,
    pub speed: f32,
    pub hp: f32,
    pub dp: f32,
    pub sprite_set: SpriteSet,
    pub current_sprite: Sprite,
    pub sprite_invalidated: bool,
    pub time_to_next_shot: f32,
    pub species: Species,
}

impl Entity {
    pub fn place_center_of(&mut self, bounds: Rectangle) {
        self.frame.x = bounds.x + (bounds.width - self.frame.width) / 2.0;
        self.frame.y = bounds.y + (bounds.height - self.frame.height) / 2.0;
    }

    pub fn change_direction(&mut self, new_direction: Vector2) {
        self.direction = new_direction;
        self.sprite_invalidated = true;
    }

    pub fn change_animation(&mut self, animation_name: &str) -> u32 {
        if self.current_sprite.animation_name != animation_name {
            self.current_sprite = self.sprite_set.sprite(&animation_name);
        }
        ((self.current_sprite.number_of_frames() as f32) / ANIMATIONS_FPS) as u32
    }
}