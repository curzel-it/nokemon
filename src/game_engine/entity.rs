use std::cmp::Ordering;
use std::fmt::Debug;

use raylib::math::{Rectangle, Vector2};

use crate::constants::{ANIMATIONS_FPS, BASE_ENTITY_SPEED, SCALE};
use crate::impl_entity;
use crate::species::species_model::Species;
use crate::sprites::sprite::Sprite;
use crate::sprites::sprite_set::SpriteSet;

pub trait Entity: Debug {
    fn id(&self) -> u32;    
    fn parent_id(&self) -> u32;
    fn species(&self) -> &Species;
    
    fn frame(&self) -> Rectangle;
    fn set_frame(&mut self, value: Rectangle);
    fn center_in(&mut self, value: &Rectangle);
    fn center_at(&mut self, value: &Vector2);
    
    fn direction(&self) -> Vector2;
    fn set_direction(&mut self, value: Vector2);
    
    fn speed(&self) -> f32;
    fn set_speed(&mut self, speed: f32);
    fn reset_speed(&mut self);
    
    fn hp(&self) -> f32;
    fn inc_hp(&mut self, value: f32);
        
    fn current_sprite_frame(&self) -> &str;
    fn current_animation(&self) -> &str;
    fn set_animation(&mut self, animation_name: &str) -> u32;    

    fn creation_time(&self) -> f32;
    fn set_creation_time(&mut self, value: f32);
}

impl PartialEq for dyn Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for dyn Entity {}

impl PartialOrd for dyn Entity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for dyn Entity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frame().y
            .partial_cmp(&other.frame().y)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.species().z_index.cmp(&other.species().z_index))
            .then_with(|| self.creation_time().partial_cmp(&other.creation_time()).unwrap_or(Ordering::Equal))
    }
}

#[derive(Debug)]
pub struct BaseEntity {
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
}

impl BaseEntity {            
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
            self.current_sprite = self.sprite_set.sprite(&animation_name);
        }
        ((self.current_sprite.number_of_frames() as f32) / ANIMATIONS_FPS) as u32
    }
}

#[derive(Debug)]
pub struct SimpleEntity {
    pub base: BaseEntity
}

impl_entity!(SimpleEntity);