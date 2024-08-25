use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::{rect::Rect, vector::Vector2d};

pub trait EmbodiedEntity: Debug {
    fn id(&self) -> Uuid;    
    fn body(&self) -> &EntityBody;
    fn body_mut(&mut self) -> &mut EntityBody;    
    fn center_in(&mut self, value: &Rect);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityBody {
    pub id: Uuid,
    pub frame: Rect,  
    pub offset: Vector2d,
    pub direction: Vector2d,
    pub current_speed: f32,
    pub base_speed: f32,
    pub creation_time: f32,
    pub is_rigid: bool,
    pub z_index: i32,
    pub lifespan: f32,
}

impl EntityBody {            
    pub fn center_in(&mut self, value: &Rect) {
        self.frame.center_in(value)
    }
            
    pub fn reset_speed(&mut self) {
        self.scale_speed(1.0);
    }
            
    pub fn scale_speed(&mut self, scalar: f32) {
        self.current_speed = self.base_speed * scalar;
    }
}