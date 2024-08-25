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
    pub parent_id: Uuid,
    pub frame: Rect,  
    pub offset: Vector2d,
    pub direction: Vector2d,
    pub current_speed: f32,
    pub base_speed: f32,
    pub hp: f32,
    pub dp: f32,
    pub creation_time: f32,
    pub requires_collision_detection: bool,
    pub is_rigid: bool,
    pub z_index: i32,
    pub is_ally: bool,
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

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::{constants::{INFINITE_LIFESPAN, NO_PARENT}, utils::{rect::Rect, vector::Vector2d}};

    use super::EntityBody;

    impl EntityBody {
        pub fn test() -> Self {
            EntityBody {
                id: Uuid::new_v4(),
                parent_id: NO_PARENT,
                frame: Rect::new(0, 0, 50, 50),
                offset: Vector2d::zero(),
                direction: Vector2d::new(0.0, 0.0),
                current_speed: 1.0,
                base_speed: 1.0,
                hp: 100.0,
                dp: 0.0,
                creation_time: 0.0,
                requires_collision_detection: true,
                is_rigid: true,
                z_index: 0,
                is_ally: false,
                lifespan: INFINITE_LIFESPAN,
            }
        }
    }
}