use std::fmt::Debug;

use raylib::math::{Rectangle, Vector2};

use crate::{constants::{ANIMATIONS_FPS, SCALE}, sprites::{sprite::Sprite, sprite_set::SpriteSet}};

pub trait EmbodiedEntity: Debug {
    fn id(&self) -> u32;    
    fn parent_id(&self) -> u32;

    fn body(&self) -> &EntityBody;
    fn body_mut(&mut self) -> &mut EntityBody;
    
    fn center_in(&mut self, value: &Rectangle);
    fn place_at(&mut self, x: f32, y: f32);
}

#[derive(Debug)]
pub struct EntityBody {
    pub id: u32,
    pub parent_id: u32,
    pub frame: Rectangle,
    pub direction: Vector2,
    pub current_speed: f32,
    pub base_speed: f32,
    pub hp: f32,
    pub dp: f32,
    pub sprite_set: SpriteSet,
    pub current_sprite: Sprite,
    pub sprite_invalidated: bool,
    pub time_to_next_shot: f32,
    pub time_between_shots: f32,
    pub creation_time: f32,
    pub requires_collision_detection: bool,
    pub is_rigid: bool,
    pub z_index: i32,
    pub is_ally: bool,
    pub is_bullet: bool,
    pub lifespan: f32,
}

impl EntityBody {            
    pub fn center_in(&mut self, value: &Rectangle) {
        let center = Vector2 {
            x: value.x + value.width / 2.0,
            y: value.y + value.height / 2.0,
        };
        self.center_at(&center);
    }
    
    pub fn center_at(&mut self, value: &Vector2) {
        self.frame.x = value.x - self.frame.width / 2.0;
        self.frame.y = value.y - self.frame.height / 2.0;
    }
    
    pub fn resize(&mut self, w: f32, h: f32) {
        self.frame.width = SCALE * w;
        self.frame.height = SCALE * h;
    }
            
    pub fn reset_speed(&mut self) {
        self.scale_speed(1.0);
    }
            
    pub fn scale_speed(&mut self, scalar: f32) {
        self.current_speed = self.base_speed * scalar;
    }
            
    pub fn set_animation(&mut self, animation_name: &str) -> u32 {
        if self.current_sprite.animation_name != animation_name {
            self.current_sprite = self.sprite_set.sprite(animation_name);
        }
        ((self.current_sprite.number_of_frames() as f32) / ANIMATIONS_FPS) as u32
    }

    pub fn current_sprite_frame(&self) -> &str {
        self.current_sprite.current_frame()
    }
}