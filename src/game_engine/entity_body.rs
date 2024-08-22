use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::{constants::TILE_SIZE, utils::{geometry_utils::Insets, rect::Rect, vector::Vector2d}};

use super::entity::EntityProps;

pub trait EmbodiedEntity: Debug {
    fn id(&self) -> u32;    
    fn parent_id(&self) -> u32;

    fn body(&self) -> &EntityBody;
    fn body_mut(&mut self) -> &mut EntityBody;
    fn collision_frame(&self) -> Rect;
    
    fn center_in(&mut self, value: &Rect);
    fn place_at(&mut self, x: f32, y: f32);
    fn snap_to_nearest_tile(&mut self);
    fn props(&self) -> EntityProps;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityBody {
    pub id: u32,
    pub parent_id: u32,
    pub frame: Rect,  
    pub collision_insets: Insets,
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
    
    pub fn center_at(&mut self, value: &Vector2d) {
        self.frame.center_at(value)
    }
    
    pub fn resize(&mut self, w: f32, h: f32) {
        self.frame.resize(w, h)
    }
            
    pub fn reset_speed(&mut self) {
        self.scale_speed(1.0);
    }
            
    pub fn scale_speed(&mut self, scalar: f32) {
        self.current_speed = self.base_speed * scalar;
    }

    pub fn collision_frame(&self) -> Rect {
        self.frame.inset(self.collision_insets)
    }

    pub fn snap_to_nearest_tile(&mut self) {
        self.frame.x = (self.frame.x / TILE_SIZE).round() * TILE_SIZE;
        self.frame.y = (self.frame.y / TILE_SIZE).round() * TILE_SIZE;
    }

    pub fn props(&self) -> EntityProps {
        EntityProps { 
            direction: self.direction, 
            frame: self.frame, 
            speed: self.current_speed 
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{constants::{INFINITE_LIFESPAN, NO_PARENT}, game_engine::entity_factory::get_next_entity_id, utils::{geometry_utils::Insets, rect::Rect, vector::Vector2d}};

    use super::EntityBody;

    impl EntityBody {
        pub fn test() -> Self {
            EntityBody {
                id: get_next_entity_id(),
                parent_id: NO_PARENT,
                frame: Rect::new(0.0, 0.0, 50.0, 50.0),
                collision_insets: Insets::zero(),
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