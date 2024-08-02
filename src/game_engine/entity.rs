use std::fmt::Debug;

use raylib::math::{Rectangle, Vector2};

use crate::constants::ANIMATIONS_FPS;
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
    pub species: String,
    pub sprite_set: SpriteSet,
    pub current_sprite: Sprite,
    pub sprite_invalidated: bool,
    pub z_index: u32,
    pub is_enemy: bool,
    pub is_shooter: bool,
    pub is_bullet: bool,
    pub time_between_shots: f32,
    pub time_to_next_shot: f32,
}

impl Entity {
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