use std::fmt::Debug;

use raylib::math::{Rectangle, Vector2};

use crate::constants::ANIMATIONS_FPS;
use crate::sprites::sprite::Sprite;
use crate::sprites::sprite_set::SpriteSet;

#[derive(Debug)]
pub struct Entity {
    pub id: u32,
    pub frame: Rectangle,
    pub direction: Vector2,
    pub speed: f32,
    pub species: String,
    pub sprite_set: SpriteSet,
    pub current_sprite: Sprite,
    pub is_enemy: bool,
    pub is_shooter: bool,
    pub time_between_shots: f32,
    pub time_to_next_shot: f32,
}

impl Entity {
    pub fn change_animation(&mut self, animation_name: &str) -> u32 {
        if self.current_sprite.animation_name != animation_name {
            self.current_sprite = self.sprite_set.sprite(&animation_name);
        }
        ((self.current_sprite.number_of_frames() as f32) / ANIMATIONS_FPS) as u32
    }

    fn current_sprite_frame(&self) -> String {
        self.current_sprite.current_frame().to_string()
    }
}