use std::fmt::Debug;

use raylib::math::{Rectangle, Vector2};

use crate::{constants::{ANIMATIONS_FPS, BASE_ENTITY_SPEED, SCALE}, species::species_model::Species, sprites::{sprite::Sprite, sprite_set::SpriteSet}};

pub trait EmbodiedEntity: Debug {
    fn id(&self) -> u32;    
    fn parent_id(&self) -> u32;
    fn species(&self) -> &Species;
    
    fn frame(&self) -> Rectangle;
    fn set_frame(&mut self, value: Rectangle);
    fn center_in(&mut self, value: &Rectangle);
    fn center_at(&mut self, value: &Vector2);
    fn place_at(&mut self, x: f32, y: f32);
    
    fn direction(&self) -> Vector2;
    fn set_direction(&mut self, value: Vector2);
    
    fn speed(&self) -> f32;
    fn set_speed(&mut self, speed: f32);
    fn reset_speed(&mut self);
    
    fn dp(&self) -> f32;
    
    fn hp(&self) -> f32;
    fn inc_hp(&mut self, value: f32);
        
    fn current_sprite_frame(&self) -> &str;
    fn current_animation(&self) -> &str;
    fn set_animation(&mut self, animation_name: &str) -> u32;    

    fn creation_time(&self) -> f32;
    fn set_creation_time(&mut self, value: f32);

    fn requires_collision_detection(&self) -> bool;
}

#[derive(Debug)]
pub struct EntityBody {
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
    pub creation_time: f32,
    pub requires_collision_detection: bool
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
            
    pub fn reset_speed(&mut self) {
        self.speed = BASE_ENTITY_SPEED * SCALE * self.species.speed;
    }
            
    pub fn set_animation(&mut self, animation_name: &str) -> u32 {
        if self.current_sprite.animation_name != animation_name {
            self.current_sprite = self.sprite_set.sprite(animation_name);
        }
        ((self.current_sprite.number_of_frames() as f32) / ANIMATIONS_FPS) as u32
    }
}